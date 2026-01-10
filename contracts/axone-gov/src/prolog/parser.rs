use std::ops::Range;

use crate::prolog::ast::Term;
use crate::prolog::lexer::{Kind, LexErrorKind, Tok};
use cosmwasm_std::{Int128, Int64, SignedDecimal};
use logos::Logos;

#[derive(Clone, Debug)]
pub struct Token<'a> {
    pub tok: Tok<'a>,
    pub span: Range<usize>,
}

#[derive(Debug)]
pub struct ParseError {
    pub msg: String,
    pub at: usize,
}

impl ParseError {
    fn new(msg: impl Into<String>, at: usize) -> Self {
        Self {
            msg: msg.into(),
            at,
        }
    }
}

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    pos: usize,
    input_len: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Result<Self, ParseError> {
        let mut lex = Tok::lexer(input);
        let mut tokens = Vec::new();

        while let Some(res) = lex.next() {
            if let Ok(tok) = res {
                tokens.push(Token {
                    tok,
                    span: lex.span(),
                });
            } else {
                let at = lex.span().start;
                let msg = match lex.extras.last_error.take() {
                    Some(LexErrorKind::UnterminatedQuotedAtom) => {
                        "unterminated quoted atom".to_string()
                    }
                    Some(LexErrorKind::InvalidEscapeInQuotedAtom) => {
                        "invalid escape in quoted atom".to_string()
                    }
                    None => "lex error".to_string(),
                };
                return Err(ParseError::new(msg, at));
            }
        }

        Ok(Self {
            tokens,
            pos: 0,
            input_len: input.len(),
        })
    }

    fn peek(&self) -> Kind {
        self.tokens
            .get(self.pos)
            .map_or(Kind::Eof, |t| t.tok.kind())
    }

    fn peek_tok(&self) -> Option<&Tok<'a>> {
        self.tokens.get(self.pos).map(|t| &t.tok)
    }

    fn bump(&mut self) -> Option<Token<'a>> {
        let t = self.tokens.get(self.pos).cloned();
        if t.is_some() {
            self.pos += 1;
        }
        t
    }

    fn expect(&mut self, k: Kind) -> Result<(), ParseError> {
        if self.peek() == k {
            self.bump();
            Ok(())
        } else {
            Err(ParseError::new(
                format!("expected {:?}, got {:?}", k, self.peek()),
                self.at(),
            ))
        }
    }

    fn at(&self) -> usize {
        self.tokens
            .get(self.pos)
            .map_or(self.input_len, |t| t.span.start)
    }

    fn is_stop(&self, stops: &[Kind]) -> bool {
        let k = self.peek();
        k == Kind::Eof || stops.iter().any(|s| *s == k)
    }

    pub fn parse_root(mut self) -> Result<Term, ParseError> {
        let t = self.parse_expr(0, &[])?;
        if self.peek() != Kind::Eof {
            return Err(ParseError::new("unexpected trailing tokens", self.at()));
        }
        Ok(t)
    }

    fn parse_expr(&mut self, min_bp: u8, stops: &[Kind]) -> Result<Term, ParseError> {
        if self.is_stop(stops) {
            return Err(ParseError::new("unexpected end of expression", self.at()));
        }

        // prefix
        let mut lhs = match self.peek() {
            Kind::Not | Kind::Plus | Kind::Minus => {
                let op = self
                    .bump()
                    .ok_or_else(|| ParseError::new("expected prefix operator", self.at()))?
                    .tok;

                if self.peek() == Kind::Eof || self.is_stop(stops) {
                    return Ok(Term::Atom(prefix_name(&op)));
                }

                let rhs = self.parse_expr(8, stops)?; // prefix binds tighter than everything here

                match op {
                    Tok::Plus => match rhs {
                        Term::Integer(_) | Term::Float(_) => rhs,
                        _ => Term::Compound("+".to_string(), vec![rhs]),
                    },
                    Tok::Minus => match rhs {
                        Term::Integer(i) => {
                            let v = i.i64();
                            let nv = v.checked_neg().ok_or_else(|| {
                                ParseError::new("integer negation overflow", self.at())
                            })?;
                            Term::Integer(Int64::from(nv))
                        }
                        Term::Float(d) => {
                            let atoms = d.atomics().i128();
                            let natoms = atoms.checked_neg().ok_or_else(|| {
                                ParseError::new("float negation overflow", self.at())
                            })?;
                            Term::Float(SignedDecimal::new(Int128::new(natoms)))
                        }
                        _ => Term::Compound("-".to_string(), vec![rhs]),
                    },
                    Tok::Not => Term::Compound("\\+".to_string(), vec![rhs]),
                    _ => Term::Compound(prefix_name(&op), vec![rhs]),
                }
            }
            _ => self.parse_primary(stops)?,
        };

        // infix loop
        loop {
            if self.is_stop(stops) {
                break;
            }

            let k = self.peek();
            let Some((bp, assoc, name)) = infix_bp(k) else {
                break;
            };
            if bp < min_bp {
                break;
            }

            self.bump(); // consume operator

            let next_min = match assoc {
                Assoc::Left => bp + 1,
                Assoc::Right => bp,
            };

            let rhs = self.parse_expr(next_min, stops)?;
            lhs = Term::Compound(name.to_string(), vec![lhs, rhs]);
        }

        Ok(lhs)
    }

    fn parse_primary(&mut self, stops: &[Kind]) -> Result<Term, ParseError> {
        if self.is_stop(stops) {
            return Err(ParseError::new("unexpected stop token", self.at()));
        }

        match self.peek_tok().cloned() {
            Some(Tok::LParen) => {
                self.bump();
                let t = self.parse_expr(0, &[Kind::RParen])?;
                self.expect(Kind::RParen)?;
                Ok(t)
            }
            Some(Tok::LBrace) => {
                self.bump(); // consume '{'
                if self.peek() == Kind::RBrace {
                    self.bump(); // consume '}'
                    Ok(Term::Atom("{}".to_string()))
                } else {
                    Err(ParseError::new(
                        "unexpected '{' (only {} is supported as an atom)",
                        self.at(),
                    ))
                }
            }
            Some(Tok::LBrack) => self.parse_list(),
            Some(Tok::Atom(_)) | Some(Tok::QuotedAtom(_)) | Some(Tok::Dot) => {
                // could be Atom OR functor (compound) OR dict tag
                self.parse_atom_like()
            }
            Some(Tok::Var(v)) => {
                self.bump();
                Ok(Term::Variable(v.to_string()))
            }
            Some(Tok::Int(s)) => {
                self.bump();
                Ok(Term::Integer(
                    parse_i64(s).map_err(|e| ParseError::new(e, self.at()))?,
                ))
            }
            Some(Tok::BasedInt(s)) => {
                self.bump();
                Ok(Term::Integer(
                    parse_based_i64(s).map_err(|e| ParseError::new(e, self.at()))?,
                ))
            }
            Some(Tok::CharCode(s)) => {
                self.bump();
                Ok(Term::Integer(
                    parse_char_code_i64(s).map_err(|e| ParseError::new(e, self.at()))?,
                ))
            }

            Some(Tok::Float(s)) => {
                self.bump();
                Ok(Term::Float(
                    parse_signed_decimal_scientific(s)
                        .map_err(|e| ParseError::new(e, self.at()))?,
                ))
            }
            Some(tok) => {
                let name = match tok {
                    Tok::Eq => "=",
                    Tok::Neq => "\\=",
                    Tok::EqNeq => "=\\=",
                    Tok::Le => "=<",
                    Tok::Ge => ">=",
                    Tok::Lt => "<",
                    Tok::Gt => ">",
                    Tok::Assign => ":=",
                    Tok::Plus => "+",
                    Tok::Minus => "-",
                    Tok::Mul => "*",
                    Tok::Div => "/",
                    Tok::Pow => "**",
                    Tok::Comma => ",",
                    Tok::Semi => ";",
                    Tok::Colon => ":",
                    Tok::Bar => "|",
                    Tok::Is => "is",
                    Tok::Mod => "mod",
                    _ => {
                        return Err(ParseError::new(
                            format!("unexpected token {:?}", self.peek()),
                            self.at(),
                        ))
                    }
                };

                self.bump();
                Ok(Term::Atom(name.to_string()))
            }
            _ => Err(ParseError::new(
                format!("unexpected token {:?}", self.peek()),
                self.at(),
            )),
        }
    }

    fn parse_atom_like(&mut self) -> Result<Term, ParseError> {
        // consume atom token as String
        let tok = self
            .bump()
            .ok_or_else(|| ParseError::new("expected atom or quoted atom", self.at()))?;
        let name = match tok.tok {
            Tok::Atom(a) => a.to_string(),
            Tok::QuotedAtom(s) => s,
            Tok::Dot => ".".to_string(),
            _ => return Err(ParseError::new("expected atom or quoted atom", self.at())),
        };

        match self.peek() {
            Kind::LParen => {
                self.bump(); // (
                let mut args = Vec::new();

                if self.peek() != Kind::RParen {
                    loop {
                        // args: stop at ',' or ')'
                        let arg = self.parse_expr(0, &[Kind::Comma, Kind::RParen])?;
                        args.push(arg);

                        if self.peek() == Kind::Comma {
                            self.bump();
                            continue;
                        }
                        break;
                    }
                }

                self.expect(Kind::RParen)?;
                Ok(Term::Compound(name, args))
            }

            Kind::LBrace => {
                // dict: tag{ k:v, ... }
                self.parse_dict(name)
            }

            _ => Ok(Term::Atom(name)),
        }
    }

    fn parse_list(&mut self) -> Result<Term, ParseError> {
        self.expect(Kind::LBrack)?;
        let mut items = Vec::new();
        let mut tail: Option<Box<Term>> = None;

        if self.peek() != Kind::RBrack {
            loop {
                // item expression: stop at ',', '|' or ']'
                let item = self.parse_expr(0, &[Kind::Comma, Kind::Bar, Kind::RBrack])?;
                items.push(item);

                match self.peek() {
                    Kind::Comma => {
                        self.bump();
                        continue;
                    }
                    Kind::Bar => {
                        self.bump();
                        let t = self.parse_expr(0, &[Kind::RBrack])?;
                        tail = Some(Box::new(t));
                        break;
                    }
                    _ => break,
                }
            }
        }

        self.expect(Kind::RBrack)?;
        Ok(Term::List(items, tail))
    }

    fn parse_dict(&mut self, tag: String) -> Result<Term, ParseError> {
        self.expect(Kind::LBrace)?;
        let mut entries: Vec<(String, Term)> = Vec::new();

        if self.peek() != Kind::RBrace {
            loop {
                // key must be atom-like
                let key = match self
                    .bump()
                    .ok_or(ParseError::new("expected dict key", self.at()))?
                    .tok
                {
                    Tok::Atom(a) => a.to_string(),
                    Tok::QuotedAtom(s) => s,
                    _ => return Err(ParseError::new("dict key must be atom", self.at())),
                };

                self.expect(Kind::Colon)?;

                let val = self.parse_expr(0, &[Kind::Comma, Kind::RBrace])?;
                entries.push((key, val));

                if self.peek() == Kind::Comma {
                    self.bump();
                    continue;
                }
                break;
            }
        }

        self.expect(Kind::RBrace)?;
        Ok(Term::Dict(tag, entries))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Assoc {
    Left,
    Right,
}

fn infix_bp(k: Kind) -> Option<(u8, Assoc, &'static str)> {
    use Assoc::*;
    Some(match k {
        Kind::Semi => (1, Right, ";"),
        Kind::Comma => (2, Right, ","),
        Kind::Assign => (3, Right, ":="),

        Kind::Is => (4, Left, "is"),
        Kind::Eq | Kind::Neq | Kind::EqNeq | Kind::Le | Kind::Ge | Kind::Lt | Kind::Gt => {
            (4, Left, op_str(k))
        }

        Kind::Plus | Kind::Minus => (5, Left, op_str(k)),
        Kind::Mul | Kind::Div | Kind::Mod => (6, Left, op_str(k)),

        Kind::Pow => (7, Right, "**"),
        _ => return None,
    })
}

fn op_str(k: Kind) -> &'static str {
    match k {
        Kind::Eq => "=",
        Kind::Neq => "\\=",
        Kind::EqNeq => "=\\=",
        Kind::Le => "=<",
        Kind::Ge => ">=",
        Kind::Lt => "<",
        Kind::Gt => ">",
        Kind::Plus => "+",
        Kind::Minus => "-",
        Kind::Mul => "*",
        Kind::Div => "/",
        Kind::Mod => "mod",
        _ => "?",
    }
}

