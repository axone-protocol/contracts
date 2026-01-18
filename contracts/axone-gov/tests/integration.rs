use abstract_app::objects::namespace::Namespace;
use abstract_client::{AbstractClient, Application};
use axone_gov::{
    gateway::logic::{
        set_query_service_ask_handler, Answer, QueryServiceAskMockGuard, QueryServiceAskResponse,
        Result as LogicResult, Substitution,
    },
    msg::{AxoneGovExecuteMsgFns, AxoneGovInstantiateMsg, AxoneGovQueryMsgFns},
    AxoneGovInterface, AXONE_NAMESPACE,
};
use cosmwasm_std::{Binary, Checksum};
use cw_orch::{anyhow, prelude::*};
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

#[derive(Clone)]
struct LogicAskExpectations(Rc<RefCell<VecDeque<(String, QueryServiceAskResponse)>>>);

impl Drop for LogicAskExpectations {
    fn drop(&mut self) {
        let remaining = self.0.borrow().len();
        assert_eq!(
            remaining, 0,
            "not all Logic::ask expectations were consumed (remaining={remaining})"
        );
    }
}

struct LogicAskScenario {
    expected: Rc<RefCell<VecDeque<(String, QueryServiceAskResponse)>>>,
}

impl LogicAskScenario {
    fn new() -> Self {
        Self {
            expected: Rc::new(RefCell::new(VecDeque::new())),
        }
    }

    fn then(self, program: impl Into<String>, response: QueryServiceAskResponse) -> Self {
        self.expected
            .borrow_mut()
            .push_back((program.into(), response));
        self
    }

    fn install(self) -> (QueryServiceAskMockGuard, LogicAskExpectations) {
        let expected = self.expected.clone();
        let hook: QueryServiceAskMockGuard = set_query_service_ask_handler(move |request| {
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

#[test]
fn instantiate_succeeds_with_valid_constitution() {
    let constitution = Binary::from(b"valid.".to_vec());
    let (hook, expectations) = LogicAskScenario::new().then("valid.", ask_ok()).install();
    let env = TestEnv::setup(constitution.clone(), hook, expectations)
        .expect("Failed to setup test environment");

    let constitution_got = env
        .app
        .constitution()
        .expect("Failed to query constitution");
    assert_eq!(constitution_got.governance, constitution);

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
            .then(program, ask_decision_without_motivation("allowed"))
            .install();
        let env =
            TestEnv::setup(constitution.clone(), hook, expectations).expect("Failed to setup test");

        let response = env
            .app
            .decide(case.to_string(), false)
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
        .then(
            program,
            ask_decision_with_motivation("allowed", "'User is authorized'"),
        )
        .install();
    let env =
        TestEnv::setup(constitution.clone(), hook, expectations).expect("Failed to setup test");

    let response = env
        .app
        .decide("case{action:transfer}".to_string(), true)
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
    let (hook, expectations) = LogicAskScenario::new().then(program, ask_ok()).install();
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
            .decide(case.to_string(), false)
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
        .then(program, ask_no_answer())
        .install();
    let env =
        TestEnv::setup(constitution.clone(), hook, expectations).expect("Failed to setup test");

    let err = env
        .app
        .decide("case{action:test}".to_string(), false)
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
        .then(program, ask_empty_results())
        .install();
    let env =
        TestEnv::setup(constitution.clone(), hook, expectations).expect("Failed to setup test");

    let err = env
        .app
        .decide("case{action:test}".to_string(), false)
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
        .then(program, ask_error("predicate failed"))
        .install();
    let env =
        TestEnv::setup(constitution.clone(), hook, expectations).expect("Failed to setup test");

    let err = env
        .app
        .decide("case{action:test}".to_string(), false)
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
        .decide("case{action:test}".to_string(), false)
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
        .then(program, ask_decision_without_motivation("allowed"))
        .install();
    let env =
        TestEnv::setup(constitution.clone(), hook, expectations).expect("Failed to setup test");

    let err = env
        .app
        .decide("case{action:test}".to_string(), true)
        .expect_err("Expected missing motivation error");

    let msg = format!("{err:?}");
    assert!(
        msg.contains("decision motivation missing"),
        "expected decision motivation missing, got: {msg}"
    );
}

#[test]
fn revise_constitution_succeeds_with_permitted_verdict() {
    let constitution = Binary::from(b"decide(_, 'gov:permitted', 'Revision allowed').".to_vec());
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let new_constitution = Binary::from(b"decide(_, allowed).".to_vec());

    let new_constitution_program = std::str::from_utf8(new_constitution.as_slice()).unwrap();
    let (hook, expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        .then(new_constitution_program, ask_ok())
        .then(
            program,
            ask_decision_with_motivation("'gov:permitted'", "'Revision allowed'"),
        )
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
    assert_eq!(constitution_got.governance, new_constitution);

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
        .then(new_constitution_program, ask_ok())
        .then(
            program,
            ask_decision_with_motivation("'gov:permitted'", "'Revision allowed'"),
        )
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
    assert_eq!(constitution_got.governance, new_constitution);

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
        .expect_err("Expected revision refused error");

    let msg = format!("{err:?}");
    assert!(
        msg.contains("revision refused"),
        "expected revision refused, got: {msg}"
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
fn revise_constitution_increments_revision_number() {
    let constitution = Binary::from(b"decide(_, 'gov:permitted', 'Revision allowed').".to_vec());
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let new_constitution_1 = Binary::from(b"decide(_, allowed).".to_vec());
    let new_constitution_2 = Binary::from(b"decide(_, denied).".to_vec());

    let new_constitution_1_program = std::str::from_utf8(new_constitution_1.as_slice()).unwrap();
    let new_constitution_2_program = std::str::from_utf8(new_constitution_2.as_slice()).unwrap();
    let (hook, expectations) = LogicAskScenario::new()
        .then(program, ask_ok())
        // First revision: validate new_constitution_1, then decide on current (program)
        .then(new_constitution_1_program, ask_ok())
        .then(
            program,
            ask_decision_with_motivation("'gov:permitted'", "'Revision allowed'"),
        )
        // Second revision: validate new_constitution_2, then decide on current (new_constitution_1)
        .then(new_constitution_2_program, ask_ok())
        .then(
            new_constitution_1_program,
            ask_decision_with_motivation("'gov:permitted'", "'Second revision allowed'"),
        )
        .install();
    let env = TestEnv::setup(constitution.clone(), hook, expectations)
        .expect("Failed to setup test environment");

    // First revision
    env.app
        .revise_constitution(new_constitution_1.clone(), None)
        .expect("Failed first revision");

    let status = env
        .app
        .constitution_status()
        .expect("Failed to query constitution status");
    assert_eq!(status.constitution_revision, 1);

    // Second revision
    env.app
        .revise_constitution(new_constitution_2.clone(), None)
        .expect("Failed second revision");

    let status = env
        .app
        .constitution_status()
        .expect("Failed to query constitution status");
    assert_eq!(status.constitution_revision, 2);
}
