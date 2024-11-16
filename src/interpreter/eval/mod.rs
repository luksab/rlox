pub mod lox_callable;
pub mod lox_function;
pub(crate) use lox_callable::LoxCallable;
use lox_function::LoxFunction;
use std::{backtrace::Backtrace, cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use super::{parser::ast::*, Expr, SouceCodeRange};

#[derive(Debug)]
pub struct ExecError {
    pub(crate) message: String,
    pub(crate) range: SouceCodeRange,
    #[allow(dead_code)]
    pub(crate) backtrace: Backtrace,
}

impl Display for ExecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // writeln!(f, "{}", self.backtrace)?;
        write!(f, "{} at line {}", self.message, self.range.line)
    }
}

type Result<T> = std::result::Result<T, ExecError>;

#[derive(Debug, PartialEq)]
pub struct EvalCtx {
    /// Stack of variable scopes
    ///
    /// The last element is the innermost scope
    variables: Rc<RefCell<HashMap<String, Rc<RefCell<Literal>>>>>,
    enclosing: Option<Rc<RefCell<EvalCtx>>>,
    /// If the current loop should break
    break_loop: Rc<RefCell<bool>>,
    /// If the current loop should continue
    continue_loop: Rc<RefCell<bool>>,
    /// If the current function should return
    return_value: Rc<RefCell<Option<Literal>>>,
}

impl Clone for EvalCtx {
    fn clone(&self) -> Self {
        EvalCtx {
            variables: self.variables.clone(),
            enclosing: self.enclosing.clone(),
            break_loop: self.break_loop.clone(),
            continue_loop: self.continue_loop.clone(),
            return_value: self.return_value.clone(),
        }
    }
}

impl EvalCtx {
    pub fn new_globals() -> Self {
        let mut globals = HashMap::new();
        globals.insert(
            "clock".to_string(),
            Rc::new(RefCell::new(Literal::Callable(Box::new(
                crate::interpreter::eval::lox_callable::Clock,
            )))),
        );
        globals.insert(
            "syscall".to_string(),
            Rc::new(RefCell::new(Literal::Callable(Box::new(
                crate::interpreter::eval::lox_callable::SysCall,
            )))),
        );
        EvalCtx {
            variables: Rc::new(RefCell::new(globals)),
            enclosing: None,
            break_loop: Rc::new(RefCell::new(false)),
            continue_loop: Rc::new(RefCell::new(false)),
            return_value: Rc::new(RefCell::new(None)),
        }
    }

    pub fn insert(&mut self, name: String, value: Literal) {
        self.variables.borrow_mut().insert(name, Rc::new(RefCell::new(value)));
    }

    pub fn assign(&mut self, name: &str, value: Literal) -> Result<()> {
        if let Some(scope) = self.variables.borrow_mut().get_mut(name) {
            *scope.borrow_mut() = value;
            return Ok(());
        }
        if let Some(enclosing) = &self.enclosing {
            let mut enclosing = enclosing.borrow_mut();
            enclosing.assign(name, value)?;
            return Ok(());
        }
        Err(ExecError {
            message: format!("Undefined variable '{}'", name),
            range: SouceCodeRange {
                line: 0,
                start_column: 0,
                length: 0,
            },
            backtrace: Backtrace::capture(),
        })
    }

    pub fn get(&self, name: &str) -> Option<Rc<RefCell<Literal>>> {
        if let Some(value) = self.variables.borrow_mut().get(name) {
            return Some(value.clone());
        }
        if let Some(enclosing) = &self.enclosing {
            let enclosing = enclosing.borrow();
            enclosing.get(name)
        } else {
            None
        }
    }

    fn new_scope(&self) -> Self {
        EvalCtx {
            variables: Rc::new(RefCell::new(HashMap::new())),
            enclosing: Some(Rc::new(RefCell::new(self.clone()))),
            break_loop: self.break_loop.clone(),
            continue_loop: self.continue_loop.clone(),
            return_value: self.return_value.clone(),
        }
    }

    fn get_break_loop(&self) -> bool {
        *self.break_loop.borrow()
    }

    fn get_continue_loop(&self) -> bool {
        *self.continue_loop.borrow()
    }

    fn get_return_value(&self) -> Option<Literal> {
        self.return_value.borrow().clone()
    }

    fn set_break_loop(&self, value: bool) {
        *self.break_loop.borrow_mut() = value;
    }

    fn set_continue_loop(&self, value: bool) {
        *self.continue_loop.borrow_mut() = value;
    }

    fn set_return_value(&self, value: Option<Literal>) {
        *self.return_value.borrow_mut() = value;
    }
}

pub trait Eval {
    fn eval(&self, ctx: &mut EvalCtx) -> Result<Literal>;
}