fn prefix_name(tok: &Tok<'_>) -> String {
    match tok {
        Tok::Not => "\\+".to_string(),
        Tok::Plus => "+".to_string(),
        Tok::Minus => "-".to_string(),
        _ => "?".to_string(),
    }
}

fn parse_i64(s: &str) -> Result<Int64, String> {
    let v = s.parse::<i64>().map_err(|_| format!("invalid int: {s}"))?;
    Ok(Int64::from(v))
}

fn parse_based_i64(s: &str) -> Result<Int64, String> {
    let (neg, body) = if let Some(rest) = s.strip_prefix('-') {
        (true, rest)
    } else if let Some(rest) = s.strip_prefix('+') {
        (false, rest)
    } else {
        (false, s)
    };

    let (radix, digits) = if let Some(rest) = body.strip_prefix("0b") {
        (2u32, rest)
    } else if let Some(rest) = body.strip_prefix("0o") {
        (8u32, rest)
    } else if let Some(rest) = body.strip_prefix("0x") {
        (16u32, rest)
    } else {
        return Err(format!("invalid based int: {s}"));
    };

    if digits.is_empty() {
        return Err(format!("invalid based int: {s}"));
    }

    let u = i128::from_str_radix(digits, radix).map_err(|_| format!("invalid based int: {s}"))?;
    let v = if neg { -u } else { u };

    if v < i64::MIN as i128 || v > i64::MAX as i128 {
        return Err(format!("based int out of range: {s}"));
    }

    Ok(Int64::from(v as i64))
}

