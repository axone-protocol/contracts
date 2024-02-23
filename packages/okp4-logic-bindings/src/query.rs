use crate::error::TermParseError;
use crate::term_parser::{from_str, TermValue};
use cosmwasm_std::CustomQuery;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum LogicCustomQuery {
    Ask { program: String, query: String },
}

impl CustomQuery for LogicCustomQuery {}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Eq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct AskResponse {
    pub height: u64,
    pub gas_used: u64,
    pub answer: Option<Answer>,
}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Eq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Answer {
    pub success: bool,
    pub error: Option<String>,
    pub has_more: bool,
    pub variables: Vec<String>,
    pub results: Vec<Result>,
}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Eq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Result {
    pub substitutions: Vec<Substitution>,
}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Eq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Substitution {
    pub variable: String,
    pub expression: String,
}

impl Substitution {
    pub fn parse_expression(self) -> std::result::Result<TermValue, TermParseError> {
        from_str(self.expression.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn term_parse() {
        assert_eq!(
            Substitution {
                variable: "X".to_string(),
                expression: "'hello'".to_string(),
            }
            .parse_expression(),
            Ok(TermValue::Value("hello".to_string()))
        );
    }
}
