use crate::state::triples::{Literal, Node, Object, Subject};
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

impl<'a> PrimaryKey<'a> for Object {
    type Prefix = ();
    type SubPrefix = ();
    type Suffix = Self;
    type SuperSuffix = Self;

    fn key(&self) -> Vec<Key> {
        match self {
            Object::Named(node) => {
                let mut key: Vec<Key> = Vec::with_capacity(3);
                key.push(Key::Val8([b'n']));
                for k in node.key() {
                    key.push(k);
                }
                key
            }
            Object::Blank(node) => {
                let mut key: Vec<Key> = Vec::with_capacity(2);
                key.push(Key::Val8([b'b']));
                key.push(Key::Ref(node.as_bytes()));
                key
            }
            Object::Literal(literal) => {
                let encoded = literal.key();
                let mut key: Vec<Key> = Vec::with_capacity(encoded.len() + 1);
                key.push(Key::Val8([b'l']));
                for k in encoded {
                    key.push(k);
                }
                key
            }
        }
    }
}

impl KeyDeserialize for Object {
    type Output = Object;

    fn from_vec(mut value: Vec<u8>) -> StdResult<Self::Output> {
        let bytes = value.split_off(3);
        match bytes[2] {
            b'n' => Node::from_vec(value).map(|n| Object::Named(n)),
            b'b' => Ok(Object::Blank(String::from_vec(value)?)),
            b'l' => Literal::from_vec(value).map(|l| Object::Literal(l)),
            _ => Err(StdError::generic_err("Could not deserialize Object")),
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

impl Literal {
    fn key(&self) -> Vec<Key> {
        match self {
            Literal::Simple { value } => {
                vec![Key::Ref(value.as_bytes()), Key::Ref(&[]), Key::Ref(&[])]
            }
            Literal::I18NString { value, language } => {
                vec![
                    Key::Ref(value.as_bytes()),
                    Key::Ref(language.as_bytes()),
                    Key::Ref(&[]),
                ]
            }
            Literal::Typed { value, datatype } => {
                let mut key: Vec<Key> = Vec::with_capacity(3);
                key.push(Key::Ref(value.as_bytes()));
                for k in datatype.key() {
                    key.push(k);
                }
                key
            }
        }
    }

    fn from_vec(mut value: Vec<u8>) -> StdResult<Self> {
        let mut part1 = value.split_off(2);
        let p1_len = parse_length(&value)?;
        let mut part2_len = part1.split_off(p1_len);

        let mut part2 = part2_len.split_off(2);
        let p2_len = parse_length(&part2_len)?;
        let part3 = part2.split_off(p2_len);

        if part3.is_empty() {
            if part2.is_empty() {
                return Ok(Literal::Simple {
                    value: String::from_vec(part1)?,
                });
            }
            return Ok(Literal::I18NString {
                value: String::from_vec(part1)?,
                language: String::from_vec(part2)?,
            });
        }
        Ok(Literal::Typed {
            value: String::from_vec(part1)?,
            datatype: Node {
                value: String::from_vec(part2)?,
                namespace: String::from_vec(part3)?,
            },
        })
    }
}
