use crate::state::triples::{Node, Subject};
use cosmwasm_std::{StdError, StdResult};
use cw_storage_plus::{IntKey, Key, KeyDeserialize, Prefixer, PrimaryKey};
use std::array::TryFromSliceError;

fn parse_length(value: &[u8]) -> StdResult<usize> {
    Ok(u16::from_be_bytes(
        value
            .try_into()
            .map_err(|_| StdError::generic_err("Could not read 2 byte length"))?,
    )
    .into())
}

impl<'a> PrimaryKey<'a> for Subject {
    type Prefix = ();
    type SubPrefix = ();
    type Suffix = Self;
    type SuperSuffix = Self;

    fn key(&self) -> Vec<Key> {
        match self {
            Subject::Named(node) => {
                let mut keys = Vec::new();
                keys.push(Key::Val8([b'n']));
                for x in node.key() {
                    keys.push(x);
                }
                keys
            }
            Subject::Blank(node) => vec![Key::Val8([b'n']), Key::Ref(node.as_bytes())],
        }
    }
}

impl<'a> Prefixer<'a> for Subject {
    fn prefix(&self) -> Vec<Key> {
        self.key()
    }
}

impl KeyDeserialize for Subject {
    type Output = Subject;

    fn from_vec(mut value: Vec<u8>) -> StdResult<Self::Output> {
        let val = value.split_off(3);
        match val[2] {
            b'n' => Node::from_vec(value).map(Subject::Named),
            b'b' => String::from_vec(value).map(Subject::Blank),
            _ => Err(StdError::generic_err("Could not deserialize subject key")),
        }
    }
}

impl<'a> PrimaryKey<'a> for Node {
    type Prefix = ();
    type SubPrefix = ();
    type Suffix = Self;
    type SuperSuffix = Self;

    fn key(&self) -> Vec<Key> {
        vec![
            Key::Val128(self.namespace.to_cw_bytes()),
            Key::Ref(self.value.as_bytes()),
        ]
    }
}

impl<'a> Prefixer<'a> for Node {
    fn prefix(&self) -> Vec<Key> {
        self.key()
    }
}

impl KeyDeserialize for Node {
    type Output = Node;

    fn from_vec(mut value: Vec<u8>) -> StdResult<Self::Output> {
        let mut val = value.split_off(2);
        let n_len = parse_length(&value)?;
        let ns = val.split_off(n_len);

        Ok(Node {
            namespace: u128::from_cw_bytes(
                ns.as_slice()
                    .try_into()
                    .map_err(|e: TryFromSliceError| StdError::generic_err(e.to_string()))?,
            ),
            value: String::from_vec(val)?,
        })
    }
}