impl Stmt {
    pub fn eval(&self, ctx: &mut EvalCtx) -> Result<()> {
        // If we're in a loop and we should break or continue, don't execute the statement
        if ctx.get_break_loop() || ctx.get_continue_loop() || ctx.get_return_value().is_some() {
            return Ok(());
        }
        match &self.intern {
            StmtType::Expr(expr) => {
                expr.eval(ctx)?;
                Ok(())
            }
            StmtType::Print(expr) => {
                let value = expr.eval(ctx)?;
                println!("{}", value);
                Ok(())
            }
            StmtType::Var(name, initalizer) => {
                let value = initalizer.eval(ctx)?;
                ctx.insert(name.clone(), value);
                Ok(())
            }
            StmtType::Block(stmts) => {
                // ctx.push_scope();
                let mut new_ctx = ctx.new_scope();
                for stmt in stmts {
                    if let Err(e) = stmt.eval(&mut new_ctx) {
                        // Make sure that the scope is popped before returning the error
                        return Err(e);
                    }
                    //TODO: if we'd want the block-return, we could add that here.
                }
                Ok(())
            }
            StmtType::IfStmt(expr, then, els) => {
                let condition = expr.eval(ctx)?;
                if bool::from(condition) {
                    then.eval(ctx)?;
                } else {
                    if let Some(els) = els {
                        els.eval(ctx)?;
                    }
                }
                Ok(())
            }
            StmtType::While(expr, stmt) => {
                while bool::from(expr.eval(ctx)?) {
                    stmt.eval(ctx)?;
                    if ctx.get_break_loop() || ctx.get_return_value().is_some() {
                        ctx.set_continue_loop(false);
                        ctx.set_break_loop(false);
                        break;
                    }
                    ctx.set_continue_loop(false);
                    ctx.set_break_loop(false);
                }
                Ok(())
            }
            StmtType::Break => {
                ctx.set_break_loop(false);
                Ok(())
            }
            StmtType::Continue => {
                ctx.set_continue_loop(false);
                Ok(())
            }
            StmtType::Return(expr) => {
                let literal = expr.eval(ctx)?;
                ctx.set_return_value(Some(literal));
                Ok(())
            }
            StmtType::Function(function_type, name, vec, stmt) => {
                let function = Literal::Callable(Box::new(LoxFunction {
                    tipe: function_type.clone(),
                    name: name.clone(),
                    args: vec.clone(),
                    body: stmt.clone(),
                    closure: ctx.clone(),
                }));
                ctx.insert(name.clone(), function);
                Ok(())
            }
        }
    }
}

impl Eval for Expr {
    fn eval(&self, ctx: &mut EvalCtx) -> Result<Literal> {
        match &*self.intern {
            ExprType::Literal(literal) => Ok(literal.to_owned()),
            ExprType::Grouping(expr) => expr.eval(ctx),
            ExprType::Unary(unary) => unary.eval(ctx),
            ExprType::Binary(binary) => binary.eval(ctx),
            ExprType::Variable(name) => {
                let res = ctx.get(name).ok_or(ExecError {
                    message: format!("Undefined variable '{}'", name),
                    range: self.range.clone(),
                    backtrace: Backtrace::capture(),
                })?;
                let value = res.borrow().clone();
                Ok(value)
            }
            ExprType::Assign(name, expr) => {
                let value = expr.eval(ctx)?;
                ctx.assign(name, value.clone())?;
                Ok(value)
            }
            ExprType::Logical(logical) => logical.eval(ctx),
            ExprType::Call(call) => call.eval(ctx),
        }
    }
}

impl Eval for Call {
    fn eval(&self, ctx: &mut EvalCtx) -> Result<Literal> {
        let callee = self.callee.eval(ctx)?;

        let mut arguments = Vec::new();
        for arg in &self.arguments {
            arguments.push(arg.eval(ctx)?);
        }

        match callee {
            Literal::Callable(callable) => {
                if !callable.arity_matches(arguments.len()) {
                    return Err(ExecError {
                        message: format!(
                            "Expected {} arguments but got {}",
                            callable.print_arity(),
                            arguments.len()
                        ),
                        range: self.callee.range.clone(),
                        backtrace: Backtrace::capture(),
                    });
                }
                callable.call(arguments, ctx)
            }
            _ => Err(ExecError {
                message: "Can only call functions and classes".to_string(),
                range: self.callee.range.clone(),
                backtrace: Backtrace::capture(),
            }),
        }
    }
}

impl Eval for Logical {
    fn eval(&self, ctx: &mut EvalCtx) -> Result<Literal> {
        let left = self.left.eval(ctx)?;
        match (&self.operator, bool::from(&left)) {
            (LogicalOperator::And, false) => Ok(left),
            (LogicalOperator::Or, true) => Ok(left),
            _ => self.right.eval(ctx),
        }
    }
}

impl Eval for Unary {
    fn eval(&self, ctx: &mut EvalCtx) -> Result<Literal> {
        match &self.intern {
            UnaryType::Neg => match self.expr.eval(ctx)? {
                Literal::Number(n) => Ok(Literal::Number(-n)),
                _ => Err(ExecError {
                    message: "Unary minus expects a number".to_string(),
                    backtrace: Backtrace::capture(),
                    range: self.expr.range.clone(),
                }),
            },
            UnaryType::Not => Ok(Literal::from(!bool::from(self.expr.eval(ctx)?))),
        }
    }
}

impl Eval for Binary {
    fn eval(&self, ctx: &mut EvalCtx) -> Result<Literal> {
        match &self.operator {
            Operator::Plus => match (self.left.eval(ctx)?, self.right.eval(ctx)?) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l + r)),
                (Literal::String(l), Literal::String(r)) => {
                    Ok(Literal::String(format!("{}{}", l, r)))
                }
                (Literal::String(l), other) => Ok(Literal::String(format!("{}{}", l, other))),
                (other, Literal::String(r)) => Ok(Literal::String(format!("{}{}", other, r))),
                _ => Err(ExecError {
                    message: "Operands must be two numbers or two strings".to_string(),
                    range: self.left.range.merge(&self.right.range),
                    backtrace: Backtrace::capture(),
                }),
            },
            Operator::Minus | Operator::Times | Operator::Div => {
                match (self.left.eval(ctx)?, self.right.eval(ctx)?) {
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
                match (self.left.eval(ctx)?, self.right.eval(ctx)?) {
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
                let left = self.left.eval(ctx)?;
                let right = self.right.eval(ctx)?;
                Ok(match self.operator {
                    Operator::EqualEqual => Literal::from(left == right),
                    Operator::NEqualEqual => Literal::from(left != right),
                    _ => unreachable!(),
                })
            }
        }
    }
}
