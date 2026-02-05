use abstract_app::objects::namespace::Namespace;
use abstract_app::std::{app, registry};
use abstract_client::{AbstractClient, Application};
use axone_gov::{
    gateway::logic::{
        set_query_service_ask_handler, Answer, QueryServiceAskMockGuard, QueryServiceAskResponse,
        Result as LogicResult, Substitution,
    },
    msg::{AxoneGovExecuteMsgFns, AxoneGovInstantiateMsg, AxoneGovQueryMsgFns, DecisionResponse},
    AxoneGovInterface, AXONE_GOV_ID, AXONE_NAMESPACE,
};
use cosmwasm_std::{to_hex, Binary, Checksum};
use cw_orch::{anyhow, prelude::*};
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const MOCK_SENDER: &str = "mock1pgm8hyk0pvphmlvfjc8wsvk4daluz5tgrw6pu5mfpemk74uxnx9qwrtv4f";
const MOCK_BLOCK_HEIGHT: u64 = 12345;
const MOCK_BLOCK_TIME: u64 = 1571797419;
const MOCK_TX_INDEX: u32 = 0;
const ABSTRACT_EVENT_TYPE: &str = "wasm-abstract";

fn record_decision_context() -> String {
    format!(
        "'gov:module': module{{id: '{AXONE_GOV_ID}', version: '{PKG_VERSION}'}}, \
'cw:tx': tx{{message: msg{{sender: {MOCK_SENDER}, funds: []}}, \
block: block{{height: {MOCK_BLOCK_HEIGHT}, time_seconds: {MOCK_BLOCK_TIME}, tx_index: {MOCK_TX_INDEX}}}}}"
    )
}

fn record_decision_case(case_body: &str) -> String {
    format!("case{{{case_body}, {}}}", record_decision_context())
}

#[derive(Clone)]
struct LogicAskExpectations(Rc<RefCell<VecDeque<(String, QueryServiceAskResponse)>>>);

impl Drop for LogicAskExpectations {
    fn drop(&mut self) {
        if std::thread::panicking() {
            return;
        }
        let remaining = self.0.borrow().len();
        assert_eq!(
            remaining, 0,
            "not all Logic::ask expectations were consumed (remaining={remaining})"
        );
    }
}

struct LogicAskScenario {
    expected: Rc<RefCell<VecDeque<(String, QueryServiceAskResponse)>>>,
    assertions: Rc<RefCell<Vec<(usize, Box<dyn Fn(&str)>)>>>,
}

