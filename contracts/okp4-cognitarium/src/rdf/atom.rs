#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum Value {
    NamedNode(String),
    BlankNode(String),
    LiteralSimple(String),
    LiteralLang(String, String),
    LiteralDatatype(String, String),
}

use std::fmt;
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::NamedNode(s) => write!(f, "{}", s),
            Value::BlankNode(s) => write!(f, "{}", s),
            Value::LiteralSimple(s) => write!(f, "{}", s),
            Value::LiteralLang(s, l) => write!(f, "{}@{}", s, l),
            Value::LiteralDatatype(s, d) => write!(f, "{}^^{}", s, d),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Atom {
    pub subject: String,
    pub property: String,
    pub value: Value,
}

impl std::fmt::Display for Atom {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(&format!(
            "<{}> <{}> '{}'",
            self.subject, self.property, self.value
        ))?;
        Ok(())
    }
}
