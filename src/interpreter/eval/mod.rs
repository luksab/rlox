pub mod lox_callable;
pub mod lox_class;
pub mod lox_function;
pub mod lox_instance;
pub(crate) use lox_callable::LoxCallable;
use lox_class::LoxClass;
use lox_function::LoxFunction;
use lox_instance::LoxInstance;
use std::{backtrace::Backtrace, cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use super::{parser::ast::*, Expr, SourceCodeRange};

#[derive(Debug)]
pub struct ExecError {
    pub(crate) message: String,
    pub(crate) range: SourceCodeRange,
    #[allow(dead_code)]
    pub(crate) backtrace: Backtrace,
}

impl ExecError {
    pub(crate) fn new(message: String, range: SourceCodeRange) -> Self {
        Self {
            message,
            range,
            backtrace: Backtrace::capture(),
        }
    }
}

impl Display for ExecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // writeln!(f, "{}", self.backtrace)?;
        write!(f, "{} at line {}", self.message, self.range.line)
    }
}

type ExecResult<T> = std::result::Result<T, ExecError>;

#[derive(Debug, PartialEq)]
pub struct EvalCtx {
    /// Stack of variable scopes
    ///
    /// The last element is the innermost scope
    variables: Rc<RefCell<HashMap<String, Rc<RefCell<Literal>>>>>,
    globals: Rc<RefCell<HashMap<String, Rc<RefCell<Literal>>>>>,
    enclosing: Option<Rc<RefCell<EvalCtx>>>,
    /// If the current loop should break
    break_loop: Rc<RefCell<bool>>,
    /// If the current loop should continue
    continue_loop: Rc<RefCell<bool>>,
    /// If the current function should return
    return_value: Rc<RefCell<Option<Literal>>>,
    locals: Rc<RefCell<HashMap<ExprId, usize>>>,
}

impl Clone for EvalCtx {
    fn clone(&self) -> Self {
        EvalCtx {
            variables: self.variables.clone(),
            globals: self.globals.clone(),
            enclosing: self.enclosing.clone(),
            break_loop: self.break_loop.clone(),
            continue_loop: self.continue_loop.clone(),
            return_value: self.return_value.clone(),
            locals: self.locals.clone(),
        }
    }
}

impl EvalCtx {
    pub fn new_globals(locals: HashMap<ExprId, usize>) -> Self {
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
            globals: Rc::new(RefCell::new(globals)),
            variables: Rc::new(RefCell::new(HashMap::new())),
            enclosing: None,
            break_loop: Rc::new(RefCell::new(false)),
            continue_loop: Rc::new(RefCell::new(false)),
            return_value: Rc::new(RefCell::new(None)),
            locals: Rc::new(RefCell::new(locals)),
        }
    }

    pub fn insert(&mut self, name: String, value: Literal) {
        self.variables
            .borrow_mut()
            .insert(name, Rc::new(RefCell::new(value)));
    }

    pub fn assign(&mut self, name: &str, id: ExprId, value: Literal) -> ExecResult<()> {
        let distance = self.locals.borrow().get(&id).cloned();
        if let Some(distance) = distance {
            let ctx = self.ancestor(distance);
            if let Some(scope) = ctx.borrow().variables.borrow_mut().get_mut(name) {
                *scope.borrow_mut() = value;
                return Ok(());
            }
            return Err(ExecError {
                message: format!("Can't assign to undefined local variable '{}'", name),
                range: SourceCodeRange {
                    line: 0,
                    start_column: 0,
                    length: 0,
                },
                backtrace: Backtrace::capture(),
            });
        } else {
            if let Some(scope) = self.globals.borrow_mut().get_mut(name) {
                *scope.borrow_mut() = value;
                return Ok(());
            }
            return Err(ExecError {
                message: format!("Can't assign to undefined global variable '{}'", name),
                range: SourceCodeRange {
                    line: 0,
                    start_column: 0,
                    length: 0,
                },
                backtrace: Backtrace::capture(),
            });
        }
    }

    pub fn get(&self, name: &str, id: ExprId) -> Option<Rc<RefCell<Literal>>> {
        let distance = self.locals.borrow().get(&id).cloned();
        if let Some(distance) = distance {
            let ctx = self.ancestor(distance);
            let x = ctx.borrow().variables.borrow().get(name).cloned();
            x
        } else {
            self.globals.borrow().get(name).cloned()
        }
    }

    fn ancestor(&self, distance: usize) -> Rc<RefCell<EvalCtx>> {
        let mut ctx = self.clone();
        for _ in 0..distance {
            let enclosing = ctx.enclosing.as_ref().unwrap().borrow().clone();
            ctx = enclosing;
        }
        Rc::new(RefCell::new(ctx))
    }

    fn new_scope(&self) -> Self {
        EvalCtx {
            variables: Rc::new(RefCell::new(HashMap::new())),
            globals: self.globals.clone(),
            enclosing: Some(Rc::new(RefCell::new(self.clone()))),
            break_loop: self.break_loop.clone(),
            continue_loop: self.continue_loop.clone(),
            return_value: self.return_value.clone(),
            locals: self.locals.clone(),
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
    fn eval(&self, ctx: &mut EvalCtx) -> ExecResult<Literal>;
}

impl Stmt {
    pub fn eval(&self, ctx: &mut EvalCtx) -> ExecResult<()> {
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
                let value = match initalizer {
                    Some(init) => init.eval(ctx)?,
                    None => Literal::Nil,
                };
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
            StmtType::Class(name, methods) => {
                let class = Literal::Class(LoxClass::new(name.clone(), methods.clone()));
                ctx.insert(name.clone(), class);
                Ok(())
            }
        }
    }
}

