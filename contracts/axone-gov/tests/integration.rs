use abstract_app::objects::namespace::Namespace;
use abstract_client::{AbstractClient, Application};
use axone_gov::{
    gateway::logic::{
        set_query_service_ask_handler, Answer, QueryServiceAskMockGuard, QueryServiceAskResponse,
        Result as LogicResult, Substitution,
    },
    msg::{AxoneGovInstantiateMsg, AxoneGovQueryMsgFns},
    AxoneGovInterface, AXONE_NAMESPACE,
};
use cosmwasm_std::Binary;
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
fn decide_succeeds_without_motivation() {
    let constitution = Binary::from(
        b"decide(case{action:transfer}, allowed).
decide(case{action:withdraw}, denied)."
            .to_vec(),
    );
    let program = std::str::from_utf8(constitution.as_slice()).unwrap();
    let (hook, expectations) = LogicAskScenario::new()
        .then(
            program,
            ask_ok(), // constitution validation
        )
        .then(
            program,
            ask_with_substitutions(vec![Substitution {
                variable: "Verdict".to_string(),
                expression: "allowed".to_string(),
            }]),
        )
        .install();
    let env =
        TestEnv::setup(constitution.clone(), hook, expectations).expect("Failed to setup test");

    let response = env
        .app
        .decide("case{action:transfer}".to_string(), false)
        .expect("Failed to query decide");

    assert_eq!(response.verdict, "allowed");
    assert_eq!(response.motivation, None);
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
            ask_with_substitutions(vec![
                Substitution {
                    variable: "Verdict".to_string(),
                    expression: "allowed".to_string(),
                },
                Substitution {
                    variable: "Motivation".to_string(),
                    expression: "'User is authorized'".to_string(),
                },
            ]),
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
        .then(
            program,
            ask_with_substitutions(vec![Substitution {
                variable: "Verdict".to_string(),
                expression: "allowed".to_string(),
            }]),
        )
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
