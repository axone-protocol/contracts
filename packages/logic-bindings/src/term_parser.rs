use crate::error::TermParseError;

/// Represents a Prolog response term element which can be a tuple, an array or a string value.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TermValue {
    Tuple(Vec<TermValue>),
    Array(Vec<TermValue>),
    Value(String),
}

struct Parser<'a> {
    slice: &'a [u8],
    index: usize,
}

impl<'a> Parser<'a> {
    pub fn new(slice: &'a [u8]) -> Parser<'_> {
        Parser { slice, index: 0 }
    }

    fn peek(&mut self) -> Option<u8> {
        self.slice.get(self.index).cloned()
    }

    fn eat_char(&mut self) {
        self.index += 1;
    }

    fn parse_seq(&mut self, end_seq: u8) -> Result<Vec<TermValue>, TermParseError> {
        let mut values: Vec<TermValue> = Vec::new();
        loop {
            match self.peek() {
                None => Err(TermParseError::Eof)?,
                Some(b'[') => {
                    self.eat_char();
                    values.push(self.parse_array()?);
                }
                Some(b'(') => {
                    self.eat_char();
                    values.push(self.parse_tuple()?);
                }
                Some(b'\'') => {
                    self.eat_char();
                    values.push(self.parse_escaped_value()?);
                }
                Some(t) if t == end_seq => {
                    if !values.is_empty() {
                        Err(TermParseError::UnexpectedEndOfSeq)?
                    }
                    self.eat_char();
                    break;
                }
                Some(_) => values.push(self.parse_value()?),
            }
            match self.peek() {
                Some(t) if t == end_seq => {
                    self.eat_char();
                    break;
                }
                Some(b',') => {
                    self.eat_char();
                }
                Some(t) => Err(TermParseError::ExpectedSeqToken(char::from(t)))?,
                None => Err(TermParseError::Eof)?,
            }
        }
        Ok(values)
    }

    fn parse_array(&mut self) -> Result<TermValue, TermParseError> {
        self.parse_seq(b']').map(|elems| TermValue::Array(elems))
    }

    fn parse_tuple(&mut self) -> Result<TermValue, TermParseError> {
        self.parse_seq(b')')
            .and_then(|elem: Vec<TermValue>| {
                if elem.is_empty() {
                    return Err(TermParseError::EmptyTuple);
                }
                Ok(elem)
            })
            .map(|elems| TermValue::Tuple(elems))
    }

    fn parse_value(&mut self) -> Result<TermValue, TermParseError> {
        let mut bytes: Vec<u8> = Vec::new();
        loop {
            match self.peek() {
                Some(t) if [b'[', b'(', b'\'', b'"', b' '].contains(&t) => {
                    Err(TermParseError::UnexpectedValueToken(char::from(t)))?
                }
                Some(b) if ![b']', b')', b','].contains(&b) => {
                    self.eat_char();
                    bytes.push(b);
                }
                _ => break,
            }
        }

        if bytes.is_empty() {
            return Err(TermParseError::EmptyValue);
        }

        String::from_utf8(bytes)
            .map_err(|e| TermParseError::NotUtf8Value(e))
            .map(|str| TermValue::Value(str))
    }

    fn parse_escaped_value(&mut self) -> Result<TermValue, TermParseError> {
        let mut bytes: Vec<u8> = Vec::new();
        loop {
            match self.peek() {
                Some(b'\'') => {
                    self.eat_char();
                    break;
                }
                Some(b'\\') => {
                    self.eat_char();
                    match self.peek() {
                        Some(b'\'') => {
                            self.eat_char();
                            bytes.push(b'\'');
                        }
                        _ => {
                            bytes.push(b'\\');
                        }
                    }
                }
                Some(b) => {
                    self.eat_char();
                    bytes.push(b);
                }
                None => Err(TermParseError::Eof)?,
            }
        }

        String::from_utf8(bytes)
            .map_err(|e| TermParseError::NotUtf8Value(e))
            .map(|str| TermValue::Value(str))
    }

    fn parse(&mut self) -> Result<TermValue, TermParseError> {
        let mut values: Vec<TermValue> = Vec::new();
        loop {
            match self.peek() {
                Some(b'[') => {
                    self.eat_char();
                    values.push(self.parse_array()?);
                }
                Some(b'(') => {
                    self.eat_char();
                    values.push(self.parse_tuple()?);
                }
                Some(b'\'') => {
                    self.eat_char();
                    values.push(self.parse_escaped_value()?);
                }
                Some(_) => {
                    values.push(self.parse_value()?);
                }
                _ => {}
            }

            match self.peek() {
                Some(b',') => {
                    self.eat_char();
                }
                None => {
                    break;
                }
                Some(t) => Err(TermParseError::UnexpectedRootToken(char::from(t)))?,
            }
        }

        if values.is_empty() {
            return Ok(TermValue::Value(String::default()));
        }

        if values.len() == 1 {
            let val = values.first().unwrap();
            return Ok(val.clone());
        }

        Ok(TermValue::Tuple(values))
    }
}

/// Parses a Prolog response term from bytes
pub fn from_slice(v: &[u8]) -> Result<TermValue, TermParseError> {
    let mut parser = Parser::new(v);
    let value = parser.parse()?;

    Ok(value)
}

/// Parses a Prolog response term from a string
pub fn from_str(s: &str) -> Result<TermValue, TermParseError> {
    from_slice(s.as_bytes())
}
