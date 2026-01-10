use logos::Lexer;

use logos::Logos;

#[derive(Clone, Debug, Default)]
pub struct LexExtras {
    pub last_error: Option<LexErrorKind>,
}

#[derive(Clone, Debug)]
pub enum LexErrorKind {
    UnterminatedQuotedAtom,
    InvalidEscapeInQuotedAtom,
}

#[derive(Clone, Debug, Logos, PartialEq)]
#[logos(extras = LexExtras)]
pub enum Tok<'a> {
    // Skip
    #[regex(r"[ \t\r\n]+", logos::skip)]
    Ws,
    #[regex(r"%[^\n]*", logos::skip, allow_greedy = true)]
    Comment,

    // Punctuation
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBrack,
    #[token("]")]
    RBrack,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("|")]
    Bar,
    #[token(".")]
    Dot,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token(";")]
    Semi,

    // Operators (put longer first)
    #[token("=\\=")]
    EqNeq,
    #[token("\\=")]
    Neq,
    #[token("=<")]
    Le,
    #[token(">=")]
    Ge,
    #[token("**")]
    Pow,
    #[token(":=")]
    Assign,
    #[token("\\+")]
    Not,
    #[token("=")]
    Eq,
    #[token("<")]
    Lt,
    #[token(">")]
    Gt,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Mul,
    #[token("/")]
    Div,

    #[token("is")]
    Is,
    #[token("mod")]
    Mod,

    #[regex(r"[0-9]+\.[0-9]+([eE][+-]?[0-9]+)?", |lex| lex.slice())]
    Float(&'a str),

    #[regex(r"(0x[0-9A-Fa-f]+|0o[0-7]+|0b[01]+)", |lex| lex.slice())]
    BasedInt(&'a str),

    #[regex(r"[0-9]+", |lex| lex.slice())]
    Int(&'a str),

    #[regex(r#"0'(\\x[0-9A-Fa-f]+\\?|\\[0-7]{1,3}\\?|\\[abfnrtv\\\\\"' ]|.)"#, |lex| lex.slice())]
    CharCode(&'a str),

    #[regex(r"[A-Z_][A-Za-z0-9_]*", |lex| lex.slice())]
    Var(&'a str),

    #[regex(r"[a-z][A-Za-z0-9_]*", |lex| lex.slice())]
    Atom(&'a str),

    #[token("'", lex_quoted_atom)]
    QuotedAtom(String),
}

fn lex_quoted_atom<'s>(lex: &mut Lexer<'s, Tok<'s>>) -> Option<String> {
    let rem = lex.remainder();
    let mut out = String::new();

    let bytes = rem.as_bytes();
    let mut i: usize = 0;

    while i < bytes.len() {
        match bytes[i] {
            b'\'' => {
                // '' -> '
                if i + 1 < bytes.len() && bytes[i + 1] == b'\'' {
                    out.push('\'');
                    i += 2;
                    continue;
                }

                // quoted atom end
                let consumed = i + 1;
                lex.bump(consumed);
                return Some(out);
            }

            b'\\' => {
                i += 1;
                if i >= bytes.len() {
                    lex.extras.last_error = Some(LexErrorKind::InvalidEscapeInQuotedAtom);
                    return None; // backslash dangling
                }

                match bytes[i] {
                    b'\n' => {
                        // line continuation: skip both
                        i += 1;
                    }

                    b'\\' => {
                        out.push('\\');
                        i += 1;
                    }
                    b'\'' => {
                        out.push('\'');
                        i += 1;
                    }

                    b'a' => {
                        out.push('\x07');
                        i += 1;
                    }
                    b'b' => {
                        out.push('\x08');
                        i += 1;
                    }
                    b'f' => {
                        out.push('\x0c');
                        i += 1;
                    }
                    b'n' => {
                        out.push('\n');
                        i += 1;
                    }
                    b'r' => {
                        out.push('\r');
                        i += 1;
                    }
                    b't' => {
                        out.push('\t');
                        i += 1;
                    }
                    b'v' => {
                        out.push('\x0b');
                        i += 1;
                    }

                    b'x' => {
                        // \x<hexdigits>\
                        i += 1;

                        let start = i;
                        while i < bytes.len() && bytes[i].is_ascii_hexdigit() {
                            i += 1;
                        }
                        if start == i {
                            lex.extras.last_error = Some(LexErrorKind::InvalidEscapeInQuotedAtom);
                            return None; // no hexdigits
                        }
                        if i >= bytes.len() || bytes[i] != b'\\' {
                            lex.extras.last_error = Some(LexErrorKind::InvalidEscapeInQuotedAtom);
                            return None; // hex must end with '\'
                        }

                        let hex = &rem[start..i];
                        let val = u32::from_str_radix(hex, 16).ok()?;
                        let ch = char::from_u32(val)?;
                        out.push(ch);

                        i += 1; // consume the terminal backslash
                    }

                    d if d.is_ascii_digit() => {
                        // \<octal 1..3 digits>\
                        let start = i;
                        let mut count = 0;
                        while i < bytes.len() && count < 3 && bytes[i].is_ascii_digit() {
                            i += 1;
                            count += 1;
                        }
                        if i >= bytes.len() || bytes[i] != b'\\' {
                            lex.extras.last_error = Some(LexErrorKind::InvalidEscapeInQuotedAtom);
                            return None; // octal must end with '\'
                        }

                        let oct = &rem[start..i];
                        let val = u32::from_str_radix(oct, 8).ok()?;
                        let ch = char::from_u32(val)?;
                        out.push(ch);

                        i += 1; // consume the terminal backslash
                    }

                    other => {
                        // fallback: Prolog often lets the escape pass as is
                        // we just put the char after the backslash
                        out.push(other as char);
                        i += 1;
                    }
                }
            }

            _ => {
                // consume regular char
                let s = &rem[i..];
                let mut it = s.chars();
                let ch = it.next()?;
                out.push(ch);
                i += ch.len_utf8();
            }
        }
    }

    lex.extras.last_error = Some(LexErrorKind::UnterminatedQuotedAtom);
    None
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Kind {
    LParen,
    RParen,
    LBrack,
    RBrack,
    LBrace,
    RBrace,
    Bar,
    Colon,
    Comma,
    Semi,
    EqNeq,
    Neq,
    Le,
    Ge,
    Pow,
    Assign,
    Not,
    Eq,
    Lt,
    Gt,
    Plus,
    Minus,
    Mul,
    Div,
    Is,
    Mod,
    Float,
    BasedInt,
    Int,
    CharCode,
    Var,
    Atom,
    QuotedAtom,
    Eof,
}

impl<'a> Tok<'a> {
    pub fn kind(&self) -> Kind {
        match self {
            Tok::LParen => Kind::LParen,
            Tok::RParen => Kind::RParen,
            Tok::LBrack => Kind::LBrack,
            Tok::RBrack => Kind::RBrack,
            Tok::LBrace => Kind::LBrace,
            Tok::RBrace => Kind::RBrace,
            Tok::Bar => Kind::Bar,
            Tok::Colon => Kind::Colon,
            Tok::Comma => Kind::Comma,
            Tok::Semi => Kind::Semi,

            Tok::EqNeq => Kind::EqNeq,
            Tok::Neq => Kind::Neq,
            Tok::Le => Kind::Le,
            Tok::Ge => Kind::Ge,
            Tok::Pow => Kind::Pow,
            Tok::Assign => Kind::Assign,
            Tok::Not => Kind::Not,
            Tok::Eq => Kind::Eq,
            Tok::Lt => Kind::Lt,
            Tok::Gt => Kind::Gt,
            Tok::Plus => Kind::Plus,
            Tok::Minus => Kind::Minus,
            Tok::Mul => Kind::Mul,
            Tok::Div => Kind::Div,

            Tok::Is => Kind::Is,
            Tok::Mod => Kind::Mod,

            Tok::Float(_) => Kind::Float,
            Tok::BasedInt(_) => Kind::BasedInt,
            Tok::Int(_) => Kind::Int,
            Tok::CharCode(_) => Kind::CharCode,
            Tok::Var(_) => Kind::Var,
            Tok::Dot | Tok::Atom(_) => Kind::Atom,
            Tok::QuotedAtom(_) => Kind::QuotedAtom,

            Tok::Ws | Tok::Comment => unreachable!("skipped by logos"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn toks(input: &str) -> Vec<Tok<'_>> {
        Tok::lexer(input).filter_map(|r| r.ok()).collect()
    }

    #[test]
    fn quoted_atom_simple() {
        assert_eq!(toks("'hello'"), vec![Tok::QuotedAtom("hello".to_string())]);
    }

    #[test]
    fn quoted_atom_doubled_quote() {
        assert_eq!(toks("'can''t'"), vec![Tok::QuotedAtom("can't".to_string())]);
    }

    #[test]
    fn quoted_atom_hex_iso() {
        assert_eq!(toks("'\\x41\\'"), vec![Tok::QuotedAtom("A".to_string())]);
    }

    #[test]
    fn operator_longest_match() {
        assert_eq!(
            toks("=< := ** =\\="),
            vec![Tok::Le, Tok::Assign, Tok::Pow, Tok::EqNeq]
        );
    }

    #[test]
    fn skip_comment_and_ws() {
        assert_eq!(toks("a % c\n b"), vec![Tok::Atom("a"), Tok::Atom("b")]);
    }

    #[test]
    fn float() {
        assert_eq!(toks("3.14"), vec![Tok::Float("3.14")]);
        assert_eq!(toks("-2.5e10"), vec![Tok::Minus, Tok::Float("2.5e10")]);
        assert_eq!(toks("1.0E-5"), vec![Tok::Float("1.0E-5")]);
    }

    #[test]
    fn int() {
        assert_eq!(toks("42"), vec![Tok::Int("42")]);
        assert_eq!(toks("-123"), vec![Tok::Minus, Tok::Int("123")]);
    }

    #[test]
    fn based_int() {
        assert_eq!(toks("0xFF"), vec![Tok::BasedInt("0xFF")]);
        assert_eq!(toks("0o77"), vec![Tok::BasedInt("0o77")]);
        assert_eq!(toks("0b101"), vec![Tok::BasedInt("0b101")]);
        assert_eq!(toks("-0x10"), vec![Tok::Minus, Tok::BasedInt("0x10")]);
    }

    #[test]
    fn var() {
        assert_eq!(toks("X"), vec![Tok::Var("X")]);
        assert_eq!(toks("Var_Name"), vec![Tok::Var("Var_Name")]);
        assert_eq!(toks("_Private"), vec![Tok::Var("_Private")]);
    }

    #[test]
    fn char_code() {
        assert_eq!(toks("0'A"), vec![Tok::CharCode("0'A")]);
        assert_eq!(toks("0'\\x41\\"), vec![Tok::CharCode("0'\\x41\\")]);
        assert_eq!(toks("0'\\101\\"), vec![Tok::CharCode("0'\\101\\")]);
    }
}