fn parse_char_code_i64(s: &str) -> Result<Int64, String> {
    let rest = s
        .strip_prefix("0'")
        .ok_or_else(|| format!("invalid char code: {s}"))?;

    if rest.is_empty() {
        return Err(format!("invalid char code: {s}"));
    }

    // 0'\x41  or 0'\x41\
    if let Some(esc) = rest.strip_prefix("\\x") {
        let hex = if let Some(h) = esc.strip_suffix('\\') {
            h
        } else {
            esc
        };
        if hex.is_empty() || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(format!("invalid hex char code: {s}"));
        }
        let v = u32::from_str_radix(hex, 16).map_err(|_| format!("invalid hex char code: {s}"))?;
        return Ok(Int64::from(v as i64));
    }

    // 0'\101 or 0'\101\
    if let Some(esc) = rest.strip_prefix('\\') {
        // common single-char escapes
        let mut chars = esc.chars();
        let Some(c0) = chars.next() else {
            return Err(format!("invalid char code: {s}"));
        };

        match c0 {
            'a' => return Ok(Int64::from(0x07)),
            'b' => return Ok(Int64::from(0x08)),
            'f' => return Ok(Int64::from(0x0c)),
            'n' => return Ok(Int64::from(0x0a)),
            'r' => return Ok(Int64::from(0x0d)),
            't' => return Ok(Int64::from(0x09)),
            'v' => return Ok(Int64::from(0x0b)),
            '\\' => return Ok(Int64::from('\\' as i64)),
            '\'' => return Ok(Int64::from('\'' as i64)),
            '"' => return Ok(Int64::from('"' as i64)),
            _ => {}
        }

        // octal escape: up to 3 digits, optional trailing backslash
        if c0.is_ascii_digit() {
            let mut oct = String::new();
            oct.push(c0);
            for _ in 0..2 {
                if let Some(c) = chars.clone().next() {
                    if c.is_ascii_digit() {
                        oct.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
            }

            let tail: String = chars.collect();
            let tail = tail.as_str();
            let tail = if tail == "\\" { "" } else { tail };
            if !tail.is_empty() {
                return Err(format!("invalid octal char code: {s}"));
            }

            let v = u32::from_str_radix(&oct, 8)
                .map_err(|_| format!("invalid octal char code: {s}"))?;
            return Ok(Int64::from(v as i64));
        }

        // fallback: treat as escaped single char
        return Ok(Int64::from(c0 as i64));
    }

    // default: first unicode scalar
    let ch = rest
        .chars()
        .next()
        .ok_or_else(|| format!("invalid char code: {s}"))?;
    Ok(Int64::from(ch as i64))
}

fn parse_signed_decimal_scientific(s: &str) -> Result<SignedDecimal, String> {
    let s = s.trim();
    let (neg, s) = if let Some(rest) = s.strip_prefix('-') {
        (true, rest)
    } else if let Some(rest) = s.strip_prefix('+') {
        (false, rest)
    } else {
        (false, s)
    };

    let (mantissa, exp_part) = match s.find(['e', 'E']) {
        Some(i) => (&s[..i], Some(&s[i + 1..])),
        None => (s, None),
    };

    let dot = mantissa
        .find('.')
        .ok_or_else(|| format!("float missing '.': {s}"))?;
    let int_part = &mantissa[..dot];
    let frac_part = &mantissa[dot + 1..];

    if int_part.is_empty() || frac_part.is_empty() {
        return Err(format!("invalid float mantissa: {s}"));
    }
    if !int_part.chars().all(|c| c.is_ascii_digit())
        || !frac_part.chars().all(|c| c.is_ascii_digit())
    {
        return Err(format!("invalid float digits: {s}"));
    }

    let exp: i32 = match exp_part {
        Some(e) => e
            .parse::<i32>()
            .map_err(|_| format!("invalid float exponent: {s}"))?,
        None => 0,
    };

    let frac_len: i32 = frac_part
        .len()
        .try_into()
        .map_err(|_| format!("float too precise: {s}"))?;

    // value = digits * 10^(exp - frac_len)
    let exp10 = exp - frac_len;

    // convert to 18-decimal atomics: atomics = digits * 10^(exp10 + 18)
    let pow10 = exp10
        .checked_add(18)
        .ok_or_else(|| format!("float exponent out of range: {s}"))?;

    let mut digits = String::with_capacity(int_part.len() + frac_part.len());
    digits.push_str(int_part);
    digits.push_str(frac_part);

    let mut n = digits
        .parse::<i128>()
        .map_err(|_| format!("float mantissa out of range: {s}"))?;
    if neg {
        n = -n;
    }

    let atomics: i128 = if pow10 >= 0 {
        let m = pow10_i128(pow10 as u32)?;
        n.checked_mul(m)
            .ok_or_else(|| format!("float out of range: {s}"))?
    } else {
        let d = pow10_i128((-pow10) as u32)?;
        if n % d != 0 {
            return Err(format!("float has more than 18 decimal places: {s}"));
        }
        n / d
    };

    SignedDecimal::from_atomics(Int128::new(atomics), 18)
        .map_err(|_| format!("float out of range: {s}"))
}

fn pow10_i128(exp: u32) -> Result<i128, String> {
    let mut acc: i128 = 1;
    for _ in 0..exp {
        acc = acc
            .checked_mul(10)
            .ok_or_else(|| "power of 10 overflow".to_string())?;
    }
    Ok(acc)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_numbers() {
        use super::*;

        let cases = vec![
            ("123", Term::Integer(Int64::new(123))),
            ("+456", Term::Integer(Int64::new(456))),
            ("-789", Term::Integer(Int64::new(-789))),
            ("0b1010", Term::Integer(Int64::new(10))),
            ("-0b1010", Term::Integer(Int64::new(-10))),
            ("0o17", Term::Integer(Int64::new(15))),
            ("+0o17", Term::Integer(Int64::new(15))),
            ("0x1A", Term::Integer(Int64::new(26))),
            ("-0x1A", Term::Integer(Int64::new(-26))),
            (r"0'\x41\", Term::Integer(Int64::new(65))),
            (r"0'\101\", Term::Integer(Int64::new(65))),
            (r"0'\n", Term::Integer(Int64::new(10))),
            (
                "3.14",
                Term::Float(
                    SignedDecimal::from_atomics(Int128::new(3140000000000000000), 18).unwrap(),
                ),
            ),
            (
                "-0.001",
                Term::Float(
                    SignedDecimal::from_atomics(Int128::new(-1000000000000000), 18).unwrap(),
                ),
            ),
            (
                "2.5e3",
                Term::Float(
                    SignedDecimal::from_atomics(Int128::new(2500000000000000000000), 18).unwrap(),
                ),
            ),
            (
                "-1.2E-2",
                Term::Float(
                    SignedDecimal::from_atomics(Int128::new(-12000000000000000), 18).unwrap(),
                ),
            ),
        ];

        let parse = |s: &str| Parser::new(s).and_then(|p| p.parse_root());

        for (input, expected) in cases {
            let term = parse(input).expect(&format!("failed to parse {input}"));
            assert_eq!(term, expected, "input: {input}");
        }
    }

    #[test]
    fn test_atoms() {
        use super::*;

        let cases = vec![
            // plain atoms (unquoted)
            ("foo", Term::Atom("foo".to_string())),
            ("a", Term::Atom("a".to_string())),
            ("void", Term::Atom("void".to_string())),
            ("a1", Term::Atom("a1".to_string())),
            ("foo_bar123", Term::Atom("foo_bar123".to_string())),
            // quoted atoms (needed for uppercase, spaces, punctuation)
            ("'Bar-Baz'", Term::Atom("Bar-Baz".to_string())),
            ("'X'", Term::Atom("X".to_string())),
            ("'foo bar'", Term::Atom("foo bar".to_string())),
            ("''", Term::Atom("".to_string())),
            // special atoms
            ("'[]'", Term::Atom("[]".to_string())),
            ("{}", Term::Atom("{}".to_string())),
            ("'{}'", Term::Atom("{}".to_string())),
            // operator / punctuation atoms
            ("=", Term::Atom("=".to_string())),
            ("'='", Term::Atom("=".to_string())),
            (r"\=", Term::Atom("\\=".to_string())),
            (r"'=\\='", Term::Atom("=\\=".to_string())),
            ("=<", Term::Atom("=<".to_string())),
            ("'>='", Term::Atom(">=".to_string())),
            ("<", Term::Atom("<".to_string())),
            (">", Term::Atom(">".to_string())),
            (":=", Term::Atom(":=".to_string())),
            ("+", Term::Atom("+".to_string())),
            ("-", Term::Atom("-".to_string())),
            ("*", Term::Atom("*".to_string())),
            ("/", Term::Atom("/".to_string())),
            ("**", Term::Atom("**".to_string())),
            (",", Term::Atom(",".to_string())),
            (";", Term::Atom(";".to_string())),
            (":", Term::Atom(":".to_string())),
            ("|", Term::Atom("|".to_string())),
            ("is", Term::Atom("is".to_string())),
            ("mod", Term::Atom("mod".to_string())),
            (r"\+", Term::Atom("\\+".to_string())),
            // quoted punctuation atoms (canonical Prolog-safe forms)
            ("','", Term::Atom(",".to_string())),
            ("';'", Term::Atom(";".to_string())),
            ("'|'", Term::Atom("|".to_string())),
            ("'-'", Term::Atom("-".to_string())),
            ("'**'", Term::Atom("**".to_string())),
            // escapes inside quoted atoms (ISO-ish)
            (r"'can \' quote'", Term::Atom("can ' quote".to_string())),
            (
                r"'and an emoji 👌'",
                Term::Atom("and an emoji 👌".to_string()),
            ),
            (r"'line\nfeed'", Term::Atom("line\nfeed".to_string())),
            (r"'\x41\'", Term::Atom("A".to_string())),
            (r"'\101\'", Term::Atom("A".to_string())),
        ];

        let parse = |s: &str| Parser::new(s).and_then(|p| p.parse_root());

        for (input, expected) in cases {
            let term = parse(input).expect(&format!("failed to parse {input}"));
            assert_eq!(term, expected, "input: {input}");
        }
    }

    #[test]
    fn test_variables() {
        use super::*;

        let cases = vec![
            ("X", Term::Variable("X".to_string())),
            ("_RESULT", Term::Variable("_RESULT".to_string())),
            ("_", Term::Variable("_".to_string())),
        ];

        let parse = |s: &str| Parser::new(s).and_then(|p| p.parse_root());

        for (input, expected) in cases {
            let term = parse(input).expect(&format!("failed to parse {input}"));
            assert_eq!(term, expected, "input: {input}");
        }
    }

    #[test]
    fn test_compounds() {
        use super::*;

        let cases = vec![
            (
                "point(X,Y,Z)",
                Term::Compound(
                    "point".to_string(),
                    vec![
                        Term::Variable("X".to_string()),
                        Term::Variable("Y".to_string()),
                        Term::Variable("Z".to_string()),
                    ],
                ),
            ),
            (
                "foo(bar,baz)",
                Term::Compound(
                    "foo".to_string(),
                    vec![Term::Atom("bar".to_string()), Term::Atom("baz".to_string())],
                ),
            ),
            (
                "foo(bar(baz),qux)",
                Term::Compound(
                    "foo".to_string(),
                    vec![
                        Term::Compound("bar".to_string(), vec![Term::Atom("baz".to_string())]),
                        Term::Atom("qux".to_string()),
                    ],
                ),
            ),
            (
                r"'Algol-68'(X)",
                Term::Compound(
                    "Algol-68".to_string(),
                    vec![Term::Variable("X".to_string())],
                ),
            ),
            (
                r"'='(X,Y)",
                Term::Compound(
                    "=".to_string(),
                    vec![
                        Term::Variable("X".to_string()),
                        Term::Variable("Y".to_string()),
                    ],
                ),
            ),
            // prefix operators as compounds on non-numbers
            (
                "-X",
                Term::Compound("-".to_string(), vec![Term::Variable("X".to_string())]),
            ),
            (
                r"\+X",
                Term::Compound("\\+".to_string(), vec![Term::Variable("X".to_string())]),
            ),
            // precedence: * binds tighter than +
            (
                "X+Y*Z",
                Term::Compound(
                    "+".to_string(),
                    vec![
                        Term::Variable("X".to_string()),
                        Term::Compound(
                            "*".to_string(),
                            vec![
                                Term::Variable("Y".to_string()),
                                Term::Variable("Z".to_string()),
                            ],
                        ),
                    ],
                ),
            ),
            // right-assoc power: X ** (Y ** Z)
            (
                "X**Y**Z",
                Term::Compound(
                    "**".to_string(),
                    vec![
                        Term::Variable("X".to_string()),
                        Term::Compound(
                            "**".to_string(),
                            vec![
                                Term::Variable("Y".to_string()),
                                Term::Variable("Z".to_string()),
                            ],
                        ),
                    ],
                ),
            ),
            // parentheses override
            (
                "(P;Q),R",
                Term::Compound(
                    ",".to_string(),
                    vec![
                        Term::Compound(
                            ";".to_string(),
                            vec![
                                Term::Variable("P".to_string()),
                                Term::Variable("Q".to_string()),
                            ],
                        ),
                        Term::Variable("R".to_string()),
                    ],
                ),
            ),
            // precedence between ; and ,
            (
                "P;Q,R",
                Term::Compound(
                    ";".to_string(),
                    vec![
                        Term::Variable("P".to_string()),
                        Term::Compound(
                            ",".to_string(),
                            vec![
                                Term::Variable("Q".to_string()),
                                Term::Variable("R".to_string()),
                            ],
                        ),
                    ],
                ),
            ),
            // compound with list argument (including tail)
            (
                "f([a,b|L])",
                Term::Compound(
                    "f".to_string(),
                    vec![Term::List(
                        vec![Term::Atom("a".to_string()), Term::Atom("b".to_string())],
                        Some(Box::new(Term::Variable("L".to_string()))),
                    )],
                ),
            ),
        ];

        let parse = |s: &str| Parser::new(s).and_then(|p| p.parse_root());

        for (input, expected) in cases {
            let term = parse(input).expect(&format!("failed to parse {input}"));
            assert_eq!(term, expected, "input: {input}");
        }
    }

    #[test]
    fn test_lists() {
        use super::*;

        let cases = vec![
            // empty list
            ("[]", Term::List(vec![], None)),
            // simple list
            (
                "[a,b,c]",
                Term::List(
                    vec![
                        Term::Atom("a".to_string()),
                        Term::Atom("b".to_string()),
                        Term::Atom("c".to_string()),
                    ],
                    None,
                ),
            ),
            // list with variables
            (
                "[X,Y,Z]",
                Term::List(
                    vec![
                        Term::Variable("X".to_string()),
                        Term::Variable("Y".to_string()),
                        Term::Variable("Z".to_string()),
                    ],
                    None,
                ),
            ),
            // tail notation
            (
                "[H|T]",
                Term::List(
                    vec![Term::Variable("H".to_string())],
                    Some(Box::new(Term::Variable("T".to_string()))),
                ),
            ),
            (
                "[a,b|L]",
                Term::List(
                    vec![Term::Atom("a".to_string()), Term::Atom("b".to_string())],
                    Some(Box::new(Term::Variable("L".to_string()))),
                ),
            ),
            // nested lists
            (
                "[[a],[],[b,c]]",
                Term::List(
                    vec![
                        Term::List(vec![Term::Atom("a".to_string())], None),
                        Term::List(vec![], None),
                        Term::List(
                            vec![Term::Atom("b".to_string()), Term::Atom("c".to_string())],
                            None,
                        ),
                    ],
                    None,
                ),
            ),
            // list containing compounds
            (
                "[point(X,Y),foo(bar),baz]",
                Term::List(
                    vec![
                        Term::Compound(
                            "point".to_string(),
                            vec![
                                Term::Variable("X".to_string()),
                                Term::Variable("Y".to_string()),
                            ],
                        ),
                        Term::Compound("foo".to_string(), vec![Term::Atom("bar".to_string())]),
                        Term::Atom("baz".to_string()),
                    ],
                    None,
                ),
            ),
            // list containing operator expressions
            (
                "[X+Y, -Z, \\+P]",
                Term::List(
                    vec![
                        Term::Compound(
                            "+".to_string(),
                            vec![
                                Term::Variable("X".to_string()),
                                Term::Variable("Y".to_string()),
                            ],
                        ),
                        Term::Compound("-".to_string(), vec![Term::Variable("Z".to_string())]),
                        Term::Compound("\\+".to_string(), vec![Term::Variable("P".to_string())]),
                    ],
                    None,
                ),
            ),
            // list with a complex tail
            (
                "[a|[b,c]]",
                Term::List(
                    vec![Term::Atom("a".to_string())],
                    Some(Box::new(Term::List(
                        vec![Term::Atom("b".to_string()), Term::Atom("c".to_string())],
                        None,
                    ))),
                ),
            ),
            // canonical cons form (should parse as a normal compound)
            (
                ".(1,.(2,.(3,[])))",
                Term::Compound(
                    ".".to_string(),
                    vec![
                        Term::Integer(Int64::from(1)),
                        Term::Compound(
                            ".".to_string(),
                            vec![
                                Term::Integer(Int64::from(2)),
                                Term::Compound(
                                    ".".to_string(),
                                    vec![Term::Integer(Int64::from(3)), Term::List(vec![], None)],
                                ),
                            ],
                        ),
                    ],
                ),
            ),
        ];

        let parse = |s: &str| Parser::new(s).and_then(|p| p.parse_root());

        for (input, expected) in cases {
            let term = parse(input).expect(&format!("failed to parse {input}"));
            assert_eq!(term, expected, "input: {input}");
        }
    }

    #[test]
    fn test_dicts() {
        use super::*;

        let cases = vec![
            // empty dict
            ("tag{}", Term::Dict("tag".to_string(), vec![])),
            // single entry
            (
                "tag{k:1}",
                Term::Dict(
                    "tag".to_string(),
                    vec![("k".to_string(), Term::Integer(Int64::from(1)))],
                ),
            ),
            // multiple entries
            (
                "tag{k:1,v:2}",
                Term::Dict(
                    "tag".to_string(),
                    vec![
                        ("k".to_string(), Term::Integer(Int64::from(1))),
                        ("v".to_string(), Term::Integer(Int64::from(2))),
                    ],
                ),
            ),
            // quoted tag + quoted key
            (
                r"'my-tag'{'Algol-68':X}",
                Term::Dict(
                    "my-tag".to_string(),
                    vec![("Algol-68".to_string(), Term::Variable("X".to_string()))],
                ),
            ),
            // operator-like key must be quoted
            (
                r"tag{'=':X}",
                Term::Dict(
                    "tag".to_string(),
                    vec![("=".to_string(), Term::Variable("X".to_string()))],
                ),
            ),
            (
                r"tag{'=\\=':Y}",
                Term::Dict(
                    "tag".to_string(),
                    vec![("=\\=".to_string(), Term::Variable("Y".to_string()))],
                ),
            ),
            // values can be any term: list
            (
                "tag{xs:[a,b|L]}",
                Term::Dict(
                    "tag".to_string(),
                    vec![(
                        "xs".to_string(),
                        Term::List(
                            vec![Term::Atom("a".to_string()), Term::Atom("b".to_string())],
                            Some(Box::new(Term::Variable("L".to_string()))),
                        ),
                    )],
                ),
            ),
            // values can be compounds
            (
                "tag{p:point(X,Y)}",
                Term::Dict(
                    "tag".to_string(),
                    vec![(
                        "p".to_string(),
                        Term::Compound(
                            "point".to_string(),
                            vec![
                                Term::Variable("X".to_string()),
                                Term::Variable("Y".to_string()),
                            ],
                        ),
                    )],
                ),
            ),
            // values can be operator expressions
            (
                "tag{e:X+Y*Z}",
                Term::Dict(
                    "tag".to_string(),
                    vec![(
                        "e".to_string(),
                        Term::Compound(
                            "+".to_string(),
                            vec![
                                Term::Variable("X".to_string()),
                                Term::Compound(
                                    "*".to_string(),
                                    vec![
                                        Term::Variable("Y".to_string()),
                                        Term::Variable("Z".to_string()),
                                    ],
                                ),
                            ],
                        ),
                    )],
                ),
            ),
            // nested dict
            (
                "outer{inner:tag{k:1}}",
                Term::Dict(
                    "outer".to_string(),
                    vec![(
                        "inner".to_string(),
                        Term::Dict(
                            "tag".to_string(),
                            vec![("k".to_string(), Term::Integer(Int64::from(1)))],
                        ),
                    )],
                ),
            ),
        ];

        let parse = |s: &str| Parser::new(s).and_then(|p| p.parse_root());

        for (input, expected) in cases {
            let term = parse(input).expect(&format!("failed to parse {input}"));
            assert_eq!(term, expected, "input: {input}");
        }
    }

    #[test]
    fn test_pathological() {
        use super::*;

        let cases = vec![
            // deeply nested parentheses
            ("((((((((((a))))))))))", Term::Atom("a".to_string())),
            (
                "a+b*c",
                Term::Compound(
                    "+".to_string(),
                    vec![
                        Term::Atom("a".to_string()),
                        Term::Compound(
                            "*".to_string(),
                            vec![Term::Atom("b".to_string()), Term::Atom("c".to_string())],
                        ),
                    ],
                ),
            ),
        ];
        let parse = |s: &str| Parser::new(s).and_then(|p| p.parse_root());
        for (input, expected) in cases {
            let term = parse(input).expect(&format!("failed to parse {input}"));
            assert_eq!(term, expected, "input: {input}");
        }
    }

    #[test]
    fn test_parse_errors() {
        use super::*;

        let cases = vec![
            // unexpected end of input
            "",
            "foo(",
            "[a,b|",
            "tag{k:",
            // invalid tokens in certain contexts
            "'Unfinished atom",
            "0b102",
            "0o8",
            "0xG1",
            r"0'\xG1",
            r"0'\8",
            "3.14.15",
            "foo{key-with-dash:1}",
            // mismatched parentheses/brackets/braces
            "foo(bar",
            "[a,b,c",
            "tag{k:1,v:2",
        ];

        let parse = |s: &str| Parser::new(s).and_then(|p| p.parse_root());

        for input in cases {
            parse(input).expect_err(&format!("expected parse error for {input}"));
        }
    }
}
