use axone_cognitarium::msg::{DataFormat, ExecuteMsg, QueryMsg, SelectQuery, SelectResponse};
use cosmwasm_std::{
    to_json_binary, Addr, Binary, Coin, CustomQuery, QuerierWrapper, QueryRequest, StdResult,
    WasmMsg, WasmQuery,
};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct CognitariumClient {
    address: Addr,
}

impl CognitariumClient {
    pub fn new(address: Addr) -> Self {
        Self { address }
    }

    pub fn select<C: CustomQuery>(
        &self,
        querier: QuerierWrapper<'_, C>,
        query: SelectQuery,
    ) -> StdResult<SelectResponse> {
        self.query_wasm(querier, &QueryMsg::Select { query })
    }

    pub fn insert_data(&self, format: Option<DataFormat>, data: Binary) -> StdResult<WasmMsg> {
        self.to_wasm_exec_msg(&ExecuteMsg::InsertData { format, data }, vec![])
    }

    fn query_wasm<C, T, U>(&self, querier: QuerierWrapper<'_, C>, msg: &T) -> StdResult<U>
    where
        C: CustomQuery,
        T: Serialize + ?Sized,
        U: DeserializeOwned,
    {
        querier.query(&QueryRequest::Wasm(self.to_wasm_query_msg(msg)?))
    }

    fn to_wasm_exec_msg<T>(&self, msg: &T, funds: Vec<Coin>) -> StdResult<WasmMsg>
    where
        T: Serialize + ?Sized,
    {
        Ok(WasmMsg::Execute {
            contract_addr: self.address.to_string(),
            msg: to_json_binary(msg)?,
            funds,
        })
    }

    fn to_wasm_query_msg<T>(&self, msg: &T) -> StdResult<WasmQuery>
    where
        T: Serialize + ?Sized,
    {
        Ok(WasmQuery::Smart {
            contract_addr: self.address.to_string(),
            msg: to_json_binary(msg)?,
        })
    }
}
