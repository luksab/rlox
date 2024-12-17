use std::fmt::Display;

use crate::interpreter::parser::ast::Literal;


#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LiteralToValueError;

impl TryFrom<Literal> for Value {
    type Error = LiteralToValueError;

    fn try_from(value: Literal) -> Result<Self, Self::Error> {
        match value {
            Literal::Number(n) => Ok(Value::Number(n)),
            Literal::True => Ok(Value::Bool(true)),
            Literal::False => Ok(Value::Bool(false)),
            Literal::Nil => Ok(Value::Nil),
            _ => Err(LiteralToValueError),
        }
    }
}