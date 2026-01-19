use crate::{
    contract::{AxoneGov, AxoneGovResult},
    domain::{Case, Constitution},
    error::AxoneGovError,
    gateway::logic::{query_service_ask, AxoneLogicQuery, QueryServiceAskRequest},
    msg::AxoneGovExecuteMsg,
    prolog::ast::Term,
    queries::decision::build_decide_query_with_motivation,
    state::{load_constitution, save_revised_constitution},
};

use crate::prolog::term as t;
use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{Binary, Coin, DepsMut, Env, MessageInfo, QuerierWrapper};

#[allow(clippy::unnecessary_wraps)]
pub fn execute_handler(
    deps: DepsMut<'_>,
    env: Env,
    info: MessageInfo,
    module: AxoneGov,
    msg: AxoneGovExecuteMsg,
) -> AxoneGovResult {
    match msg {
        AxoneGovExecuteMsg::ReviseConstitution { constitution, case } => {
            revise_constitution(deps, env, info, module, constitution, case)
        }
    }
}

fn revise_constitution(
    deps: DepsMut<'_>,
    env: Env,
    info: MessageInfo,
    module: AxoneGov,
    constitution_bytes: Binary,
    case_input: Option<String>,
) -> AxoneGovResult {
    let querier = QuerierWrapper::<AxoneLogicQuery>::new(&*deps.querier);
    let revised_constitution = Constitution::try_new(constitution_bytes, &querier)?;

    let mut case = if let Some(input) = case_input {
        Case::new(&input)?
    } else {
        Case::default()
    };

    let enrichment_term = t::dict(
        "ctx",
        vec![
            t::kv("intent", t::atom("gov:revise_constitution")),
            t::kv("gov:module", module_term(&module)),
            t::kv("gov:cosmwasm", cosmwasm_term(&env, &info)),
        ],
    );

    let enrichment = Case::try_from(enrichment_term)?;
    case.merge(&enrichment);
    let current_constitution = load_constitution(deps.storage)?;
    let program = current_constitution.source();
    let query = build_decide_query_with_motivation(&case);

    let request = QueryServiceAskRequest::one(program, query);
    let response = query_service_ask(&querier, request)?;
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
    let verdict_term = verdict_substitution.expression_term().map_err(|err| {
        AxoneGovError::PrologEngineError(format!(
            "invalid verdict term at offset {}: {}",
            err.at, err.msg
        ))
    })?;

    let motivation = result
        .substitutions
        .iter()
        .find(|sub| sub.variable == "Motivation")
        .map(|sub| sub.expression.clone())
        .ok_or(AxoneGovError::DecisionMissingMotivation)?;

    let authorized = verdict_term == t::atom("gov:permitted");

    if !authorized {
        return Err(AxoneGovError::RevisionRefused {
            verdict,
            motivation,
        });
    }

    let status = save_revised_constitution(deps.storage, &revised_constitution)?;

    Ok(module.custom_response(
        "revise_constitution",
        vec![
            (
                "constitution_revision".to_string(),
                status.constitution_revision().to_string(),
            ),
            (
                "constitution_hash".to_string(),
                status.constitution_hash_hex(),
            ),
            ("constitution_reviser".to_string(), info.sender.to_string()),
        ],
    ))
}

fn coin_term(c: &Coin) -> Term {
    t::compound2("coin", c.amount.into(), t::atom(c.denom.clone()))
}

fn module_term(module: &AxoneGov) -> Term {
    t::dict(
        "module",
        vec![
            t::kv("id", t::atom(module.module_id())),
            t::kv("version", t::atom(module.version())),
        ],
    )
}

fn cosmwasm_term(env: &Env, info: &MessageInfo) -> Term {
    let sender = t::atom(info.sender.to_string());
    let funds = t::list(info.funds.iter().map(coin_term).collect());
    let block_entries: Vec<(String, Term)> = [
        t::kv("height", env.block.height.into()),
        t::kv("time", env.block.time.into()),
    ]
    .into_iter()
    .chain(
        env.transaction
            .as_ref()
            .map(|tx| t::kv("tx_index", tx.index.into())),
    )
    .collect();

    t::dict(
        "cosmwasm",
        vec![
            t::kv(
                "message",
                t::dict(
                    "message",
                    vec![t::kv("sender", sender), t::kv("funds", funds)],
                ),
            ),
            t::kv("block", t::dict("block", block_entries)),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{Addr, BlockInfo, Coin, ContractInfo, Timestamp, TransactionInfo, Uint128};

    #[test]
    fn test_build_cosmwasm_term() {
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
                "cosmwasm{message: message{sender: sender, funds: []}, block: block{height: 100, time: 1609459200}}",
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
                "cosmwasm{message: message{sender: alice, funds: [coin(1000, uaxone)]}, block: block{height: 200, time: 1609459300}}",
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
                "cosmwasm{message: message{sender: bob, funds: [coin(5000, uaxone), coin(2500, uatom)]}, block: block{height: 300, time: 1609459400}}",
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
                "cosmwasm{message: message{sender: charlie, funds: []}, block: block{height: 400, time: 1609459500, tx_index: 42}}",
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
                "cosmwasm{message: message{sender: dave, funds: [coin(12345, uaxone)]}, block: block{height: 500, time: 1609459600, tx_index: 99}}",
            ),
        ];

        for (description, env, info, expected) in cases {
            let got = cosmwasm_term(&env, &info).to_string();
            assert_eq!(
                got, expected,
                "test case failed: {}\nexpected: {}\ngot: {}",
                description, expected, got
            );
        }
    }
}