impl LogicAskScenario {
    fn new() -> Self {
        Self {
            expected: Rc::new(RefCell::new(VecDeque::new())),
            assertions: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn then(self, program: impl Into<String>, response: QueryServiceAskResponse) -> Self {
        self.expected
            .borrow_mut()
            .push_back((program.into(), response));
        self
    }

    fn assert_query<F>(self, query_index: usize, assertion: F) -> Self
    where
        F: Fn(&str) + 'static,
    {
        self.assertions
            .borrow_mut()
            .push((query_index, Box::new(assertion)));
        self
    }

    fn install(self) -> (QueryServiceAskMockGuard, LogicAskExpectations) {
        let expected = self.expected.clone();
        let assertions = self.assertions.clone();
        let queries = Rc::new(RefCell::new(Vec::new()));
        let queries_clone = queries.clone();

        let hook: QueryServiceAskMockGuard = set_query_service_ask_handler(move |request| {
            let query_index = queries_clone.borrow().len();
            queries_clone.borrow_mut().push(request.query.clone());

            for (idx, assertion) in assertions.borrow().iter() {
                if *idx == query_index {
                    assertion(&request.query);
                }
            }

            let mut q = expected.borrow_mut();
            let (program, response) = q
                .pop_front()
                .expect("unexpected Logic::ask call (no expectation left)");
            assert_eq!(request.program, program, "unexpected Logic::ask program");
            Ok(response)
        });

        (hook, LogicAskExpectations(self.expected))
    }
}

struct TestEnv<Env: CwEnv> {
    _hook: QueryServiceAskMockGuard,
    _expectations: LogicAskExpectations,
    app: Application<Env, AxoneGovInterface<Env>>,
}

impl TestEnv<MockBech32> {
    fn setup(
        constitution: Binary,
        hook: QueryServiceAskMockGuard,
        expectations: LogicAskExpectations,
    ) -> anyhow::Result<Self> {
        let chain = MockBech32::new("mock");
        let client = AbstractClient::builder(chain.clone()).build()?;
        let publisher = client
            .account_builder()
            .namespace(Namespace::new(AXONE_NAMESPACE)?)
            .build()?
            .publisher()?;
        publisher.publish_app::<AxoneGovInterface<MockBech32>>()?;

        let app = publisher
            .account()
            .install_app::<AxoneGovInterface<MockBech32>>(
                &AxoneGovInstantiateMsg { constitution },
                &[],
            )?;

        Ok(Self {
            _hook: hook,
            _expectations: expectations,
            app,
        })
    }
}

fn ask_ok() -> QueryServiceAskResponse {
    let result = LogicResult {
        error: None,
        substitutions: Vec::new(),
    };
    let answer = Answer {
        has_more: false,
        variables: Vec::new(),
        results: vec![result],
    };

    QueryServiceAskResponse {
        height: 0,
        gas_used: 0,
        answer: Some(answer),
        user_output: None,
    }
}

fn ask_error(msg: impl Into<String>) -> QueryServiceAskResponse {
    let result = LogicResult {
        error: Some(msg.into()),
        substitutions: Vec::new(),
    };
    let answer = Answer {
        has_more: false,
        variables: Vec::new(),
        results: vec![result],
    };

    QueryServiceAskResponse {
        height: 0,
        gas_used: 0,
        answer: Some(answer),
        user_output: None,
    }
}

fn ask_with_substitutions(substitutions: Vec<Substitution>) -> QueryServiceAskResponse {
    let result = LogicResult {
        error: None,
        substitutions,
    };
    let answer = Answer {
        has_more: false,
        variables: Vec::new(),
        results: vec![result],
    };

    QueryServiceAskResponse {
        height: 0,
        gas_used: 0,
        answer: Some(answer),
        user_output: None,
    }
}

fn ask_decision_without_motivation(verdict: impl Into<String>) -> QueryServiceAskResponse {
    let substitutions = vec![Substitution {
        variable: "Verdict".to_string(),
        expression: verdict.into(),
    }];
    ask_with_substitutions(substitutions)
}

fn ask_decision_with_motivation(
    verdict: impl Into<String>,
    motivation: impl Into<String>,
) -> QueryServiceAskResponse {
    let substitutions = vec![
        Substitution {
            variable: "Verdict".to_string(),
            expression: verdict.into(),
        },
        Substitution {
            variable: "Motivation".to_string(),
            expression: motivation.into(),
        },
    ];
    ask_with_substitutions(substitutions)
}

fn ask_establish_permitted() -> QueryServiceAskResponse {
    ask_decision_with_motivation("'gov:permitted'", "ok")
}

fn ask_no_answer() -> QueryServiceAskResponse {
    QueryServiceAskResponse {
        height: 0,
        gas_used: 0,
        answer: None,
        user_output: None,
    }
}

fn ask_empty_results() -> QueryServiceAskResponse {
    let answer = Answer {
        has_more: false,
        variables: Vec::new(),
        results: Vec::new(),
    };

    QueryServiceAskResponse {
        height: 0,
        gas_used: 0,
        answer: Some(answer),
        user_output: None,
    }
}

fn atom_literal(value: &str) -> String {
    let mut chars = value.chars();
    match chars.next() {
        Some(c)
            if c.is_ascii_lowercase() && chars.all(|c| c.is_ascii_alphanumeric() || c == '_') =>
        {
            value.to_string()
        }
        _ => format!("'{}'", value.replace('\'', "''")),
    }
}

// --- Test helpers ---
fn assert_hash_matches(payload: &[u8], got: &Binary) {
    let expected = Checksum::generate(payload);
    assert_eq!(*got, Binary::from(expected.as_slice()));
}

fn assert_decision_response(
    record: &DecisionResponse,
    expected_id: u64,
    expected_constitution: &Binary,
    expected_case: &str,
    expected_verdict: &str,
    expected_motivation: Option<&str>,
) {
    assert_eq!(record.decision_id, expected_id);
    assert_eq!(record.constitution_revision, 0);
    assert_hash_matches(expected_constitution.as_slice(), &record.constitution_hash);

    assert_eq!(record.case, expected_case);
    assert_hash_matches(expected_case.as_bytes(), &record.case_hash);

    assert_eq!(record.verdict, expected_verdict);
    assert_hash_matches(expected_verdict.as_bytes(), &record.verdict_hash);

    assert_eq!(record.motivation, expected_motivation.map(str::to_string));
    match (&record.motivation_hash, expected_motivation) {
        (None, None) => {}
        (Some(got), Some(motivation)) => assert_hash_matches(motivation.as_bytes(), got),
        (got, expected) => panic!("unexpected motivation_hash: got={got:?} expected={expected:?}"),
    }

    assert_eq!(record.author, MOCK_SENDER);
    assert_eq!(record.block_height, MOCK_BLOCK_HEIGHT);
    assert_eq!(record.block_time_seconds, MOCK_BLOCK_TIME);
}

#[test]
fn instantiate_succeeds_with_valid_constitution() {
    let constitution = Binary::from(b"valid.".to_vec());
    let (hook, expectations) = LogicAskScenario::new()
        .then("valid.", ask_ok())
        .then("valid.", ask_establish_permitted())
        .install();
    let env = TestEnv::setup(constitution.clone(), hook, expectations)
        .expect("Failed to setup test environment");

    let constitution_got = env
        .app
        .constitution()
        .expect("Failed to query constitution");
    assert_eq!(constitution_got.constitution, constitution);

    let status = env
        .app
        .constitution_status()
        .expect("Failed to query constitution status");
    let expected_hash = Checksum::generate(constitution.as_slice());
    assert_eq!(status.constitution_revision, 0);
    assert_eq!(
        status.constitution_hash,
        Binary::from(expected_hash.as_slice())
    );
}

#[test]
fn instantiate_rejects_invalid_constitution() {
    let (hook, expectations) = LogicAskScenario::new()
        .then("invalid(", ask_error("parse error"))
        .install();

    let err = match TestEnv::setup(Binary::from(b"invalid(".to_vec()), hook, expectations) {
        Ok(_) => panic!("Expected invalid constitution error"),
        Err(err) => err,
    };
    let msg = err.to_string();
    let chain = err
        .chain()
        .map(|cause| cause.to_string())
        .collect::<Vec<_>>();
    let has_invalid = msg.contains("constitution is invalid")
        || chain
            .iter()
            .any(|cause| cause.contains("constitution is invalid"));
    assert!(
        has_invalid,
        "expected constitution is invalid, got: {msg}; chain: {chain:?}"
    );
}

#[test]
fn instantiate_rejects_constitution_missing_required_predicates() {
    let (hook, expectations) = LogicAskScenario::new()
        .then("valid.", ask_empty_results())
        .install();

    let err = match TestEnv::setup(Binary::from(b"valid.".to_vec()), hook, expectations) {
        Ok(_) => panic!("Expected missing required predicates error"),
        Err(err) => err,
    };
    let msg = err.to_string();
    let chain = err
        .chain()
        .map(|cause| cause.to_string())
        .collect::<Vec<_>>();
    let has_missing_predicates = msg.contains("missing required predicates")
        || chain
            .iter()
            .any(|cause| cause.contains("missing required predicates"));
    assert!(
        has_missing_predicates,
        "expected missing required predicates, got: {msg}; chain: {chain:?}"
    );
}

#[test]
fn instantiate_succeeds_without_registered_gov() {
    let constitution = Binary::from(b"valid.".to_vec());
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let (_hook, _expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(program, ask_establish_permitted())
        .install();

    let chain = MockBech32::new("mock");
    let contract = AxoneGovInterface::new("axone-gov-direct-missing", chain.clone());
    contract.upload().expect("Failed to upload axone-gov");

    let init_msg = app::InstantiateMsg {
        base: app::BaseInstantiateMsg {
            account: registry::Account::new(chain.addr_make("not-an-account")),
        },
        module: AxoneGovInstantiateMsg {
            constitution: constitution.clone(),
        },
    };

    contract
        .instantiate(&init_msg, None, &[])
        .expect("Expected instantiation to succeed");

    let stored = contract
        .constitution()
        .expect("Failed to query constitution");
    assert_eq!(stored.constitution, constitution);
}

#[test]
fn instantiate_fails_when_gov_already_registered() {
    let constitution = Binary::from(b"valid.".to_vec());
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let (_hook, _expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(program, ask_establish_permitted())
        .install();

    let chain = MockBech32::new("mock");
    let client = AbstractClient::builder(chain.clone())
        .build()
        .expect("Failed to build Abstract client");
    let publisher = client
        .account_builder()
        .namespace(Namespace::new(AXONE_NAMESPACE).expect("Invalid namespace"))
        .build()
        .expect("Failed to build account")
        .publisher()
        .expect("Failed to build publisher");
    publisher
        .publish_app::<AxoneGovInterface<MockBech32>>()
        .expect("Failed to publish axone-gov");

    let account = publisher.account();
    account
        .install_app::<AxoneGovInterface<MockBech32>>(
            &AxoneGovInstantiateMsg {
                constitution: constitution.clone(),
            },
            &[],
        )
        .expect("Failed to install axone-gov");

    let contract = AxoneGovInterface::new("axone-gov-direct", chain);
    contract.upload().expect("Failed to upload axone-gov");

    let account_addr = account.address().expect("Failed to get account address");
    let init_msg = app::InstantiateMsg {
        base: app::BaseInstantiateMsg {
            account: registry::Account::new(account_addr),
        },
        module: AxoneGovInstantiateMsg { constitution },
    };

    let err = contract
        .instantiate(&init_msg, None, &[])
        .expect_err("Expected duplicate governance error");

    let msg = format!("{err:?}");
    assert!(
        msg.contains("module already installed:"),
        "expected module already installed error, got: {msg}"
    );
    assert!(
        msg.contains(AXONE_GOV_ID),
        "expected module id '{AXONE_GOV_ID}' in error, got: {msg}"
    );
}

#[test]
fn decide_succeeds_without_motivation() {
    let constitution = Binary::from(
        b"decide(case{action:transfer}, allowed).
decide(case{action:withdraw}, denied)."
            .to_vec(),
    );
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();

    let test_cases = vec![
        ("case{action:transfer}", "nominal case"),
        (" case{action:transfer} ", "leading and trailing spaces"),
        ("case{'foo-bar':baz_qux}", "complex atom in key"),
        ("case{action:(transfer, withdraw)}", "tuple as value"),
        ("case{actions:[transfer, withdraw]}", "array as value"),
    ];

    for (case, description) in test_cases {
        let (hook, expectations) = LogicAskScenario::new()
            .then(program, ask_ok())
            .then(program, ask_establish_permitted())
            .then(program, ask_decision_without_motivation("allowed"))
            .install();
        let env =
            TestEnv::setup(constitution.clone(), hook, expectations).expect("Failed to setup test");

        let response = AxoneGovQueryMsgFns::decide(&env.app, case.to_string(), Some(false))
            .unwrap_or_else(|_| panic!("Failed to query decide for case: {}", description));

        assert_eq!(
            response.verdict, "allowed",
            "Unexpected verdict for case: {}",
            description
        );
        assert_eq!(
            response.motivation, None,
            "Unexpected motivation for case: {}",
            description
        );
    }
}

#[test]
fn decide_succeeds_with_motivation() {
    let constitution = Binary::from(
        b"decide(case{action:transfer}, allowed, 'User is authorized').
decide(case{action:withdraw}, denied, 'Insufficient funds')."
            .to_vec(),
    );
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let (hook, expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(program, ask_establish_permitted())
        .then(
            program,
            ask_decision_with_motivation("allowed", "'User is authorized'"),
        )
        .install();
    let env =
        TestEnv::setup(constitution.clone(), hook, expectations).expect("Failed to setup test");

    let response = env
        .app
        .decide("case{action:transfer}".to_string(), Some(true))
        .expect("Failed to query decide");

    assert_eq!(response.verdict, "allowed");
    assert_eq!(
        response.motivation,
        Some("'User is authorized'".to_string())
    );
}

#[test]
fn decide_fails_with_invalid_case() {
    let constitution = Binary::from(b"decide(_, verdict).".to_vec());
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let (hook, expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(program, ask_establish_permitted())
        .install();
    let env =
        TestEnv::setup(constitution.clone(), hook, expectations).expect("Failed to setup test");

    let invalid_cases = vec![
        ("not_a_dict", "case must be a Prolog dict"),
        (
            "'not_a_term",
            "syntax error at offset 0: unterminated quoted atom",
        ),
        (
            "case{action:transfer, user:User}",
            "case must be ground (no variables)",
        ),
    ];

    for (case, expected_msg) in invalid_cases {
        let err = env
            .app
            .decide(case.to_string(), Some(false))
            .expect_err("Expected invalid case error");

        let msg = format!("{err:?}");
        assert!(
            msg.contains(&format!("invalid case: {}", expected_msg)),
            "got: {msg}"
        );
    }
}

#[test]
fn decide_fails_with_no_answer() {
    let constitution = Binary::from(b"decide(_, verdict).".to_vec());
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let (hook, expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(program, ask_establish_permitted())
        .then(program, ask_no_answer())
        .install();
    let env =
        TestEnv::setup(constitution.clone(), hook, expectations).expect("Failed to setup test");

    let err = env
        .app
        .decide("case{action:test}".to_string(), Some(false))
        .expect_err("Expected prolog engine no answer error");

    let msg = format!("{err:?}");
    assert!(
        msg.contains("prolog engine returned no answer"),
        "expected prolog engine no answer, got: {msg}"
    );
}

#[test]
fn decide_fails_with_no_results() {
    let constitution = Binary::from(b"decide(_, verdict).".to_vec());
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let (hook, expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(program, ask_establish_permitted())
        .then(program, ask_empty_results())
        .install();
    let env =
        TestEnv::setup(constitution.clone(), hook, expectations).expect("Failed to setup test");

    let err = env
        .app
        .decide("case{action:test}".to_string(), Some(false))
        .expect_err("Expected decision no result error");

    let msg = format!("{err:?}");
    assert!(
        msg.contains("decision returned no result"),
        "expected decision no result, got: {msg}"
    );
}

#[test]
fn decide_fails_with_prolog_error() {
    let constitution = Binary::from(b"decide(Case, Verdict) :- fail.".to_vec());
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let (hook, expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(program, ask_establish_permitted())
        .then(program, ask_error("predicate failed"))
        .install();
    let env =
        TestEnv::setup(constitution.clone(), hook, expectations).expect("Failed to setup test");

    let err = AxoneGovQueryMsgFns::decide(&env.app, "case{action:test}".to_string(), Some(false))
        .expect_err("Expected decision failed error");

    let msg = format!("{err:?}");
    assert!(
        msg.contains("decision failed"),
        "expected decision failed, got: {msg}"
    );
}

#[test]
fn decide_fails_with_missing_verdict() {
    let constitution = Binary::from(b"decide(_, verdict).".to_vec());
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let (hook, expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(program, ask_establish_permitted())
        .then(
            program,
            ask_with_substitutions(vec![Substitution {
                variable: "WrongVar".to_string(),
                expression: "allowed".to_string(),
            }]),
        )
        .install();
    let env =
        TestEnv::setup(constitution.clone(), hook, expectations).expect("Failed to setup test");

    let err = env
        .app
        .decide("case{action:test}".to_string(), Some(false))
        .expect_err("Expected missing verdict error");

    let msg = format!("{err:?}");
    assert!(
        msg.contains("decision verdict missing"),
        "expected decision verdict missing, got: {msg}"
    );
}

#[test]
fn decide_fails_with_missing_motivation() {
    let constitution = Binary::from(b"decide(_, verdict, motivation).".to_vec());
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let (hook, expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(program, ask_establish_permitted())
        .then(program, ask_decision_without_motivation("allowed"))
        .install();
    let env =
        TestEnv::setup(constitution.clone(), hook, expectations).expect("Failed to setup test");

    let err = env
        .app
        .decide("case{action:test}".to_string(), Some(true))
        .expect_err("Expected missing motivation error");

    let msg = format!("{err:?}");
    assert!(
        msg.contains("decision motivation missing"),
        "expected decision motivation missing, got: {msg}"
    );
}

#[test]
fn execute_record_decision_succeeds_without_motivation_and_emits_events() {
    let constitution = Binary::from(
        b"decide(case{action:transfer}, allowed).
decide(case{action:withdraw}, denied)."
            .to_vec(),
    );
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let case = "case{action:transfer}";
    let case_term = record_decision_case("action: transfer");
    let verdict = "allowed";
    let expected_query = format!("decide({case_term}, Verdict).");

    let (hook, expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(program, ask_establish_permitted())
        .then(program, ask_decision_without_motivation(verdict))
        .assert_query(2, move |query| {
            assert_eq!(query, expected_query, "unexpected decide query");
        })
        .install();
    let env =
        TestEnv::setup(constitution.clone(), hook, expectations).expect("Failed to setup test");

    let response = env
        .app
        .record_decision(case.to_string(), None)
        .expect("Failed to execute decide");

    assert_eq!(
        response
            .event_attr_value(ABSTRACT_EVENT_TYPE, "action")
            .expect("Missing action attribute"),
        "record_decision"
    );
    assert_eq!(
        response
            .event_attr_value(ABSTRACT_EVENT_TYPE, "decision_id")
            .expect("Missing decision_id attribute"),
        "1"
    );
    assert_eq!(
        response
            .event_attr_value(ABSTRACT_EVENT_TYPE, "constitution_revision")
            .expect("Missing constitution_revision attribute"),
        "0"
    );

    let expected_constitution_hash = to_hex(Checksum::generate(constitution.as_slice()).as_ref());
    let expected_case_hash = to_hex(Checksum::generate(case_term.as_bytes()).as_ref());
    let expected_verdict_hash = to_hex(Checksum::generate(verdict.as_bytes()).as_ref());

    assert_eq!(
        response
            .event_attr_value(ABSTRACT_EVENT_TYPE, "constitution_hash")
            .expect("Missing constitution_hash attribute"),
        expected_constitution_hash
    );
    assert_eq!(
        response
            .event_attr_value(ABSTRACT_EVENT_TYPE, "case_hash")
            .expect("Missing case_hash attribute"),
        expected_case_hash
    );
    assert_eq!(
        response
            .event_attr_value(ABSTRACT_EVENT_TYPE, "verdict_hash")
            .expect("Missing verdict_hash attribute"),
        expected_verdict_hash
    );
    assert_eq!(
        response
            .event_attr_value(ABSTRACT_EVENT_TYPE, "verdict")
            .expect("Missing verdict attribute"),
        verdict
    );
    assert!(
        response
            .event_attr_values(ABSTRACT_EVENT_TYPE, "motivation_hash")
            .is_empty(),
        "motivation_hash should not be emitted without motivation"
    );
}

#[test]
fn execute_record_decision_increments_decision_id_and_supports_motivation() {
    let constitution = Binary::from(
        b"decide(case{action:transfer}, allowed).
decide(case{action:withdraw}, denied)."
            .to_vec(),
    );
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let case_one = "case{action:transfer}";
    let case_two = "case{action:withdraw}";
    let case_one_term = record_decision_case("action: transfer");
    let case_two_term = record_decision_case("action: withdraw");
    let expected_query_one = format!("decide({case_one_term}, Verdict, Motivation).");
    let expected_query_two = format!("decide({case_two_term}, Verdict).");

    let (hook, expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(program, ask_establish_permitted())
        .then(program, ask_decision_with_motivation("allowed", "'reason'"))
        .then(program, ask_decision_without_motivation("denied"))
        .assert_query(2, move |query| {
            assert_eq!(query, expected_query_one, "unexpected decide/3 query");
        })
        .assert_query(3, move |query| {
            assert_eq!(query, expected_query_two, "unexpected decide/2 query");
        })
        .install();
    let env =
        TestEnv::setup(constitution.clone(), hook, expectations).expect("Failed to setup test");

    let response_one = env
        .app
        .record_decision(case_one.to_string(), Some(true))
        .expect("Failed to execute decide with motivation");
    let response_two = env
        .app
        .record_decision(case_two.to_string(), None)
        .expect("Failed to execute decide without motivation");

    assert_eq!(
        response_one
            .event_attr_value(ABSTRACT_EVENT_TYPE, "decision_id")
            .expect("Missing decision_id attribute"),
        "1"
    );
    assert_eq!(
        response_two
            .event_attr_value(ABSTRACT_EVENT_TYPE, "decision_id")
            .expect("Missing decision_id attribute"),
        "2"
    );
}

#[test]
fn execute_decide_fails_with_missing_motivation() {
    let constitution = Binary::from(b"decide(_, verdict, motivation).".to_vec());
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let (hook, expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(program, ask_establish_permitted())
        .then(program, ask_decision_without_motivation("allowed"))
        .install();
    let env =
        TestEnv::setup(constitution.clone(), hook, expectations).expect("Failed to setup test");

    let err = env
        .app
        .record_decision("case{action:test}".to_string(), Some(true))
        .expect_err("Expected missing motivation error");

    let msg = format!("{err:?}");
    assert!(
        msg.contains("decision motivation missing"),
        "expected decision motivation missing, got: {msg}"
    );
}

#[test]
fn query_decision_returns_recorded_decision_without_motivation() {
    let constitution = Binary::from(b"decide(case{action:transfer}, allowed).".to_vec());
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let case_input = "case{action:transfer}";
    let case_term = record_decision_case("action: transfer");
    let verdict = "allowed";

    let (hook, expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(program, ask_establish_permitted())
        .then(program, ask_decision_without_motivation(verdict))
        .install();
    let env =
        TestEnv::setup(constitution.clone(), hook, expectations).expect("Failed to setup test");

    env.app
        .record_decision(case_input.to_string(), None)
        .expect("Failed to record decision");

    let response = AxoneGovQueryMsgFns::decision(&env.app, 1).expect("Failed to query decision");

    assert_decision_response(&response, 1, &constitution, &case_term, verdict, None);
}

#[test]
fn query_decision_returns_recorded_decision_with_motivation() {
    let constitution = Binary::from(b"decide(case{action:transfer}, allowed, ok).".to_vec());
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let case_input = "case{action:transfer}";
    let case_term = record_decision_case("action: transfer");
    let verdict = "allowed";
    let motivation = "ok";

    let (hook, expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(program, ask_establish_permitted())
        .then(program, ask_decision_with_motivation(verdict, motivation))
        .install();
    let env =
        TestEnv::setup(constitution.clone(), hook, expectations).expect("Failed to setup test");

    env.app
        .record_decision(case_input.to_string(), Some(true))
        .expect("Failed to record decision");

    let response = AxoneGovQueryMsgFns::decision(&env.app, 1).expect("Failed to query decision");

    assert_decision_response(
        &response,
        1,
        &constitution,
        &case_term,
        verdict,
        Some(motivation),
    );
}

#[test]
fn query_decision_fails_when_missing() {
    let constitution = Binary::from(b"decide(case{action:transfer}, allowed).".to_vec());
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let (hook, expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(program, ask_establish_permitted())
        .install();
    let env =
        TestEnv::setup(constitution, hook, expectations).expect("Failed to setup test environment");

    let err =
        AxoneGovQueryMsgFns::decision(&env.app, 1).expect_err("Expected missing decision error");

    let msg = format!("{err:?}");
    assert!(
        msg.contains("not found"),
        "expected decision not found error, got: {msg}"
    );
}

#[test]
fn query_decisions_returns_empty_when_no_records() {
    let constitution = Binary::from(b"decide(case{action:transfer}, allowed).".to_vec());
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let (hook, expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(program, ask_establish_permitted())
        .install();
    let env =
        TestEnv::setup(constitution, hook, expectations).expect("Failed to setup test environment");

    let response =
        AxoneGovQueryMsgFns::decisions(&env.app, None, None).expect("Failed to query decisions");

    assert!(response.decisions.is_empty());
}

#[test]
fn query_decisions_returns_records_in_order_and_supports_pagination() {
    let constitution = Binary::from(
        b"decide(case{action:transfer}, allowed).
decide(case{action:withdraw}, denied, reason).
decide(case{action:mint}, allowed)."
            .to_vec(),
    );
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let (hook, expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(program, ask_establish_permitted())
        .then(program, ask_decision_without_motivation("allowed"))
        .then(program, ask_decision_with_motivation("denied", "reason"))
        .then(program, ask_decision_without_motivation("allowed"))
        .install();
    let env =
        TestEnv::setup(constitution.clone(), hook, expectations).expect("Failed to setup test");

    env.app
        .record_decision("case{action:transfer}".to_string(), None)
        .expect("Failed to record decision 1");
    env.app
        .record_decision("case{action:withdraw}".to_string(), Some(true))
        .expect("Failed to record decision 2");
    env.app
        .record_decision("case{action:mint}".to_string(), None)
        .expect("Failed to record decision 3");

    let response =
        AxoneGovQueryMsgFns::decisions(&env.app, None, None).expect("Failed to query decisions");
    assert_eq!(response.decisions.len(), 3);

    assert_decision_response(
        &response.decisions[0],
        1,
        &constitution,
        &record_decision_case("action: transfer"),
        "allowed",
        None,
    );
    assert_decision_response(
        &response.decisions[1],
        2,
        &constitution,
        &record_decision_case("action: withdraw"),
        "denied",
        Some("reason"),
    );
    assert_decision_response(
        &response.decisions[2],
        3,
        &constitution,
        &record_decision_case("action: mint"),
        "allowed",
        None,
    );

    let page = AxoneGovQueryMsgFns::decisions(&env.app, Some(1), Some(1))
        .expect("Failed to query decisions page");
    assert_eq!(page.decisions.len(), 1);
    assert_eq!(page.decisions[0].decision_id, 2);
}

#[test]
fn revise_constitution_succeeds_with_permitted_verdict() {
    let constitution = Binary::from(b"decide(_, 'gov:permitted', 'Revision allowed').".to_vec());
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let new_constitution = Binary::from(b"decide(_, allowed).".to_vec());

    let new_constitution_program = std::str::from_utf8(new_constitution.as_slice()).unwrap();
    let (hook, expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(program, ask_establish_permitted())
        .then(new_constitution_program, ask_ok())
        .then(
            program,
            ask_decision_with_motivation("'gov:permitted'", "'Revision allowed'"),
        )
        .then(new_constitution_program, ask_establish_permitted())
        .install();
    let env = TestEnv::setup(constitution.clone(), hook, expectations)
        .expect("Failed to setup test environment");

    env.app
        .revise_constitution(new_constitution.clone(), None)
        .expect("Failed to revise constitution");

    let constitution_got = env
        .app
        .constitution()
        .expect("Failed to query constitution");
    assert_eq!(constitution_got.constitution, new_constitution);

    let status = env
        .app
        .constitution_status()
        .expect("Failed to query constitution status");
    assert_eq!(status.constitution_revision, 1);
    let expected_hash = Checksum::generate(new_constitution.as_slice());
    assert_eq!(
        status.constitution_hash,
        Binary::from(expected_hash.as_slice())
    );
}

#[test]
fn revise_constitution_succeeds_with_custom_case() {
    let constitution = Binary::from(b"decide(_, 'gov:permitted', 'Revision allowed').".to_vec());
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let new_constitution = Binary::from(b"decide(_, allowed).".to_vec());
    let new_constitution_program = std::str::from_utf8(new_constitution.as_slice()).unwrap();

    let (hook, expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(program, ask_establish_permitted())
        .then(new_constitution_program, ask_ok())
        .then(
            program,
            ask_decision_with_motivation("'gov:permitted'", "'Revision allowed'"),
        )
        .then(new_constitution_program, ask_establish_permitted())
        .install();
    let env = TestEnv::setup(constitution.clone(), hook, expectations)
        .expect("Failed to setup test environment");

    let custom_case = "case{proposer:'alice'}";
    env.app
        .revise_constitution(new_constitution.clone(), Some(custom_case.to_string()))
        .expect("Failed to revise constitution");

    let constitution_got = env
        .app
        .constitution()
        .expect("Failed to query constitution");
    assert_eq!(constitution_got.constitution, new_constitution);

    let status = env
        .app
        .constitution_status()
        .expect("Failed to query constitution status");
    assert_eq!(status.constitution_revision, 1);
}

#[test]
fn revise_constitution_fails_with_invalid_new_constitution() {
    let constitution = Binary::from(b"decide(_, 'gov:permitted', 'Revision allowed').".to_vec());
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let invalid_constitution = Binary::from(b"invalid(".to_vec());

    let (hook, expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(program, ask_establish_permitted())
        .then("invalid(", ask_error("parse error"))
        .install();
    let env = TestEnv::setup(constitution.clone(), hook, expectations)
        .expect("Failed to setup test environment");

    let err = env
        .app
        .revise_constitution(invalid_constitution, None)
        .expect_err("Expected invalid constitution error");

    let msg = format!("{err:?}");
    assert!(
        msg.contains("constitution is invalid"),
        "expected constitution is invalid, got: {msg}"
    );
}

#[test]
fn revise_constitution_fails_with_missing_required_predicates() {
    let constitution = Binary::from(b"decide(_, 'gov:permitted', 'Revision allowed').".to_vec());
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let new_constitution = Binary::from(b"valid.".to_vec());

    let (hook, expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(program, ask_establish_permitted())
        .then("valid.", ask_empty_results())
        .install();
    let env = TestEnv::setup(constitution.clone(), hook, expectations)
        .expect("Failed to setup test environment");

    let err = env
        .app
        .revise_constitution(new_constitution, None)
        .expect_err("Expected missing required predicates error");

    let msg = format!("{err:?}");
    assert!(
        msg.contains("missing required predicates"),
        "expected missing required predicates, got: {msg}"
    );
}

#[test]
fn revise_constitution_fails_with_denied_verdict() {
    let constitution = Binary::from(b"decide(_, denied, 'Unauthorized').".to_vec());
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let new_constitution = Binary::from(b"decide(_, allowed).".to_vec());
    let new_constitution_program = std::str::from_utf8(new_constitution.as_slice()).unwrap();

    let (hook, expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(program, ask_establish_permitted())
        .then(new_constitution_program, ask_ok())
        .then(
            program,
            ask_decision_with_motivation("denied", "'Unauthorized'"),
        )
        .install();
    let env = TestEnv::setup(constitution.clone(), hook, expectations)
        .expect("Failed to setup test environment");

    let err = env
        .app
        .revise_constitution(new_constitution, None)
        .expect_err("Expected decision refused error");

    let msg = format!("{err:?}");
    assert!(
        msg.contains("decision refused"),
        "expected decision refused, got: {msg}"
    );
    assert!(
        msg.contains("denied"),
        "expected verdict 'denied' in error, got: {msg}"
    );
}

#[test]
fn revise_constitution_fails_with_invalid_case() {
    let constitution = Binary::from(b"decide(_, 'gov:permitted', 'Revision allowed').".to_vec());
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let new_constitution = Binary::from(b"decide(_, allowed).".to_vec());
    let new_constitution_program = std::str::from_utf8(new_constitution.as_slice()).unwrap();

    let (hook, expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(program, ask_establish_permitted())
        .then(new_constitution_program, ask_ok())
        .then(new_constitution_program, ask_ok())
        .then(new_constitution_program, ask_ok())
        .install();
    let env = TestEnv::setup(constitution.clone(), hook, expectations)
        .expect("Failed to setup test environment");

    let invalid_cases = vec![
        ("not_a_dict", "case must be a Prolog dict"),
        (
            "'not_a_term",
            "syntax error at offset 0: unterminated quoted atom",
        ),
        ("case{proposer:User}", "case must be ground (no variables)"),
    ];

    for (case, expected_msg) in invalid_cases {
        let err = env
            .app
            .revise_constitution(new_constitution.clone(), Some(case.to_string()))
            .expect_err("Expected invalid case error");

        let msg = format!("{err:?}");
        assert!(
            msg.contains(&format!("invalid case: {}", expected_msg)),
            "got: {msg}"
        );
    }
}

#[test]
fn revise_constitution_fails_with_no_answer() {
    let constitution = Binary::from(b"decide(_, 'gov:permitted', 'Revision allowed').".to_vec());
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let new_constitution = Binary::from(b"decide(_, allowed).".to_vec());
    let new_constitution_program = std::str::from_utf8(new_constitution.as_slice()).unwrap();

    let (hook, expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(program, ask_establish_permitted())
        .then(new_constitution_program, ask_ok())
        .then(program, ask_no_answer())
        .install();
    let env = TestEnv::setup(constitution.clone(), hook, expectations)
        .expect("Failed to setup test environment");

    let err = env
        .app
        .revise_constitution(new_constitution, None)
        .expect_err("Expected prolog engine no answer error");

    let msg = format!("{err:?}");
    assert!(
        msg.contains("prolog engine returned no answer"),
        "expected prolog engine no answer, got: {msg}"
    );
}

#[test]
fn revise_constitution_fails_with_no_results() {
    let constitution = Binary::from(b"decide(_, 'gov:permitted', 'Revision allowed').".to_vec());
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let new_constitution = Binary::from(b"decide(_, allowed).".to_vec());
    let new_constitution_program = std::str::from_utf8(new_constitution.as_slice()).unwrap();

    let (hook, expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(program, ask_establish_permitted())
        .then(new_constitution_program, ask_ok())
        .then(program, ask_empty_results())
        .install();
    let env = TestEnv::setup(constitution.clone(), hook, expectations)
        .expect("Failed to setup test environment");

    let err = env
        .app
        .revise_constitution(new_constitution, None)
        .expect_err("Expected decision no result error");

    let msg = format!("{err:?}");
    assert!(
        msg.contains("decision returned no result"),
        "expected decision no result, got: {msg}"
    );
}

#[test]
fn revise_constitution_fails_with_prolog_error() {
    let constitution = Binary::from(b"decide(Case, Verdict, Motivation) :- fail.".to_vec());
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let new_constitution = Binary::from(b"decide(_, allowed).".to_vec());
    let new_constitution_program = std::str::from_utf8(new_constitution.as_slice()).unwrap();

    let (hook, expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(program, ask_establish_permitted())
        .then(new_constitution_program, ask_ok())
        .then(program, ask_error("predicate failed"))
        .install();
    let env = TestEnv::setup(constitution.clone(), hook, expectations)
        .expect("Failed to setup test environment");

    let err = env
        .app
        .revise_constitution(new_constitution, None)
        .expect_err("Expected decision failed error");

    let msg = format!("{err:?}");
    assert!(
        msg.contains("decision failed"),
        "expected decision failed, got: {msg}"
    );
}

#[test]
fn revise_constitution_fails_with_missing_verdict() {
    let constitution = Binary::from(b"decide(_, 'gov:permitted', 'Revision allowed').".to_vec());
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let new_constitution = Binary::from(b"decide(_, allowed).".to_vec());
    let new_constitution_program = std::str::from_utf8(new_constitution.as_slice()).unwrap();

    let (hook, expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(program, ask_establish_permitted())
        .then(new_constitution_program, ask_ok())
        .then(
            program,
            ask_with_substitutions(vec![Substitution {
                variable: "WrongVar".to_string(),
                expression: "'gov:permitted'".to_string(),
            }]),
        )
        .install();
    let env = TestEnv::setup(constitution.clone(), hook, expectations)
        .expect("Failed to setup test environment");

    let err = env
        .app
        .revise_constitution(new_constitution, None)
        .expect_err("Expected missing verdict error");

    let msg = format!("{err:?}");
    assert!(
        msg.contains("decision verdict missing"),
        "expected decision verdict missing, got: {msg}"
    );
}

#[test]
fn revise_constitution_fails_with_missing_motivation() {
    let constitution = Binary::from(b"decide(_, 'gov:permitted', 'Revision allowed').".to_vec());
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let new_constitution = Binary::from(b"decide(_, allowed).".to_vec());
    let new_constitution_program = std::str::from_utf8(new_constitution.as_slice()).unwrap();

    let (hook, expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(program, ask_establish_permitted())
        .then(new_constitution_program, ask_ok())
        .then(program, ask_decision_without_motivation("'gov:permitted'"))
        .install();
    let env = TestEnv::setup(constitution.clone(), hook, expectations)
        .expect("Failed to setup test environment");

    let err = env
        .app
        .revise_constitution(new_constitution, None)
        .expect_err("Expected missing motivation error");

    let msg = format!("{err:?}");
    assert!(
        msg.contains("decision motivation missing"),
        "expected decision motivation missing, got: {msg}"
    );
}

#[test]
fn revise_constitution_fails_with_invalid_verdict_term() {
    let constitution = Binary::from(b"decide(_, 'gov:permitted', 'Revision allowed').".to_vec());
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let new_constitution = Binary::from(b"decide(_, allowed).".to_vec());
    let new_constitution_program = std::str::from_utf8(new_constitution.as_slice()).unwrap();

    let (hook, expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(program, ask_establish_permitted())
        .then(new_constitution_program, ask_ok())
        .then(
            program,
            ask_decision_with_motivation("invalid(term(", "'Some motivation'"),
        )
        .install();
    let env = TestEnv::setup(constitution.clone(), hook, expectations)
        .expect("Failed to setup test environment");

    let err = env
        .app
        .revise_constitution(new_constitution, None)
        .expect_err("Expected invalid verdict term error");

    let msg = format!("{err:?}");
    assert!(
        msg.contains("invalid verdict term at offset"),
        "expected invalid verdict term error, got: {msg}"
    );
}

#[test]
fn revise_constitution_injects_complete_context_as_per_specification() {
    let constitution = Binary::from(b"decide(_, 'gov:permitted', ok).".to_vec());
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let new_constitution = Binary::from(b"decide(_, 'gov:permitted', ok).".to_vec());
    let new_constitution_program = std::str::from_utf8(new_constitution.as_slice()).unwrap();

    let proposed_hash = atom_literal(&to_hex(
        Checksum::generate(new_constitution.as_slice()).as_ref(),
    ));
    let current_hash = atom_literal(&to_hex(
        Checksum::generate(constitution.as_slice()).as_ref(),
    ));
    let expected_revise_query = format!(
        "decide(ctx{{intent: 'gov:revise_constitution', 'gov:proposed_constitution_sha256': {}, 'gov:current_constitution_sha256': {}, 'gov:current_constitution_revision': 0, 'gov:module': module{{id: 'axone:axone-gov', version: '{}'}}, 'cw:tx': tx{{message: msg{{sender: mock1pgm8hyk0pvphmlvfjc8wsvk4daluz5tgrw6pu5mfpemk74uxnx9qwrtv4f, funds: []}}, block: block{{height: 12345, time_seconds: 1571797419, tx_index: 0}}}}}}, Verdict, Motivation).",
        proposed_hash, current_hash, PKG_VERSION
    );
    let expected_establish_query = format!(
        "decide(ctx{{intent: 'gov:establish', 'gov:proposed_constitution_sha256': {}, 'gov:current_constitution_sha256': {}, 'gov:current_constitution_revision': 0, 'gov:module': module{{id: 'axone:axone-gov', version: '{}'}}, 'cw:tx': tx{{message: msg{{sender: mock1pgm8hyk0pvphmlvfjc8wsvk4daluz5tgrw6pu5mfpemk74uxnx9qwrtv4f, funds: []}}, block: block{{height: 12345, time_seconds: 1571797419, tx_index: 0}}}}}}, Verdict, Motivation).",
        proposed_hash, current_hash, PKG_VERSION
    );

    let (hook, expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(program, ask_establish_permitted())
        .then(new_constitution_program, ask_ok())
        .then(
            program,
            ask_decision_with_motivation("'gov:permitted'", "ok"),
        )
        .then(new_constitution_program, ask_establish_permitted())
        .assert_query(3, {
            let expected_revise_query = expected_revise_query.clone();
            move |query| {
                assert_eq!(
                    query, expected_revise_query,
                    "Query must exactly match the expected structure.\nExpected:\n{}\nGot:\n{}",
                    expected_revise_query, query
                );
            }
        })
        .assert_query(4, {
            let expected_establish_query = expected_establish_query.clone();
            move |query| {
                assert_eq!(
                    query, expected_establish_query,
                    "Query must exactly match the expected structure.\nExpected:\n{}\nGot:\n{}",
                    expected_establish_query, query
                );
            }
        })
        .install();

    let env = TestEnv::setup(constitution.clone(), hook, expectations)
        .expect("Failed to setup test environment");

    env.app
        .revise_constitution(new_constitution.clone(), None)
        .expect("Revision should succeed");
}

#[test]
fn revise_constitution_hash_value_matches_proposed_constitution() {
    let constitution = Binary::from(b"decide(_, 'gov:permitted', ok).".to_vec());
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let new_constitution = Binary::from(b"decide(_, allowed).".to_vec());
    let new_constitution_program = std::str::from_utf8(new_constitution.as_slice()).unwrap();

    let proposed_hash = atom_literal(&to_hex(
        Checksum::generate(new_constitution.as_slice()).as_ref(),
    ));
    let current_hash = atom_literal(&to_hex(
        Checksum::generate(constitution.as_slice()).as_ref(),
    ));
    let expected_query = format!(
        "decide(ctx{{intent: 'gov:revise_constitution', 'gov:proposed_constitution_sha256': {}, 'gov:current_constitution_sha256': {}, 'gov:current_constitution_revision': 0, 'gov:module': module{{id: 'axone:axone-gov', version: '{}'}}, 'cw:tx': tx{{message: msg{{sender: mock1pgm8hyk0pvphmlvfjc8wsvk4daluz5tgrw6pu5mfpemk74uxnx9qwrtv4f, funds: []}}, block: block{{height: 12345, time_seconds: 1571797419, tx_index: 0}}}}}}, Verdict, Motivation).",
        proposed_hash, current_hash, PKG_VERSION
    );

    let (hook, expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(program, ask_establish_permitted())
        .then(new_constitution_program, ask_ok())
        .then(
            program,
            ask_decision_with_motivation("'gov:permitted'", "ok"),
        )
        .then(new_constitution_program, ask_establish_permitted())
        .assert_query(3, move |query| {
            assert_eq!(
                query, expected_query,
                "Query must exactly match with correct hash.\nExpected:\n{}\nGot:\n{}",
                expected_query, query
            );
        })
        .install();

    let env = TestEnv::setup(constitution.clone(), hook, expectations)
        .expect("Failed to setup test environment");

    env.app
        .revise_constitution(new_constitution.clone(), None)
        .expect("Revision should succeed");
}

#[test]
fn revise_constitution_increments_revision_number() {
    let constitution = Binary::from(b"decide(_, 'gov:permitted', 'Revision allowed').".to_vec());
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let new_constitution_1 = Binary::from(b"decide(_, allowed).".to_vec());
    let new_constitution_2 = Binary::from(b"decide(_, denied).".to_vec());

    let new_constitution_1_program = std::str::from_utf8(new_constitution_1.as_slice()).unwrap();
    let new_constitution_2_program = std::str::from_utf8(new_constitution_2.as_slice()).unwrap();
    let (hook, expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(program, ask_establish_permitted())
        .then(new_constitution_1_program, ask_ok())
        .then(
            program,
            ask_decision_with_motivation("'gov:permitted'", "'Revision allowed'"),
        )
        .then(new_constitution_1_program, ask_establish_permitted())
        .then(new_constitution_2_program, ask_ok())
        .then(
            new_constitution_1_program,
            ask_decision_with_motivation("'gov:permitted'", "'Second revision allowed'"),
        )
        .then(new_constitution_2_program, ask_establish_permitted())
        .install();
    let env = TestEnv::setup(constitution.clone(), hook, expectations)
        .expect("Failed to setup test environment");

    env.app
        .revise_constitution(new_constitution_1.clone(), None)
        .expect("Failed first revision");

    let status = env
        .app
        .constitution_status()
        .expect("Failed to query constitution status");
    assert_eq!(status.constitution_revision, 1);

    env.app
        .revise_constitution(new_constitution_2.clone(), None)
        .expect("Failed second revision");

    let status = env
        .app
        .constitution_status()
        .expect("Failed to query constitution status");
    assert_eq!(status.constitution_revision, 2);
}
