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

use crate::interpreter::{token::TokenType, SouceCodeRange};

pub(crate) struct Stmt {
    pub intern: StmtType,
    pub range: SouceCodeRange,
}

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.intern)
    }
}

pub(crate) enum StmtType {
    Expr(Expr),
    Print(Expr),
    Var(String, Expr),
    Block(Vec<Stmt>),
}

impl Display for StmtType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StmtType::Expr(expr) => write!(f, "{}", expr),
            StmtType::Print(expr) => write!(f, "(print {})", expr),
            StmtType::Var(name, initializer) => {
                write!(f, "(var {} = {})", name, initializer)
            }
            StmtType::Block(stmts) => {
                let mut result = String::new();
                result.push_str("{\n");
                for stmt in stmts {
                    result.push_str(&format!("{}\n", stmt));
                }
                result.push_str("}");
                write!(f, "{}", result)
            }
        }
    }
}

pub(crate) struct Expr {
    pub intern: Box<ExprType>,
    pub range: SouceCodeRange,
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
    Variable(String),
    Assign(String, Expr),
}

impl Display for ExprType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprType::Literal(literal) => write!(f, "{literal:?}"),
            ExprType::Grouping(expression) => write!(f, "(group {expression})"),
            ExprType::Unary(unary) => write!(f, "{unary}"),
            ExprType::Binary(binary) => write!(f, "{binary}"),
            ExprType::Variable(name) => write!(f, "{name}"),
            ExprType::Assign(name, expr) => write!(f, "(assign {name} {expr})"),
        }
    }
}

#[derive(Clone, PartialEq)]
pub(crate) enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}

impl From<bool> for Literal {
    fn from(b: bool) -> Self {
        match b {
            true => Literal::True,
            false => Literal::False,
        }
    }
}

impl From<Literal> for bool {
    fn from(l: Literal) -> Self {
        match l {
            Literal::True => true,
            Literal::False => false,
            Literal::Nil => false,
            Literal::Number(num) => num != 0.0,
            Literal::String(_) => true,
        }
    }
}

impl std::fmt::Debug for Literal {
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

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Number(n) => {
                // if n.fract() == 0.0 {
                //     write!(f, "{:.1}", n)
                // } else {
                //     write!(f, "{}", n)
                // }
                write!(f, "{}", n)
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
