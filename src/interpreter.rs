mod eval;
pub mod lexer;
mod parser;
mod resolver;

use std::fmt::Display;

pub(crate) use lexer::token;
use lexer::tokenize;
use parser::ast::{Expr, Stmt};

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
    ParseError(()),
    ResolverError(resolver::ResolverError),
    ExecError(()),
}

impl Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpreterError::LexError => write!(f, "Lex error"),
            InterpreterError::ResolverError(err) => write!(f, "Resolver error: {:?}", err),
            InterpreterError::ParseError(_) => write!(f, "Parse error"),
            InterpreterError::ExecError(_) => write!(f, "Exec error"),
        }
    }
}

pub fn parse(input: &str) -> Result<Vec<Stmt>, InterpreterError> {
    let tokens = tokenize(input).map_err(|_| InterpreterError::LexError)?;

    let mut parser = parser::ParserInstance::new(tokens);
    parser.parse().map_err(|_| InterpreterError::ParseError(()))
}

pub fn parse_expr(input: &str) -> Result<Expr, InterpreterError> {
    let tokens = tokenize(input).map_err(|_| InterpreterError::LexError)?;
    let mut parser = parser::ParserInstance::new(tokens);
    parser
        .parse_expr()
        .map_err(|_| InterpreterError::ParseError(()))
}

pub fn eval(input: &str) -> Result<parser::ast::Literal, InterpreterError> {
    let expr = parse_expr(input)?;

    let mut resolver = resolver::Resolver::new();
    resolver
        .resolve_expr(&expr)
        .map_err(InterpreterError::ResolverError)?;

    let mut ctx = eval::EvalCtx::new_globals(resolver.into_resolved_exprs());
    eval::Eval::eval(&expr, &mut ctx).map_err(|_| InterpreterError::ExecError(()))
}

pub fn run(input: &str) -> Result<(), InterpreterError> {
    let stmts = parse(input).map_err(|_| InterpreterError::ParseError(()))?;

    let mut resolver = resolver::Resolver::new();
    resolver
        .resolve(&stmts)
        .map_err(InterpreterError::ResolverError)?;

    let mut ctx = eval::EvalCtx::new_globals(resolver.into_resolved_exprs());
    for stmt in &stmts {
        let result = stmt.eval(&mut ctx);
        if let Err(err) = result {
            eprintln!("ExecError: {}", err);
            return Err(InterpreterError::ExecError(()));
        }
    }
    Ok(())
}
