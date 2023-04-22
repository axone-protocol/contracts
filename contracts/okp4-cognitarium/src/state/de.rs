use crate::state::triples::{Node, Subject};
use cosmwasm_std::{StdError, StdResult};
use cw_storage_plus::{Key, KeyDeserialize, Prefixer, PrimaryKey};

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
            Subject::Named(node) => node.key(),
            Subject::Blank(node) => vec![Key::Ref(&[]), Key::Ref(node.as_bytes())],
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

    fn from_vec(value: Vec<u8>) -> StdResult<Self::Output> {
        let named = Node::from_vec(value)?;
        if named.namespace.is_empty() {
            return Ok(Subject::Blank(named.value));
        }
        Ok(Subject::Named(named))
    }
}

impl<'a> PrimaryKey<'a> for Node {
    type Prefix = ();
    type SubPrefix = ();
    type Suffix = Self;
    type SuperSuffix = Self;

    fn key(&self) -> Vec<Key> {
        vec![
            Key::Ref(self.namespace.as_bytes()),
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
            namespace: String::from_vec(ns)?,
            value: String::from_vec(val)?,
        })
    }
}
