use crate::{
    contract::{AxoneGov, AxoneGovResult},
    domain::{Case, Constitution},
    error::AxoneGovError,
    gateway::logic::{query_service_ask, AxoneLogicQuery, QueryServiceAskRequest},
    msg::AxoneGovExecuteMsg,
    prolog::ast::Term,
    queries::decision::{build_decide_query, build_decide_query_with_motivation},
    state::{load_constitution, load_constitution_status, save_revised_constitution},
    GOV_CTX_COSMWASM, GOV_CTX_MODULE, GOV_INTENT_REVISE_CONSTITUTION, GOV_VERDICT_PERMITTED,
    RESPONSE_KEY_CASE_HASH, RESPONSE_KEY_CONSTITUTION_HASH, RESPONSE_KEY_CONSTITUTION_REVISER,
    RESPONSE_KEY_CONSTITUTION_REVISION, RESPONSE_KEY_DECISION_ID, RESPONSE_KEY_MOTIVATION_HASH,
    RESPONSE_KEY_VERDICT, RESPONSE_KEY_VERDICT_HASH,
};

use crate::domain::Decision;
use crate::prolog::term as t;
use crate::state::record_decision;
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
            execute_revise_constitution(deps, env, info, module, constitution, case)
        }
        AxoneGovExecuteMsg::RecordDecision { case, motivated } => {
            execute_record_decision(deps, env, info, module, case, motivated.unwrap_or(false))
        }
    }
}

fn execute_revise_constitution(
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
            t::kv("intent", t::atom(GOV_INTENT_REVISE_CONSTITUTION)),
            t::kv(
                "gov:proposed_constitution_hash",
                t::atom(revised_constitution.hash_hex()),
            ),
            t::kv(GOV_CTX_MODULE, module_term(&module)),
            t::kv(GOV_CTX_COSMWASM, cosmwasm_term(&env, &info)),
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

    let authorized = verdict_term == t::atom(GOV_VERDICT_PERMITTED);

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
                RESPONSE_KEY_CONSTITUTION_REVISION.to_string(),
                status.constitution_revision().to_string(),
            ),
            (
                RESPONSE_KEY_CONSTITUTION_HASH.to_string(),
                status.constitution_hash_hex(),
            ),
            (
                RESPONSE_KEY_CONSTITUTION_REVISER.to_string(),
                info.sender.to_string(),
            ),
        ],
    ))
}

fn execute_record_decision(
    deps: DepsMut<'_>,
    env: Env,
    info: MessageInfo,
    module: AxoneGov,
    case_input: String,
    motivated: bool,
) -> AxoneGovResult {
    let mut case = Case::new(&case_input)?;
    let enrichment_term = t::dict(
        "ctx",
        vec![
            t::kv(GOV_CTX_MODULE, module_term(&module)),
            t::kv(GOV_CTX_COSMWASM, cosmwasm_term(&env, &info)),
        ],
    );

    let enrichment = Case::try_from(enrichment_term)?;
    case.merge(&enrichment);

    let case_term = case.to_string();

    let constitution = load_constitution(deps.storage)?;
    let status = load_constitution_status(deps.storage)?;
    let program = constitution.source();
    let query = if motivated {
        build_decide_query_with_motivation(&case)
    } else {
        build_decide_query(&case)
    };

    let request = QueryServiceAskRequest::one(program, query);
    let response = query_service_ask(
        &QuerierWrapper::<AxoneLogicQuery>::new(&*deps.querier),
        request,
    )?;
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
    let verdict =
        find_substitution(result, "Verdict").ok_or(AxoneGovError::DecisionMissingVerdict)?;
    let motivation = if motivated {
        Some(
            find_substitution(result, "Motivation")
                .ok_or(AxoneGovError::DecisionMissingMotivation)?,
        )
    } else {
        None
    };

    let decision = Decision::new(
        &status,
        case_term,
        verdict,
        motivation,
        info.sender,
        env.block.height,
        env.block.time.seconds(),
    );

    let decision_record = record_decision(deps.storage, decision)?;
    let mut attrs = vec![
        (
            RESPONSE_KEY_DECISION_ID.to_string(),
            decision_record.id().to_string(),
        ),
        (
            RESPONSE_KEY_CONSTITUTION_REVISION.to_string(),
            decision_record.constitution_revision().to_string(),
        ),
        (
            RESPONSE_KEY_CONSTITUTION_HASH.to_string(),
            decision_record.constitution_hash_hex(),
        ),
        (
            RESPONSE_KEY_CASE_HASH.to_string(),
            decision_record.case_hash_hex(),
        ),
        (
            RESPONSE_KEY_VERDICT.to_string(),
            decision_record.verdict().clone(),
        ),
        (
            RESPONSE_KEY_VERDICT_HASH.to_string(),
            decision_record.verdict_hash_hex(),
        ),
    ];

    if let Some(h) = decision_record.motivation_hash_hex() {
        attrs.push((RESPONSE_KEY_MOTIVATION_HASH.to_string(), h));
    }

    Ok(module.custom_response("record_decision", attrs))
}

fn find_substitution(result: &crate::gateway::logic::Result, variable: &str) -> Option<String> {
    result
        .substitutions
        .iter()
        .find(|sub| sub.variable == variable)
        .map(|sub| sub.expression.clone())
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
