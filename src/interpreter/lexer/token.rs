use std::fmt::Display;

use crate::interpreter::SouceCodeRange;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct Token {
    pub(crate) inner: TokenType,
    pub(crate) lexeme: String,
    // pub(crate) line: usize,
    // pub(crate) start_column: usize,
    // pub(crate) length: usize,
    pub(crate) range: SouceCodeRange,
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
            range: SouceCodeRange {
                line,
                start_column,
                length,
            },
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // one or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
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

impl TryFrom<&str> for TokenType {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "and" => Ok(Self::And),
            "break" => Ok(Self::Break),
            "continue" => Ok(Self::Continue),
            "class" => Ok(Self::Class),
            "else" => Ok(Self::Else),
            "false" => Ok(Self::False),
            "fun" => Ok(Self::Fun),
            "for" => Ok(Self::For),
            "if" => Ok(Self::If),
            "nil" => Ok(Self::Nil),
            "or" => Ok(Self::Or),
            "print" => Ok(Self::Print),
            "return" => Ok(Self::Return),
            "super" => Ok(Self::Super),
            "this" => Ok(Self::This),
            "true" => Ok(Self::True),
            "var" => Ok(Self::Var),
            "while" => Ok(Self::While),
            _ => Err("Not parseable as a token type from string"),
        }
    }
}

impl TryFrom<char> for TokenType {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(Self::LeftParen),
            ')' => Ok(Self::RightParen),
            '{' => Ok(Self::LeftBrace),
            '}' => Ok(Self::RightBrace),
            ',' => Ok(Self::Comma),
            '.' => Ok(Self::Dot),
            '-' => Ok(Self::Minus),
            '+' => Ok(Self::Plus),
            ';' => Ok(Self::Semicolon),
            '/' => Ok(Self::Slash),
            '*' => Ok(Self::Star),
            '!' => Ok(Self::Bang),
            '=' => Ok(Self::Equal),
            '<' => Ok(Self::Less),
            '>' => Ok(Self::Greater),
            _ => Err("Not parseable as a token type from char"),
        }
    }
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

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::LeftParen => write!(f, "LEFT_PAREN"),
            TokenType::RightParen => write!(f, "RIGHT_PAREN"),
            TokenType::LeftBrace => write!(f, "LEFT_BRACE"),
            TokenType::RightBrace => write!(f, "RIGHT_BRACE"),
            TokenType::Comma => write!(f, "COMMA"),
            TokenType::Dot => write!(f, "DOT"),
            TokenType::Minus => write!(f, "MINUS"),
            TokenType::Plus => write!(f, "PLUS"),
            TokenType::Semicolon => write!(f, "SEMICOLON"),
            TokenType::Slash => write!(f, "SLASH"),
            TokenType::Star => write!(f, "STAR"),
            TokenType::Bang => write!(f, "BANG"),
            TokenType::BangEqual => write!(f, "BANG_EQUAL"),
            TokenType::Equal => write!(f, "EQUAL"),
            TokenType::EqualEqual => write!(f, "EQUAL_EQUAL"),
            TokenType::Greater => write!(f, "GREATER"),
            TokenType::GreaterEqual => write!(f, "GREATER_EQUAL"),
            TokenType::Less => write!(f, "LESS"),
            TokenType::LessEqual => write!(f, "LESS_EQUAL"),
            TokenType::Identifier(_) => write!(f, "IDENTIFIER"),
            TokenType::String(_) => write!(f, "STRING"),
            TokenType::Number(_) => write!(f, "NUMBER"),
            TokenType::And => write!(f, "AND"),
            TokenType::Break => write!(f, "BREAK"),
            TokenType::Continue => write!(f, "CONTINUE"),
            TokenType::Class => write!(f, "CLASS"),
            TokenType::Else => write!(f, "ELSE"),
            TokenType::False => write!(f, "FALSE"),
            TokenType::Fun => write!(f, "FUN"),
            TokenType::For => write!(f, "FOR"),
            TokenType::If => write!(f, "IF"),
            TokenType::Nil => write!(f, "NIL"),
            TokenType::Or => write!(f, "OR"),
            TokenType::Print => write!(f, "PRINT"),
            TokenType::Return => write!(f, "RETURN"),
            TokenType::Super => write!(f, "SUPER"),
            TokenType::This => write!(f, "THIS"),
            TokenType::True => write!(f, "TRUE"),
            TokenType::Var => write!(f, "VAR"),
            TokenType::While => write!(f, "WHILE"),
            TokenType::EOF => write!(f, "EOF"),
        }
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
