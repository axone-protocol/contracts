use crate::addr::addr;
use cosmwasm_std::testing::{mock_env, MOCK_CONTRACT_ADDR};
use cosmwasm_std::Env;

pub fn mock_env_addr() -> Env {
    let mut env = mock_env();
    env.contract.address = addr(MOCK_CONTRACT_ADDR);
    env
}
