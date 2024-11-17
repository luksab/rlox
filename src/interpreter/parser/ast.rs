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

use crate::interpreter::{eval::LoxCallable, token::TokenType, SouceCodeRange};

#[derive(Debug, Clone)]
pub(crate) struct Stmt {
    pub intern: StmtType,
    pub range: SouceCodeRange,
}

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.intern)
    }
}

#[derive(Debug, Clone)]
pub(crate) enum StmtType {
    Expr(Expr),
    IfStmt(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Print(Expr),
    Return(Expr),
    Var(String, Option<Expr>),
    While(Expr, Box<Stmt>),
    Block(Vec<Stmt>),
    Break,
    Continue,
    Function(FunctionType, String, Vec<String>, Box<Stmt>),
}

impl Display for StmtType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StmtType::Expr(expr) => write!(f, "{}", expr),
            StmtType::Print(expr) => write!(f, "(print {})", expr),
            StmtType::Return(expr) => {
                write!(f, "(return {})", expr)
            }
            StmtType::Var(name, initializer) => match initializer {
                Some(initializer) => write!(f, "(var {} = {})", name, initializer),
                None => write!(f, "(var {})", name),
            },
            StmtType::IfStmt(condition, then_branch, else_branch) => {
                let mut result = String::new();
                result.push_str("(if ");
                result.push_str(&format!("{} ", condition));
                result.push_str(&format!("{} ", then_branch));
                if let Some(else_branch) = else_branch {
                    result.push_str(&format!("{} ", else_branch));
                }
                result.push_str(")");
                write!(f, "{}", result)
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
            StmtType::While(expr, stmt) => {
                write!(f, "(while {} {})", expr, stmt)
            }
            StmtType::Break => write!(f, "break"),
            StmtType::Continue => write!(f, "continue"),
            StmtType::Function(function, name, ..) => write!(f, "{} {}", function.tipe(), name),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ExprId(pub usize);

impl Display for ExprId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Expr {
    pub intern: Box<ExprType>,
    pub range: SouceCodeRange,
    pub id: ExprId,
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.intern)
    }
}

#[derive(Debug, Clone)]
pub(crate) enum ExprType {
    Literal(Literal),
    Grouping(Expr),
    Unary(Unary),
    Binary(Binary),
    Logical(Logical),
    Variable(String),
    Assign(String, Expr),
    Call(Call),
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
            ExprType::Logical(logical) => write!(f, "{logical}"),
            ExprType::Call(call) => write!(f, "{call}"),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum FunctionType {
    /// Name, parameters, body
    Function,
}

impl FunctionType {
    pub fn tipe(&self) -> String {
        match self {
            FunctionType::Function => "function".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Call {
    pub callee: Expr,
    pub arguments: Vec<Expr>,
}

impl Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        result.push_str(&format!("(call {} ", self.callee));
        for arg in &self.arguments {
            result.push_str(&format!("{} ", arg));
        }
        result.push_str(")");
        write!(f, "{}", result)
    }
}

#[derive(Clone, PartialEq, Default)]
pub(crate) enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    #[default]
    Nil,
    Callable(Box<dyn LoxCallable>),
}

impl From<bool> for Literal {
    fn from(b: bool) -> Self {
        match b {
            true => Literal::True,
            false => Literal::False,
        }
    }
}

impl From<&Literal> for bool {
    fn from(l: &Literal) -> Self {
        match l {
            Literal::True => true,
            Literal::False => false,
            Literal::Nil => false,
            Literal::Number(num) => *num != 0.0,
            Literal::String(_) => true,
            Literal::Callable(_) => true,
        }
    }
}

impl From<Literal> for bool {
    fn from(l: Literal) -> Self {
        (&l).into()
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
            Literal::Callable(lox_callable) => write!(f, "{:?}", lox_callable),
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
            Literal::Callable(lox_callable) => write!(f, "{}", lox_callable),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Unary {
    pub intern: UnaryType,
    pub expr: Expr,
}

impl Display for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.intern, self.expr)
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub(crate) struct Logical {
    pub left: Expr,
    pub operator: LogicalOperator,
    pub right: Expr,
}

impl Display for Logical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.operator, self.left, self.right)
    }
}

#[derive(Debug, Clone)]
pub(crate) enum LogicalOperator {
    And,
    Or,
}

impl From<&TokenType> for LogicalOperator {
    fn from(token: &TokenType) -> Self {
        match token {
            TokenType::And => LogicalOperator::And,
            TokenType::Or => LogicalOperator::Or,
            _ => panic!("Invalid logical operator"),
        }
    }
}

impl Display for LogicalOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogicalOperator::And => write!(f, "and"),
            LogicalOperator::Or => write!(f, "or"),
        }
    }
}

// operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
//                | "+"  | "-"  | "*" | "/" ;
#[derive(Debug, Clone)]
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
