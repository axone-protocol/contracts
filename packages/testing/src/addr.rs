use cosmwasm_std::testing::MockApi;
use cosmwasm_std::Addr;

pub const CREATOR: &str = "creator";
pub const SENDER: &str = "sender";
pub const OWNER: &str = "owner";

pub fn addr(input: &str) -> Addr {
    MockApi::default().addr_make(input)
}
