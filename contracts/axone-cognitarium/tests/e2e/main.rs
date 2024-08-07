use axone_cognitarium::contract::{execute, instantiate};
use axone_cognitarium::ContractError;
use base64::engine::general_purpose;
use base64::Engine;
use cosmwasm_std::testing::{
    message_info, mock_dependencies, mock_env, MockApi, MockQuerier, MockStorage,
};
use cosmwasm_std::{MessageInfo, OwnedDeps, Response};
use cucumber::parser::{Basic, Error};
use cucumber::{gherkin, given, then, when, World};
use futures::{stream, TryStreamExt};
use serde_yaml::Value;
use std::fmt::Debug;
use std::path::Path;
use std::vec;
use testing::addr::addr;

#[derive(World)]
#[world(init = Self::new)]
pub struct SmartContractWorld {
    deps: OwnedDeps<MockStorage, MockApi, MockQuerier>,
    info: MessageInfo,
    response: Result<Response, ContractError>,
}

impl SmartContractWorld {
    fn new() -> Self {
        SmartContractWorld {
            deps: mock_dependencies(),
            info: message_info(&addr("owner"), &[]),
            response: Ok(Response::new()),
        }
    }
}

impl Debug for SmartContractWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SmartContractWorld")
            .field("info", &self.info)
            .field("response", &self.response)
            .finish()
    }
}

struct CustomParser;

impl<I: AsRef<Path>> cucumber::Parser<I> for CustomParser {
    type Cli = <Basic as cucumber::Parser<I>>::Cli;
    type Output = stream::MapOk<
        stream::Iter<vec::IntoIter<Result<gherkin::Feature, Error>>>,
        fn(gherkin::Feature) -> gherkin::Feature,
    >;

    fn parse(self, input: I, cli: Self::Cli) -> Self::Output {
        Basic::new().parse(input, cli).map_ok(|mut feature| {
            feature.scenarios = feature
                .scenarios
                .into_iter()
                .map(|mut scenario| {
                    scenario.steps = scenario
                        .steps
                        .into_iter()
                        .map(|mut step| {
                            Self::convert_step_data_to_base64(&mut step);
                            step
                        })
                        .collect();
                    scenario
                })
                .collect();
            feature
        })
    }
}

impl CustomParser {
    /// Convert the data in the step to base64 if it is a yaml docstring with a 'data' field
    /// that is a string. This allows to keep the data in human-readable format in the feature rather
    /// than base64.
    fn convert_step_data_to_base64(step: &mut gherkin::Step) {
        if let Some(docstring) = &step.docstring {
            if let ("yaml", content) = extract_type_and_content(docstring) {
                let mut value: Value = serde_yaml::from_str(content).unwrap();

                if let Value::Tagged(ref mut tag) = value {
                    if let Some(data_value) = tag.value.get_mut("data") {
                        if let Value::String(data) = data_value {
                            *data = general_purpose::STANDARD.encode(&data);
                        }
                    }
                }
                step.docstring = Some(format!("yaml\n{}", serde_yaml::to_string(&value).unwrap()));
            }
        }
    }
}

#[given(regex = r"^a smart contract instantiated with message:$")]
fn smart_contract_instantiated_with_message(world: &mut SmartContractWorld, step: &gherkin::Step) {
    match &step.docstring {
        Some(docstring) => {
            let content = extract_yaml_content(docstring).unwrap();
            let instantiate_msg = serde_yaml::from_str(content).unwrap();

            instantiate(
                world.deps.as_mut(),
                mock_env(),
                world.info.clone(),
                instantiate_msg,
            )
            .unwrap();
            return;
        }
        _ => panic!("No message provided"),
    }
}

#[when(regex = r"^the smart contract is called with the following execute message:$")]
fn the_smart_contract_is_called_with_the_following_execute_message(
    world: &mut SmartContractWorld,
    step: &gherkin::Step,
) {
    match &step.docstring {
        Some(docstring) => {
            let content = extract_yaml_content(docstring).unwrap();
            let insert_data_msg = serde_yaml::from_str(content).unwrap();

            world.response = execute(
                world.deps.as_mut(),
                mock_env(),
                world.info.clone(),
                insert_data_msg,
            );
        }
        None => panic!("No message provided"),
    };
}

#[then(regex = r"^response is (successful|error)$")]
#[allow(unused_variables)]
fn response_is_successful_or_error(
    world: &mut SmartContractWorld,
    step: &gherkin::Step,
    status: String,
) {
    match status.as_str() {
        "successful" => assert!(world.response.is_ok()),
        "error" => assert!(world.response.is_err()),
        _ => unreachable!(),
    }
}

#[then(regex = r"^response attributes should be:$")]
fn response_attributes_should_be(world: &mut SmartContractWorld, step: &gherkin::Step) {
    if let Some(table) = &step.table {
        let response = world.response.as_ref().unwrap();

        for row in &table.rows {
            let key = &row[0];
            let value = &row[1];

            if let Some(attr) = response.attributes.iter().find(|&attr| attr.key == *key) {
                assert_eq!(
                    attr.value, *value,
                    "Expected attribute '{}' to have value '{}', but found '{}'",
                    key, value, attr.value
                );
            } else {
                panic!(
                    "Expected attribute '{}' with value '{}' was not found",
                    key, value
                );
            }
        }
    } else {
        panic!("No attributes provided");
    }
}

fn extract_type_and_content(docstring: &str) -> (&str, &str) {
    let (doctype, content) = docstring.split_once('\n').unwrap_or((docstring, ""));
    (doctype, content)
}

fn extract_yaml_content(docstring: &str) -> Result<&str, &str> {
    match extract_type_and_content(docstring) {
        ("yaml", content) => Ok(content),
        _ => Err("only yaml docstrings are supported"),
    }
}

fn main() {
    futures::executor::block_on(
        SmartContractWorld::cucumber::<&str>()
            .with_parser(CustomParser)
            .run("tests/e2e/features"),
    );
}
