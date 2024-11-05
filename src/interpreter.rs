mod eval;
mod lexer;
mod parser;

use std::fmt::Display;

use eval::Eval;
pub(crate) use lexer::token;
use parser::ast::Expr;

#[derive(Debug, Clone)]
pub(crate) struct SouceCodeRange {
    pub(crate) line: usize,
    pub(crate) start_column: usize,
    pub(crate) length: usize,
}

impl SouceCodeRange {
    pub(crate) fn merge(&self, other: &Self) -> Self {
        let line = self.line.min(other.line);
        let start_column = self.start_column.min(other.start_column);
        let length = self.length + other.length;
        Self {
            line,
            start_column,
            length,
        }
    }
}

#[derive(Debug)]
pub(crate) enum InterpreterError {
    LexError,
    ParseError(parser::ParserError),
    ExecError(eval::ExecError),
}

impl Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpreterError::LexError => write!(f, "Lex error"),
            InterpreterError::ParseError(err) => write!(f, "Parse error: {}", err),
            InterpreterError::ExecError(err) => write!(f, "Exec error: {}", err),
        }
    }
}

pub fn lex(input: &str) -> Result<Vec<token::Token>, Vec<token::Token>> {
    let mut lexer = lexer::LexerInstance::new();
    lexer.tokenize(input);
    if lexer.had_error {
        Err(lexer.tokens)
    } else {
        Ok(lexer.tokens)
    }
}

pub fn parse(input: &str) -> Result<Expr, InterpreterError> {
    let tokens = lex(input);
    match tokens {
        Ok(tokens) => {
            let mut parser = parser::ParserInstance::new(tokens);
            let expr = parser.parse();
            match expr {
                Ok(expr) => Ok(expr),
                Err(err) => Err(InterpreterError::ParseError(err)),
            }
        }
        Err(_) => {
            eprintln!("Failed to lex input");
            Err(InterpreterError::LexError)
        }
    }
}

pub fn eval(input: &str) -> Result<parser::ast::Literal, InterpreterError> {
    let expr = parse(input);
    match expr {
        Ok(expr) => {
            let result = expr.eval();
            match result {
                Ok(result) => Ok(result),
                Err(err) => {
                    eprintln!("{}", err);
                    Err(InterpreterError::ExecError(err))
                }
            }
        }
        Err(err) => Err(err),
    }
}
