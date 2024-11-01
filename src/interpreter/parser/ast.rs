// expression     → literal
//                | unary
//                | binary
//                | grouping ;

// literal        → NUMBER | STRING | "true" | "false" | "nil" ;
// grouping       → "(" expression ")" ;
// unary          → ( "-" | "!" ) expression ;
// binary         → expression operator expression ;
// operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
//                | "+"  | "-"  | "*" | "/" ;

use std::fmt::Display;

use crate::interpreter::token::TokenType;

pub(crate) struct Expr {
    pub intern: Box<ExprType>,
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.intern)
    }
}

pub(crate) enum ExprType {
    Literal(Literal),
    Grouping(Expr),
    Unary(Unary),
    Binary(Binary),
}

impl Display for ExprType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprType::Literal(literal) => write!(f, "{literal}"),
            ExprType::Grouping(expression) => write!(f, "( group {expression} )"),
            ExprType::Unary(unary) => write!(f, "{unary}"),
            ExprType::Binary(binary) => write!(f, "{binary}"),
        }
    }
}

pub(crate) enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{:.1}", n)
                } else {
                    write!(f, "{}", n)
                }
            }
            Literal::String(s) => write!(f, "{}", s),
            Literal::True => write!(f, "true"),
            Literal::False => write!(f, "false"),
            Literal::Nil => write!(f, "nil"),
        }
    }
}

pub(crate) struct Unary {
    pub intern: UnaryType,
    pub expr: Expr,
}

impl Display for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.intern, self.expr)
    }
}

pub(crate) enum UnaryType {
    Not,
    Neg,
}

impl From<&TokenType> for UnaryType {
    fn from(token: &TokenType) -> Self {
        match token {
            TokenType::Bang => UnaryType::Not,
            TokenType::Minus => UnaryType::Neg,
            _ => panic!("Invalid unary operator"),
        }
    }
}

impl Display for UnaryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryType::Not => write!(f, "!"),
            UnaryType::Neg => write!(f, "-"),
        }
    }
}

pub(crate) struct Binary {
    pub left: Expr,
    pub operator: Operator,
    pub right: Expr,
}

impl Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.operator, self.left, self.right)
    }
}

// operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
//                | "+"  | "-"  | "*" | "/" ;
pub(crate) enum Operator {
    EqualEqual,
    NEqualEqual,
    Less,
    Leq,
    Greater,
    Greq,
    Plus,
    Minus,
    Times,
    Div,
}

impl From<&TokenType> for Operator {
    fn from(token: &TokenType) -> Self {
        match token {
            TokenType::EqualEqual => Operator::EqualEqual,
            TokenType::BangEqual => Operator::NEqualEqual,
            TokenType::Less => Operator::Less,
            TokenType::LessEqual => Operator::Leq,
            TokenType::Greater => Operator::Greater,
            TokenType::GreaterEqual => Operator::Greq,
            TokenType::Plus => Operator::Plus,
            TokenType::Minus => Operator::Minus,
            TokenType::Star => Operator::Times,
            TokenType::Slash => Operator::Div,
            tok => panic!("Invalid operator {:?}", tok),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::EqualEqual => write!(f, "=="),
            Operator::NEqualEqual => write!(f, "!="),
            Operator::Less => write!(f, "<"),
            Operator::Leq => write!(f, "<="),
            Operator::Greater => write!(f, ">"),
            Operator::Greq => write!(f, ">="),
            Operator::Plus => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
            Operator::Times => write!(f, "*"),
            Operator::Div => write!(f, "/"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn print() {
        // let ast = Expression {
        //     intern: Box::new(ExpressionType::Literal(Literal::False)),
        // };
        // (* (- 123) (group 45.67))
        let ast = Expr {
            intern: Box::new(ExprType::Binary(Binary {
                left: Expr {
                    intern: Box::new(ExprType::Unary(Unary {
                        intern: UnaryType::Neg,
                        expr: Expr {
                            intern: Box::new(ExprType::Literal(Literal::Number(123.0))),
                        },
                    })),
                },
                operator: Operator::Times,
                right: Expr {
                    intern: Box::new(ExprType::Grouping(Expr {
                        intern: Box::new(ExprType::Literal(Literal::Number(45.67))),
                    })),
                },
            })),
        };

        println!("{ast}");
        panic!()
    }
}
