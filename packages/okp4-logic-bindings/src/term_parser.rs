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
            values.push(match self.peek() {
                None => Err(TermParseError::Eof),
                Some(t) if t == end_seq => {
                    if !values.is_empty() {
                        return Err(TermParseError::UnexpectedEndOfSeq);
                    }
                    self.eat_char();
                    break;
                }
                Some(b'[') => {
                    self.eat_char();
                    self.parse_array()
                }
                Some(b'(') => {
                    self.eat_char();
                    self.parse_tuple()
                }
                Some(b'\'') => {
                    self.eat_char();
                    self.parse_escaped_value()
                }
                Some(_) => self.parse_value(),
            }?);

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
        self.parse_seq(b']').map(TermValue::Array)
    }

    fn parse_tuple(&mut self) -> Result<TermValue, TermParseError> {
        self.parse_seq(b')')
            .and_then(|elem: Vec<TermValue>| {
                if elem.is_empty() {
                    return Err(TermParseError::EmptyTuple);
                }
                Ok(elem)
            })
            .map(TermValue::Tuple)
    }

    fn parse_value(&mut self) -> Result<TermValue, TermParseError> {
        let mut bytes: Vec<u8> = Vec::new();
        loop {
            match self.peek() {
                Some(t) if [b'[', b'(', b'\'', b'"', b' '].contains(&t) => {
                    Err(TermParseError::UnexpectedValueToken(char::from(t)))?;
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
            .map_err(TermParseError::NotUtf8Value)
            .map(TermValue::Value)
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
            .map_err(TermParseError::NotUtf8Value)
            .map(TermValue::Value)
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
            return Ok(values[0].clone());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_value() {
        let cases = vec![
            ("hello", Ok(TermValue::Value("hello".to_string()))),
            ("47", Ok(TermValue::Value("47".to_string()))),
            ("25.18", Ok(TermValue::Value("25.18".to_string()))),
            ("78/foo", Ok(TermValue::Value("78/foo".to_string()))),
            ("", Err(TermParseError::EmptyValue)),
            ("]", Err(TermParseError::EmptyValue)),
            (")", Err(TermParseError::EmptyValue)),
            (",", Err(TermParseError::EmptyValue)),
            ("foo[", Err(TermParseError::UnexpectedValueToken('['))),
            ("foo(", Err(TermParseError::UnexpectedValueToken('('))),
            ("foo'", Err(TermParseError::UnexpectedValueToken('\''))),
            ("foo\"", Err(TermParseError::UnexpectedValueToken('"'))),
            ("foo ", Err(TermParseError::UnexpectedValueToken(' '))),
        ];

        for case in cases {
            let res = Parser {
                slice: case.0.as_bytes(),
                index: 0,
            }
            .parse_value();
            assert_eq!(res, case.1);
        }

        let res = Parser {
            slice: &[255u8],
            index: 0,
        }
        .parse_value();
        assert!(res.is_err());
        matches!(res.err().unwrap(), TermParseError::NotUtf8Value(_));
    }

    #[test]
    fn parse_escaped_value() {
        let cases = vec![
            ("hello'", Ok(TermValue::Value("hello".to_string()))),
            (
                "47.18/\\foo&é#@'",
                Ok(TermValue::Value("47.18/\\foo&é#@".to_string())),
            ),
            (
                "can \\' quote'",
                Ok(TermValue::Value("can ' quote".to_string())),
            ),
            (
                "    a     '",
                Ok(TermValue::Value("    a     ".to_string())),
            ),
            (
                "and an emoji 👌'",
                Ok(TermValue::Value("and an emoji 👌".to_string())),
            ),
            ("eof", Err(TermParseError::Eof)),
        ];

        for case in cases {
            let res = Parser {
                slice: case.0.as_bytes(),
                index: 0,
            }
            .parse_escaped_value();
            assert_eq!(res, case.1);
        }

        let res = Parser {
            slice: &[255u8, b'\''],
            index: 0,
        }
        .parse_escaped_value();
        assert!(res.is_err());
        matches!(res.err().unwrap(), TermParseError::NotUtf8Value(_));
    }

    #[test]
    fn parse_array() {
        let cases = vec![
            ("[]", Ok(TermValue::Array(vec![]))),
            ("[,]", Err(TermParseError::EmptyValue)),
            ("[[]", Err(TermParseError::Eof)),
            ("[", Err(TermParseError::Eof)),
            (
                "[hello]",
                Ok(TermValue::Array(vec![TermValue::Value(
                    "hello".to_string(),
                )])),
            ),
            ("['hello'oups]", Err(TermParseError::ExpectedSeqToken('o'))),
            (
                "[hello,'cosmonaut']",
                Ok(TermValue::Array(vec![
                    TermValue::Value("hello".to_string()),
                    TermValue::Value("cosmonaut".to_string()),
                ])),
            ),
            (
                "[hello,'cosmonaut',]",
                Err(TermParseError::UnexpectedEndOfSeq),
            ),
            (
                "[hello, 'cosmonaut']",
                Err(TermParseError::UnexpectedValueToken(' ')),
            ),
            (
                "[[],[[]],['that\\'s a lot!']]",
                Ok(TermValue::Array(vec![
                    TermValue::Array(vec![]),
                    TermValue::Array(vec![TermValue::Array(vec![])]),
                    TermValue::Array(vec![TermValue::Value("that's a lot!".to_string())]),
                ])),
            ),
        ];

        for case in cases {
            let res = from_str(case.0);
            assert_eq!(res, case.1);
        }
    }

    #[test]
    fn parse_tuple() {
        let cases = vec![
            (
                "(1,2)",
                Ok(TermValue::Tuple(vec![
                    TermValue::Value("1".to_string()),
                    TermValue::Value("2".to_string()),
                ])),
            ),
            ("()", Err(TermParseError::EmptyTuple)),
            ("(,)", Err(TermParseError::EmptyValue)),
            ("((1,2)", Err(TermParseError::Eof)),
            ("(", Err(TermParseError::Eof)),
            (
                "(((1,2),(1,2,3)),('that\\'s',' a lot!'))",
                Ok(TermValue::Tuple(vec![
                    TermValue::Tuple(vec![
                        TermValue::Tuple(vec![
                            TermValue::Value("1".to_string()),
                            TermValue::Value("2".to_string()),
                        ]),
                        TermValue::Tuple(vec![
                            TermValue::Value("1".to_string()),
                            TermValue::Value("2".to_string()),
                            TermValue::Value("3".to_string()),
                        ]),
                    ]),
                    TermValue::Tuple(vec![
                        TermValue::Value("that's".to_string()),
                        TermValue::Value(" a lot!".to_string()),
                    ]),
                ])),
            ),
        ];

        for case in cases {
            let res = from_str(case.0);
            assert_eq!(res, case.1);
        }
    }

    #[test]
    fn parse() {
        let cases = vec![
            ("", Ok(TermValue::Value("".to_string()))),
            ("hello", Ok(TermValue::Value("hello".to_string()))),
            ("'hello'", Ok(TermValue::Value("hello".to_string()))),
            ("(1,2))", Err(TermParseError::UnexpectedRootToken(')'))),
            ("[]]", Err(TermParseError::UnexpectedRootToken(']'))),
            (
                "[hello],([[],'an \\' escape'],'an emoji 👌'),[cosmos]",
                Ok(TermValue::Tuple(vec![
                    TermValue::Array(vec![TermValue::Value("hello".to_string())]),
                    TermValue::Tuple(vec![
                        TermValue::Array(vec![
                            TermValue::Array(vec![]),
                            TermValue::Value("an \' escape".to_string()),
                        ]),
                        TermValue::Value("an emoji 👌".to_string()),
                    ]),
                    TermValue::Array(vec![TermValue::Value("cosmos".to_string())]),
                ])),
            ),
        ];

        for case in cases {
            let res = from_str(case.0);
            assert_eq!(res, case.1);
        }
    }
}
