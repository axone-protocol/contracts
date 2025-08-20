use crate::crypto::Hash;
use crate::msg::Cursor;
use crate::state::{Object, Pin, PinPK};
use bin_it::{BinaryReader, BinaryWriter};
use cosmwasm_std::{Addr, StdError, StdResult};

pub trait AsCursor<PK> {
    fn encode_cursor(&self) -> StdResult<Cursor>;
    fn decode_cursor(_: Cursor) -> StdResult<PK>;
}

impl AsCursor<Hash> for Object {
    fn encode_cursor(&self) -> StdResult<Cursor> {
        let mut writer = BinaryWriter::new();
        writer.write_vec_u8(self.id.as_ref());

        Ok(bs58::encode(writer.get_data()).into_string())
    }

    fn decode_cursor(cursor: Cursor) -> StdResult<Hash> {
        let decoded = bs58::decode(cursor)
            .into_vec()
            .map_err(|err| StdError::parse_err("Cursor", err))?;

        let mut reader = BinaryReader::new(&decoded);
        let hash = reader
            .read_vec_u8()
            .map_err(|err| StdError::parse_err("Cursor", err))?;

        Ok(hash.into())
    }
}

impl AsCursor<PinPK> for Pin {
    fn encode_cursor(&self) -> StdResult<Cursor> {
        let mut writer = BinaryWriter::new();
        writer.write_vec_u8(self.id.as_ref());
        writer.write_string(self.address.as_str());

        Ok(bs58::encode(writer.get_data()).into_string())
    }

    fn decode_cursor(cursor: Cursor) -> StdResult<(Hash, Addr)> {
        let decoded = bs58::decode(cursor)
            .into_vec()
            .map_err(|err| StdError::parse_err("Cursor", err))?;

        let mut reader = BinaryReader::new(&decoded);
        let hash = reader
            .read_vec_u8()
            .map_err(|err| StdError::parse_err("Cursor", err))?;
        let addr = reader
            .read_string()
            .map_err(|err| StdError::parse_err("Cursor", err))?;

        Ok((hash.into(), Addr::unchecked(addr)))
    }
}

#[cfg(test)]
mod tests {}
