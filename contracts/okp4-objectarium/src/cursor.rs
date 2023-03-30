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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proper_encode() {
        assert_eq!(encode("".to_string()), "".to_string());
        assert_eq!(encode("an_id".to_string()), "BzZCCcK".to_string());
    }

    #[test]
    fn proper_decode() {
        assert_eq!(decode("".to_string()), Ok("".to_string()));
        assert_eq!(decode("BzZCCcK".to_string()), Ok("an_id".to_string()));
    }

    #[test]
    fn invalid_decode() {
        assert_eq!(
            decode("?".to_string()),
            Err(StdError::parse_err(
                "Cursor",
                "provided string contained invalid character '?' at byte 0"
            ))
        );
        assert_eq!(
            decode("VtB5VXc".to_string()),
            Err(StdError::parse_err(
                "Cursor",
                "invalid utf-8 sequence of 1 bytes from index 0"
            ))
        );
    }
}