impl Eval for Expr {
    fn eval(&self, ctx: &mut EvalCtx) -> ExecResult<Literal> {
        match &*self.intern {
            ExprType::Literal(literal) => Ok(literal.to_owned()),
            ExprType::Grouping(expr) => expr.eval(ctx),
            ExprType::Unary(unary) => unary.eval(ctx),
            ExprType::Binary(binary) => binary.eval(ctx),
            ExprType::Variable(name) => {
                let res = ctx.get(name, self.id).ok_or(ExecError {
                    message: format!("Can't get undefined variable {} '{name}'", self.id),
                    range: self.range.clone(),
                    backtrace: Backtrace::capture(),
                })?;
                let value = res.borrow().clone();
                Ok(value)
            }
            ExprType::Assign(name, expr) => {
                let value = expr.eval(ctx)?;
                ctx.assign(name, self.id, value.clone())?;
                Ok(value)
            }
            ExprType::Logical(logical) => logical.eval(ctx),
            ExprType::Call(call) => call.eval(ctx),
            ExprType::Get(get, name) => {
                let object = get.eval(ctx)?;
                match object {
                    Literal::Instance(instance) => {
                        if let Some(method) = instance.borrow().methods.get(name) {
                            Ok(Literal::Callable(Box::new(method.clone())))
                        } else {
                            instance.borrow().get(name)
                        }
                    }
                    _ => Err(ExecError {
                        message: "Only instances have properties".to_string(),
                        range: self.range.clone(),
                        backtrace: Backtrace::capture(),
                    }),
                }
            }
            ExprType::Set(set, name, value) => {
                let object = set.eval(ctx)?;

                match object {
                    Literal::Instance(instance) => {
                        // // check, if name exists, before setting it
                        // instance.borrow().get(name)?;
                        instance.borrow_mut().set(name, value.eval(ctx)?)
                    }
                    _ => Err(ExecError::new(
                        "Only instances have fields".to_string(),
                        self.range.clone(),
                    )),
                }
            }
        }
    }
}

impl Eval for Call {
    fn eval(&self, ctx: &mut EvalCtx) -> ExecResult<Literal> {
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
            Literal::Class(class) => {
                let instance = Literal::Instance(Rc::new(RefCell::new(LoxInstance::new(&class))));
                let init = class.methods.get("init").cloned();
                if let Some(init) = init {
                    let mut new_ctx = ctx.new_scope();
                    new_ctx.insert("this".to_string(), instance.clone());
                    init.body.eval(&mut new_ctx)?;
                }
                Ok(instance)
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
    fn eval(&self, ctx: &mut EvalCtx) -> ExecResult<Literal> {
        let left = self.left.eval(ctx)?;
        match (&self.operator, bool::from(&left)) {
            (LogicalOperator::And, false) => Ok(left),
            (LogicalOperator::Or, true) => Ok(left),
            _ => self.right.eval(ctx),
        }
    }
}

impl Eval for Unary {
    fn eval(&self, ctx: &mut EvalCtx) -> ExecResult<Literal> {
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
    fn eval(&self, ctx: &mut EvalCtx) -> ExecResult<Literal> {
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
