use std::fmt::Display;

use crate::interpreter::parser::ast::Literal;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Nil,
    String(String),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            (Value::String(a), Value::String(b)) => a == b,
            _ => false,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
            Value::String(s) => write!(f, "{}", s),
        }
    }
}

impl From<&Value> for bool {
    fn from(v: &Value) -> Self {
        match v {
            Value::Bool(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::Nil => false,
            Value::String(s) => !s.is_empty(),
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
            Literal::String(s) => Ok(Value::String(s)),
            _ => Err(LiteralToValueError),
        }
    }
}
