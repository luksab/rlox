use std::{cell::RefCell, fmt::Display, rc::Rc};

use strum::Display;

use crate::interpreter::{
    eval::{lox_class::LoxClass, lox_instance::LoxInstance, LoxCallable},
    token::TokenType,
    SourceCodeRange,
};

#[derive(Debug, Clone)]
pub(crate) struct Stmt {
    pub intern: StmtType,
    pub range: SourceCodeRange,
}

impl Stmt {
    pub(crate) fn into_format(&self) -> super::format::StmtFormatter {
        super::format::StmtFormatter {
            stmt: self,
            print_block: true,
        }
    }

    pub(crate) fn into_format_no_block(&self) -> super::format::StmtFormatter {
        super::format::StmtFormatter {
            stmt: self,
            print_block: false,
        }
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.into_format().fmt(f)
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
    Class(String, Vec<Stmt>),
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
    pub range: SourceCodeRange,
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
    Get(Expr, String),
    Set(Expr, String, Expr),
}

impl Display for ExprType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprType::Literal(literal) => write!(f, "{literal}"),
            ExprType::Grouping(expression) => write!(f, "{expression}"),
            ExprType::Unary(unary) => write!(f, "{unary}"),
            ExprType::Binary(binary) => write!(f, "{binary}"),
            ExprType::Variable(name) => write!(f, "{name}"),
            ExprType::Assign(name, expr) => write!(f, "{name} = {expr}"),
            ExprType::Logical(logical) => write!(f, "{logical}"),
            ExprType::Call(call) => write!(f, "{call}"),
            ExprType::Get(expr, name) => write!(f, "{expr}.{name}"),
            ExprType::Set(expr, name, value) => write!(f, "{expr}.{name} = {value}"),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum FunctionType {
    Function,
    Method,
}

impl FunctionType {
    pub fn tipe(&self) -> String {
        match self {
            FunctionType::Function => "function".to_string(),
            FunctionType::Method => "method".to_string(),
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
        result.push_str(&format!("{}(", self.callee));
        result.push_str(
            &self
                .arguments
                .iter()
                .map(|arg| format!("{}", arg))
                .collect::<Vec<String>>()
                .join(", "),
        );
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
    Class(LoxClass),
    Instance(Rc<RefCell<LoxInstance>>),
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
            Literal::True
            | Literal::String(_)
            | Literal::Callable(_)
            | Literal::Class(_)
            | Literal::Instance(_) => true,
            Literal::False | Literal::Nil => false,
            Literal::Number(num) => *num != 0.0,
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
            Literal::Class(lox_class) => write!(f, "{:?}", lox_class),
            Literal::Instance(lox_instance) => write!(f, "{:?}", lox_instance),
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Number(n) => write!(f, "{}", n),
            Literal::String(s) => write!(f, "\"{}\"", s),
            Literal::True => write!(f, "true"),
            Literal::False => write!(f, "false"),
            Literal::Nil => write!(f, "nil"),
            Literal::Callable(lox_callable) => write!(f, "{}", lox_callable),
            Literal::Class(lox_class) => write!(f, "{}", lox_class),
            Literal::Instance(lox_instance) => write!(f, "{}", lox_instance.borrow()),
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
        write!(f, "{}{}", self.intern, self.expr)
    }
}

#[derive(Debug, Clone, Display)]
pub(crate) enum UnaryType {
    #[strum(serialize = "!")]
    Not,
    #[strum(serialize = "-")]
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

#[derive(Debug, Clone)]
pub(crate) struct Binary {
    pub left: Expr,
    pub operator: Operator,
    pub right: Expr,
}

impl Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.left, self.operator, self.right)
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

#[derive(Debug, Clone, Display)]
pub(crate) enum Operator {
    #[strum(serialize = "==")]
    EqualEqual,
    #[strum(serialize = "!=")]
    NEqualEqual,
    #[strum(serialize = "<")]
    Less,
    #[strum(serialize = "<=")]
    Leq,
    #[strum(serialize = ">")]
    Greater,
    #[strum(serialize = ">=")]
    Greq,
    #[strum(serialize = "+")]
    Plus,
    #[strum(serialize = "-")]
    Minus,
    #[strum(serialize = "*")]
    Times,
    #[strum(serialize = "/")]
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
