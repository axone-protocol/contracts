use std::marker::PhantomData;
use cosmwasm_std::{Coin, OwnedDeps, QuerierResult, SystemResult, to_binary};
use cosmwasm_std::testing::{MOCK_CONTRACT_ADDR, MockApi, MockQuerier, MockStorage};
use crate::{Answer, AskResponse, LogicCustomQuery, Substitution, Term};

/// Creates all external requirements that can be injected for unit tests.
///
/// It sets the given balance for the contract itself, nothing else and set the custom default logic
/// querier handler.
pub fn mock_dependencies_with_logic_and_balance( contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, MockQuerier<LogicCustomQuery>, LogicCustomQuery> {
    mock_dependencies_with_logic_and_balances(&[(MOCK_CONTRACT_ADDR, contract_balance)])
}

/// Initializes the querier along with the mock_dependencies.
///
/// Set the logic querier mock handler.
/// Sets all balances provided (you must explicitly set contract balance if desired).
pub fn mock_dependencies_with_logic_and_balances(balances: &[(&str, &[Coin])]) -> OwnedDeps<MockStorage, MockApi, MockQuerier<LogicCustomQuery>, LogicCustomQuery> {
    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: MockLogicQuerier::new(LogicQuerier::default(), balances),
        custom_query_type: PhantomData,
    }

}

trait MockLogicQuerier {
    fn new(logic: LogicQuerier, balances: &[(&str, &[Coin])]) -> Self;
}

impl MockLogicQuerier for MockQuerier<LogicCustomQuery> {
    fn new(logic: LogicQuerier, balances: &[(&str, &[Coin])]) -> Self {
        MockQuerier::new(balances).with_custom_handler(Box::new(logic.handler))
    }
}

struct LogicQuerier {
    /// A handler to handle Logic queries. This is set to a dummy handler that
    /// always return a successful foo / bar response by default. Update it via `update_handler`.
    ///
    /// Use box to avoid the need of generic type.
    handler: Box<dyn for<'a> Fn(&'a LogicCustomQuery) -> QuerierResult>,
}

impl LogicQuerier {
    fn new(handler: Box<dyn for<'a> Fn(&'a LogicCustomQuery) -> QuerierResult>) -> Self {
        Self { handler }
    }

    #[allow(dead_code)]
    fn update_handler<LH: 'static>(&mut self, handler: LH)
        where
            LH: Fn(&LogicCustomQuery) -> QuerierResult,
    {
        self.handler = Box::from(handler)
    }
}

impl Default for LogicQuerier {
    fn default() -> Self {
        let handler = Box::from(|request: &LogicCustomQuery| -> QuerierResult {
            let result = match request {
                LogicCustomQuery::Ask { .. } => to_binary(&AskResponse {
                    height: 1,
                    gas_used: 1000,
                    answer: Some(Answer {
                        success: true,
                        has_more: false,
                        variables: vec!["foo".to_string()],
                        results: vec![
                            crate::Result {
                                substitutions: vec![Substitution {
                                    variable: "foo".to_string(),
                                    term: Term {
                                        name: "bar".to_string(),
                                        arguments: vec![]
                                    }
                                }]  }
                            ],
                    }),
                }),
            };
            SystemResult::Ok(result.into())
        });
        Self::new(handler)
    }
}
