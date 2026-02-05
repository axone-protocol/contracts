use crate::{
    contract::{AxoneGov, AxoneGovResult},
    domain::{constitution::ConstitutionStatus, Case, Constitution},
    error::AxoneGovError,
    gateway::logic::{query_service_ask, AxoneLogicQuery, QueryServiceAskRequest},
    prolog::ast::Term,
    prolog::parser::Parser,
    prolog::term as t,
    queries::decision::build_decide_query_with_motivation,
    GOV_CTX_CURRENT_CONSTITUTION_REVISION, GOV_CTX_CURRENT_CONSTITUTION_SHA256, GOV_CTX_MODULE,
    GOV_CTX_PROPOSED_CONSTITUTION_SHA256, GOV_CTX_TX,
};
use cosmwasm_std::{Coin, Env, MessageInfo, QuerierWrapper};

pub(crate) struct DecisionOutcome {
    pub(crate) verdict: Term,
    pub(crate) motivation: Term,
}

impl DecisionOutcome {
    pub(crate) fn try_new(verdict: String, motivation: String) -> AxoneGovResult<Self> {
        let parse_term = |input: &str, label: &str| {
            Parser::new(input)
                .and_then(Parser::parse_root)
                .map_err(|err| {
                    AxoneGovError::PrologEngineError(format!(
                        "invalid {} term at offset {}: {}",
                        label, err.at, err.msg
                    ))
                })
        };

        let (verdict_term, motivation_term) = (
            parse_term(&verdict, "verdict")?,
            parse_term(&motivation, "motivation")?,
        );

        Ok(Self {
            verdict: verdict_term,
            motivation: motivation_term,
        })
    }
}

pub(crate) fn decide_case_with_motivation(
    querier: &QuerierWrapper<'_, AxoneLogicQuery>,
    program: &str,
    case: &Case,
) -> AxoneGovResult<DecisionOutcome> {
    let query = build_decide_query_with_motivation(case);
    let request = QueryServiceAskRequest::one(program, query);
    let response = query_service_ask(querier, request)?;
    let answer = response.answer.ok_or(AxoneGovError::PrologEngineNoAnswer)?;

    if let Some(error) = answer
        .results
        .iter()
        .find_map(|result| result.error.as_deref())
    {
        return Err(AxoneGovError::DecisionFailed(error.to_string()));
    }

    let result = answer
        .results
        .first()
        .ok_or(AxoneGovError::DecisionNoResult)?;

    let verdict_substitution = result
        .substitutions
        .iter()
        .find(|sub| sub.variable == "Verdict")
        .ok_or(AxoneGovError::DecisionMissingVerdict)?;
    let verdict = verdict_substitution.expression.clone();

    let motivation = result
        .substitutions
        .iter()
        .find(|sub| sub.variable == "Motivation")
        .map(|sub| sub.expression.clone())
        .ok_or(AxoneGovError::DecisionMissingMotivation)?;

    DecisionOutcome::try_new(verdict, motivation)
}

pub(crate) fn build_governance_case(
    case_input: Option<&str>,
    intent: &str,
    proposed_constitution: &Constitution,
    current_status: Option<&ConstitutionStatus>,
    module: &AxoneGov,
    env: &Env,
    info: &MessageInfo,
) -> AxoneGovResult<Case> {
    let mut case = match case_input {
        Some(input) => Case::new(input)?,
        None => Case::default(),
    };

    let mut pairs = vec![
        t::kv("intent", t::atom(intent)),
        t::kv(
            GOV_CTX_PROPOSED_CONSTITUTION_SHA256,
            t::atom(proposed_constitution.hash_hex()),
        ),
    ];

    if let Some(status) = current_status {
        pairs.push(t::kv(
            GOV_CTX_CURRENT_CONSTITUTION_SHA256,
            t::atom(status.constitution_hash_hex()),
        ));
        pairs.push(t::kv(
            GOV_CTX_CURRENT_CONSTITUTION_REVISION,
            status.constitution_revision().into(),
        ));
    }

    pairs.push(t::kv(GOV_CTX_MODULE, module_term(module)));
    pairs.push(t::kv(GOV_CTX_TX, tx_term(env, info)));

    let enrichment = Case::try_from(t::dict("ctx", pairs))?;
    case.merge(&enrichment);

    Ok(case)
}

fn coin_term(c: &Coin) -> Term {
    t::compound2("coin", c.amount.into(), t::atom(c.denom.clone()))
}

pub(crate) fn module_term(module: &AxoneGov) -> Term {
    t::dict(
        "module",
        vec![
            t::kv("id", t::atom(module.module_id())),
            t::kv("version", t::atom(module.version())),
        ],
    )
}

