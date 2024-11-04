mod lexer;
mod parser;

use std::fmt::Display;

pub(crate) use lexer::token;
use parser::ast::Expr;

#[derive(Debug)]
pub(crate) enum InterpreterError {
    LexError,
    ParseError(parser::ParserError),
}

impl Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpreterError::LexError => write!(f, "Lex error"),
            InterpreterError::ParseError(err) => write!(f, "Parse error: {}", err),
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
