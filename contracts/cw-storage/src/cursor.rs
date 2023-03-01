use crate::msg::{Cursor, ObjectId};
use cosmwasm_std::{StdError, StdResult};

pub fn encode(id: ObjectId) -> Cursor {
    bs58::encode(id).into_string()
}

pub fn decode(cursor: String) -> StdResult<Cursor> {
    let raw = bs58::decode(cursor)
        .into_vec()
        .map_err(|err| StdError::parse_err("Cursor", err))?;

    String::from_utf8(raw).map_err(|err| StdError::parse_err("Cursor", err))
}
