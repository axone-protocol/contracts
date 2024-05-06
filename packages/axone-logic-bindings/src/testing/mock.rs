use crate::LogicCustomQuery;
use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage};
use cosmwasm_std::{Coin, OwnedDeps, QuerierResult};
use std::marker::PhantomData;

pub fn mock_dependencies_with_logic_handler<LH>(
    handler: LH,
) -> OwnedDeps<MockStorage, MockApi, MockQuerier<LogicCustomQuery>, LogicCustomQuery>
where
    LH: Fn(&LogicCustomQuery) -> QuerierResult + 'static,
{
    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: MockLogicQuerier::new(LogicQuerier::new(Box::new(handler)), &[]),
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
    fn update_handler<LH>(&mut self, handler: LH)
    where
        LH: Fn(&LogicCustomQuery) -> QuerierResult + 'static,
    {
        self.handler = Box::from(handler);
    }
}
