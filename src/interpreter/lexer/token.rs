use std::fmt::Display;

use strum::{Display, EnumString};

use crate::interpreter::SourceCodeRange;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct Token {
    pub(crate) inner: TokenType,
    pub(crate) lexeme: String,
    // pub(crate) line: usize,
    // pub(crate) start_column: usize,
    // pub(crate) length: usize,
    pub(crate) range: SourceCodeRange,
}

impl Token {
    pub(crate) fn new(
        inner: TokenType,
        lexeme: String,
        line: usize,
        start_column: usize,
        length: usize,
    ) -> Self {
        Self {
            inner,
            lexeme,
            range: SourceCodeRange {
                line,
                start_column,
                length,
            },
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Display, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum TokenType {
    // single-character tokens
    #[strum(serialize = "(")]
    LeftParen,
    #[strum(serialize = ")")]
    RightParen,
    #[strum(serialize = "{")]
    LeftBrace,
    #[strum(serialize = "}")]
    RightBrace,
    #[strum(serialize = ",")]
    Comma,
    #[strum(serialize = ".")]
    Dot,
    #[strum(serialize = "-")]
    Minus,
    #[strum(serialize = "+")]
    Plus,
    #[strum(serialize = ";")]
    Semicolon,
    #[strum(serialize = "/")]
    Slash,
    #[strum(serialize = "*")]
    Star,

    // one or two character tokens
    #[strum(serialize = "!")]
    Bang,
    BangEqual,
    #[strum(serialize = "=")]
    Equal,
    EqualEqual,
    #[strum(serialize = ">")]
    Greater,
    GreaterEqual,
    #[strum(serialize = "<")]
    Less,
    LessEqual,

    // literals
    Identifier(String),
    String(String),
    Number(f64),

    // keywords
    And,
    Break,
    Continue,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

impl TokenType {
    pub(crate) fn is_binary(&self) -> bool {
        match self {
            TokenType::EqualEqual
            | TokenType::BangEqual
            | TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::Less
            | TokenType::LessEqual
            | TokenType::Plus
            | TokenType::Minus
            | TokenType::Star
            | TokenType::Slash => true,
            _ => false,
        }
    }
}

impl TryFrom<char> for TokenType {
    type Error = strum::ParseError;

    fn try_from(c: char) -> Result<Self, strum::ParseError> {
        (c.to_string().as_str()).try_into()
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let literal = match &self.inner {
            TokenType::String(s) => s.clone(),
            // print the number as a string, keeping at least one decimal place
            TokenType::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{:.1}", n)
                } else {
                    n.to_string()
                }
            }
            _ => "null".to_string(),
        };
        write!(f, "{} {} {literal}", self.inner, self.lexeme)
    }
}
