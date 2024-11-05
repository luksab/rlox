use std::{backtrace::Backtrace, fmt::Display};

use super::{parser::ast::*, Expr, SouceCodeRange};

#[derive(Debug)]
pub struct ExecError {
    pub(crate) message: String,
    pub(crate) range: SouceCodeRange,
    pub(crate) backtrace: Backtrace,
}

impl Display for ExecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // writeln!(f, "{}", self.backtrace)?;
        write!(f, "{} at line {}", self.message, self.range.line)
    }
}

type Result<T> = std::result::Result<T, ExecError>;

pub struct EvalInstance {
    pub expr: Expr,
}

pub trait Eval {
    fn eval(&self) -> Result<Literal>;
}

impl Eval for Expr {
    fn eval(&self) -> Result<Literal> {
        match &*self.intern {
            ExprType::Literal(literal) => Ok(literal.to_owned()),
            ExprType::Grouping(expr) => expr.eval(),
            ExprType::Unary(unary) => unary.eval(),
            ExprType::Binary(binary) => binary.eval(),
        }
    }
}

impl Eval for Unary {
    fn eval(&self) -> Result<Literal> {
        match &self.intern {
            UnaryType::Neg => match self.expr.eval()? {
                Literal::Number(n) => Ok(Literal::Number(-n)),
                _ => Err(ExecError {
                    message: "Unary minus expects a number".to_string(),
                    backtrace: Backtrace::capture(),
                    range: self.expr.range.clone(),
                }),
            },
            UnaryType::Not => Ok(Literal::from(!bool::from(self.expr.eval()?))),
        }
    }
}

impl Eval for Binary {
    fn eval(&self) -> Result<Literal> {
        match &self.operator {
            Operator::Plus => match (self.left.eval()?, self.right.eval()?) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l + r)),
                (Literal::String(l), Literal::String(r)) => {
                    Ok(Literal::String(format!("{}{}", l, r)))
                }
                _ => Err(ExecError {
                    message: "Operands must be two numbers or two strings".to_string(),
                    range: self.left.range.merge(&self.right.range),
                    backtrace: Backtrace::capture(),
                }),
            },
            Operator::Minus | Operator::Times | Operator::Div => {
                match (self.left.eval()?, self.right.eval()?) {
                    (Literal::Number(l), Literal::Number(r)) => {
                        Ok(Literal::Number(match self.operator {
                            Operator::Minus => l - r,
                            Operator::Times => l * r,
                            Operator::Div => l / r,
                            _ => unreachable!(),
                        }))
                    }
                    _ => Err(ExecError {
                        message: "Operands must be numbers".to_string(),
                        range: self.left.range.merge(&self.right.range),
                        backtrace: Backtrace::capture(),
                    }),
                }
            }
            Operator::Greq | Operator::Greater | Operator::Leq | Operator::Less => {
                match (self.left.eval()?, self.right.eval()?) {
                    (Literal::Number(l), Literal::Number(r)) => {
                        Ok(Literal::from(match self.operator {
                            Operator::Greq => l >= r,
                            Operator::Greater => l > r,
                            Operator::Leq => l <= r,
                            Operator::Less => l < r,
                            _ => unreachable!(),
                        }))
                    }
                    _ => Err(ExecError {
                        message: "Operands must be numbers".to_string(),
                        range: self.left.range.merge(&self.right.range),
                        backtrace: Backtrace::capture(),
                    }),
                }
            }
            Operator::EqualEqual | Operator::NEqualEqual => {
                let left = self.left.eval()?;
                let right = self.right.eval()?;
                Ok(match self.operator {
                    Operator::EqualEqual => Literal::from(left == right),
                    Operator::NEqualEqual => Literal::from(left != right),
                    _ => unreachable!(),
                })
            }
        }
    }
}