pub(crate) fn tx_term(env: &Env, info: &MessageInfo) -> Term {
    let sender = t::atom(info.sender.to_string());
    let funds = t::list(info.funds.iter().map(coin_term).collect());
    let block_entries: Vec<(String, Term)> = [
        t::kv("height", env.block.height.into()),
        t::kv("time_seconds", env.block.time.into()),
    ]
    .into_iter()
    .chain(
        env.transaction
            .as_ref()
            .map(|tx| t::kv("tx_index", tx.index.into())),
    )
    .collect();

    t::dict(
        "tx",
        vec![
            t::kv(
                "message",
                t::dict("msg", vec![t::kv("sender", sender), t::kv("funds", funds)]),
            ),
            t::kv("block", t::dict("block", block_entries)),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{Addr, BlockInfo, ContractInfo, Timestamp, TransactionInfo, Uint128};

    #[test]
    fn test_build_tx_term() {
        let cases = vec![
            (
                "basic case with no funds and no transaction",
                Env {
                    block: BlockInfo {
                        height: 100,
                        time: Timestamp::from_seconds(1609459200),
                        chain_id: "test-chain".to_string(),
                    },
                    transaction: None,
                    contract: ContractInfo {
                        address: Addr::unchecked("contract"),
                    },
                },
                MessageInfo {
                    sender: Addr::unchecked("sender"),
                    funds: vec![],
                },
                "tx{message: msg{sender: sender, funds: []}, block: block{height: 100, time_seconds: 1609459200}}",
            ),
            (
                "case with single coin",
                Env {
                    block: BlockInfo {
                        height: 200,
                        time: Timestamp::from_seconds(1609459300),
                        chain_id: "test-chain".to_string(),
                    },
                    transaction: None,
                    contract: ContractInfo {
                        address: Addr::unchecked("contract"),
                    },
                },
                MessageInfo {
                    sender: Addr::unchecked("alice"),
                    funds: vec![Coin {
                        denom: "uaxone".to_string(),
                        amount: Uint128::new(1000),
                    }],
                },
                "tx{message: msg{sender: alice, funds: [coin(1000, uaxone)]}, block: block{height: 200, time_seconds: 1609459300}}",
            ),
            (
                "case with multiple coins",
                Env {
                    block: BlockInfo {
                        height: 300,
                        time: Timestamp::from_seconds(1609459400),
                        chain_id: "test-chain".to_string(),
                    },
                    transaction: None,
                    contract: ContractInfo {
                        address: Addr::unchecked("contract"),
                    },
                },
                MessageInfo {
                    sender: Addr::unchecked("bob"),
                    funds: vec![
                        Coin {
                            denom: "uaxone".to_string(),
                            amount: Uint128::new(5000),
                        },
                        Coin {
                            denom: "uatom".to_string(),
                            amount: Uint128::new(2500),
                        },
                    ],
                },
                "tx{message: msg{sender: bob, funds: [coin(5000, uaxone), coin(2500, uatom)]}, block: block{height: 300, time_seconds: 1609459400}}",
            ),
            (
                "case with transaction info",
                Env {
                    block: BlockInfo {
                        height: 400,
                        time: Timestamp::from_seconds(1609459500),
                        chain_id: "test-chain".to_string(),
                    },
                    transaction: Some(TransactionInfo { index: 42 }),
                    contract: ContractInfo {
                        address: Addr::unchecked("contract"),
                    },
                },
                MessageInfo {
                    sender: Addr::unchecked("charlie"),
                    funds: vec![],
                },
                "tx{message: msg{sender: charlie, funds: []}, block: block{height: 400, time_seconds: 1609459500, tx_index: 42}}",
            ),
            (
                "case with funds and transaction",
                Env {
                    block: BlockInfo {
                        height: 500,
                        time: Timestamp::from_seconds(1609459600),
                        chain_id: "test-chain".to_string(),
                    },
                    transaction: Some(TransactionInfo { index: 99 }),
                    contract: ContractInfo {
                        address: Addr::unchecked("contract"),
                    },
                },
                MessageInfo {
                    sender: Addr::unchecked("dave"),
                    funds: vec![Coin {
                        denom: "uaxone".to_string(),
                        amount: Uint128::new(12345),
                    }],
                },
                "tx{message: msg{sender: dave, funds: [coin(12345, uaxone)]}, block: block{height: 500, time_seconds: 1609459600, tx_index: 99}}",
            ),
        ];

        for (description, env, info, expected) in cases {
            let got = tx_term(&env, &info).to_string();
            assert_eq!(
                got, expected,
                "test case failed: {}\nexpected: {}\ngot: {}",
                description, expected, got
            );
        }
    }
}
