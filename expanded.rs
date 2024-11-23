#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod interpreter {
    mod eval {
        pub mod lox_callable {
            use std::backtrace::Backtrace;
            use std::usize;
            use crate::interpreter::eval::ExecError;
            use crate::interpreter::eval::EvalCtx;
            use super::Literal;
            pub(crate) trait LoxCallable: LoxCallableClone + std::fmt::Debug + std::fmt::Display {
                fn call(
                    &self,
                    args: Vec<Literal>,
                    ctx: &mut EvalCtx,
                ) -> Result<Literal, ExecError>;
                fn arity_matches(&self, arity: usize) -> bool;
                fn print_arity(&self) -> String;
            }
            pub(crate) trait LoxCallableClone {
                fn clone_box(&self) -> Box<dyn LoxCallable>;
            }
            impl<T> LoxCallableClone for T
            where
                T: 'static + LoxCallable + Clone,
            {
                fn clone_box(&self) -> Box<dyn LoxCallable> {
                    Box::new(self.clone())
                }
            }
            impl Clone for Box<dyn LoxCallable> {
                fn clone(&self) -> Box<dyn LoxCallable> {
                    self.clone_box()
                }
            }
            impl PartialEq for Box<dyn LoxCallable> {
                fn eq(&self, other: &Self) -> bool {
                    self as *const _ == other as *const _
                }
            }
            pub(crate) struct Clock;
            #[automatically_derived]
            impl ::core::clone::Clone for Clock {
                #[inline]
                fn clone(&self) -> Clock {
                    Clock
                }
            }
            impl LoxCallable for Clock {
                fn call(
                    &self,
                    _args: Vec<Literal>,
                    _ctx: &mut EvalCtx,
                ) -> Result<Literal, ExecError> {
                    Ok(
                        Literal::Number(
                            std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_secs_f64(),
                        ),
                    )
                }
                fn arity_matches(&self, arity: usize) -> bool {
                    (0..).contains(&arity)
                }
                fn print_arity(&self) -> String {
                    "0".to_string()
                }
            }
            impl std::fmt::Display for Clock {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("<native clock fn>"))
                }
            }
            impl std::fmt::Debug for Clock {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("<native clock fn>"))
                }
            }
            pub(crate) struct SysCall;
            #[automatically_derived]
            impl ::core::clone::Clone for SysCall {
                #[inline]
                fn clone(&self) -> SysCall {
                    SysCall
                }
            }
            impl LoxCallable for SysCall {
                fn call(
                    &self,
                    args: Vec<Literal>,
                    _ctx: &mut EvalCtx,
                ) -> Result<Literal, ExecError> {
                    let mut args = args.iter();
                    let syscall = args.next().unwrap();
                    let args = args.collect::<Vec<_>>();
                    match syscall {
                        Literal::String(syscall) => {
                            let syscall = syscall.as_str();
                            match syscall {
                                "exit" => {
                                    let code = args
                                        .get(0)
                                        .map(|arg| match arg {
                                            Literal::Number(num) => Ok(*num as i32),
                                            _ => {
                                                Err(ExecError {
                                                    message: ::alloc::__export::must_use({
                                                        let res = ::alloc::fmt::format(
                                                            format_args!("Expected number as argument to exit syscall"),
                                                        );
                                                        res
                                                    }),
                                                    range: super::SouceCodeRange {
                                                        line: 0,
                                                        start_column: 0,
                                                        length: 0,
                                                    },
                                                    backtrace: Backtrace::force_capture(),
                                                })
                                            }
                                        })
                                        .unwrap_or(Ok(0))?;
                                    std::process::exit(code);
                                }
                                _ => {
                                    Err(ExecError {
                                        message: ::alloc::__export::must_use({
                                            let res = ::alloc::fmt::format(
                                                format_args!("Unknown syscall: {0}", syscall),
                                            );
                                            res
                                        }),
                                        range: super::SouceCodeRange {
                                            line: 0,
                                            start_column: 0,
                                            length: 0,
                                        },
                                        backtrace: Backtrace::force_capture(),
                                    })
                                }
                            }
                        }
                        _ => {
                            Err(ExecError {
                                message: ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Expected string as first argument to syscall"),
                                    );
                                    res
                                }),
                                range: super::SouceCodeRange {
                                    line: 0,
                                    start_column: 0,
                                    length: 0,
                                },
                                backtrace: Backtrace::force_capture(),
                            })
                        }
                    }
                }
                fn arity_matches(&self, arity: usize) -> bool {
                    (1..).contains(&arity)
                }
                fn print_arity(&self) -> String {
                    "1..".to_string()
                }
            }
            impl std::fmt::Display for SysCall {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("<native syscall fn>"))
                }
            }
            impl std::fmt::Debug for SysCall {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("<native syscall fn>"))
                }
            }
        }
        pub mod lox_class {
            use std::collections::HashMap;
            use super::LoxFunction;
            pub(crate) struct LoxClass {
                pub(crate) name: String,
                pub(crate) methods: HashMap<String, LoxFunction>,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for LoxClass {
                #[inline]
                fn clone(&self) -> LoxClass {
                    LoxClass {
                        name: ::core::clone::Clone::clone(&self.name),
                        methods: ::core::clone::Clone::clone(&self.methods),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::default::Default for LoxClass {
                #[inline]
                fn default() -> LoxClass {
                    LoxClass {
                        name: ::core::default::Default::default(),
                        methods: ::core::default::Default::default(),
                    }
                }
            }
            impl PartialEq for LoxClass {
                fn eq(&self, other: &Self) -> bool {
                    self.name == other.name
                }
            }
            impl std::fmt::Debug for LoxClass {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_struct("LoxClass")
                        .field("name", &self.name)
                        .field("methods", &self.methods)
                        .finish()
                }
            }
            impl std::fmt::Display for LoxClass {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("<class {0}>", self.name))
                }
            }
            impl LoxClass {
                pub(crate) fn new(name: String, methods: Vec<super::Stmt>) -> Self {
                    let mut methods = HashMap::new();
                    Self { name, methods }
                }
            }
        }
        pub mod lox_function {
            use std::fmt::Display;
            use super::{EvalCtx, ExecError, FunctionType, Literal, LoxCallable, Stmt};
            pub(crate) struct LoxFunction {
                pub tipe: FunctionType,
                pub name: String,
                pub args: Vec<String>,
                pub body: Box<Stmt>,
                pub closure: EvalCtx,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for LoxFunction {
                #[inline]
                fn clone(&self) -> LoxFunction {
                    LoxFunction {
                        tipe: ::core::clone::Clone::clone(&self.tipe),
                        name: ::core::clone::Clone::clone(&self.name),
                        args: ::core::clone::Clone::clone(&self.args),
                        body: ::core::clone::Clone::clone(&self.body),
                        closure: ::core::clone::Clone::clone(&self.closure),
                    }
                }
            }
            impl std::fmt::Debug for LoxFunction {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_struct("LoxFunction")
                        .field("tipe", &self.tipe)
                        .field("name", &self.name)
                        .field("args", &self.args)
                        .field("body", &self.body)
                        .finish()
                }
            }
            impl LoxCallable for LoxFunction {
                fn call(
                    &self,
                    args: Vec<Literal>,
                    _ctx: &mut EvalCtx,
                ) -> Result<Literal, ExecError> {
                    let mut new_ctx = self.closure.clone().new_scope();
                    for (param, arg) in self.args.iter().zip(args.iter()) {
                        new_ctx.insert(param.clone(), arg.clone());
                    }
                    self.body.eval(&mut new_ctx)?;
                    return Ok(new_ctx.return_value.take().unwrap_or_default());
                }
                fn arity_matches(&self, arity: usize) -> bool {
                    (0..).contains(&arity)
                }
                fn print_arity(&self) -> String {
                    "0".to_string()
                }
            }
            impl Display for LoxFunction {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("<fn>"))
                }
            }
        }
        pub mod lox_instance {
            use std::collections::HashMap;
            use super::{LoxCallable, LoxClass, LoxFunction};
            use super::ExecResult;
            pub(crate) struct LoxInstance {
                pub(crate) name: String,
                pub(crate) methods: HashMap<String, LoxFunction>,
                pub(crate) fields: HashMap<String, super::Literal>,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for LoxInstance {
                #[inline]
                fn clone(&self) -> LoxInstance {
                    LoxInstance {
                        name: ::core::clone::Clone::clone(&self.name),
                        methods: ::core::clone::Clone::clone(&self.methods),
                        fields: ::core::clone::Clone::clone(&self.fields),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::default::Default for LoxInstance {
                #[inline]
                fn default() -> LoxInstance {
                    LoxInstance {
                        name: ::core::default::Default::default(),
                        methods: ::core::default::Default::default(),
                        fields: ::core::default::Default::default(),
                    }
                }
            }
            impl PartialEq for LoxInstance {
                fn eq(&self, other: &Self) -> bool {
                    self.name == other.name
                }
            }
            impl std::fmt::Debug for LoxInstance {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_struct("LoxInstance")
                        .field("name", &self.name)
                        .field("methods", &self.methods)
                        .finish()
                }
            }
            impl std::fmt::Display for LoxInstance {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("<instance {0}>", self.name))
                }
            }
            impl LoxCallable for LoxInstance {
                fn call(
                    &self,
                    _args: Vec<super::Literal>,
                    _ctx: &mut super::EvalCtx,
                ) -> Result<super::Literal, super::ExecError> {
                    Ok(super::Literal::Nil)
                }
                fn arity_matches(&self, arity: usize) -> bool {
                    arity == 0
                }
                fn print_arity(&self) -> String {
                    "0".to_string()
                }
            }
            impl LoxInstance {
                pub(crate) fn new(class: &LoxClass) -> Self {
                    let mut methods = HashMap::new();
                    let fields = HashMap::new();
                    Self {
                        name: class.name.clone(),
                        methods,
                        fields,
                    }
                }
                pub(crate) fn get(&self, name: &str) -> ExecResult<super::Literal> {
                    self.fields
                        .get(name)
                        .ok_or(
                            super::ExecError::new(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Undefined property \'{0}\'.", name),
                                    );
                                    res
                                }),
                                super::SouceCodeRange {
                                    line: 0,
                                    start_column: 0,
                                    length: 0,
                                },
                            ),
                        )
                        .cloned()
                }
                pub(crate) fn set(
                    &mut self,
                    name: &str,
                    value: super::Literal,
                ) -> Result<super::Literal, super::ExecError> {
                    self.fields.insert(name.to_string(), value.clone());
                    Ok(value)
                }
            }
        }
        pub(crate) use lox_callable::LoxCallable;
        use lox_class::LoxClass;
        use lox_function::LoxFunction;
        use lox_instance::LoxInstance;
        use std::{
            backtrace::Backtrace, cell::RefCell, collections::HashMap, fmt::Display,
            rc::Rc,
        };
        use super::{parser::ast::*, Expr, SouceCodeRange};
        pub struct ExecError {
            pub(crate) message: String,
            pub(crate) range: SouceCodeRange,
            #[allow(dead_code)]
            pub(crate) backtrace: Backtrace,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ExecError {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "ExecError",
                    "message",
                    &self.message,
                    "range",
                    &self.range,
                    "backtrace",
                    &&self.backtrace,
                )
            }
        }
        impl ExecError {
            pub(crate) fn new(message: String, range: SouceCodeRange) -> Self {
                Self {
                    message,
                    range,
                    backtrace: Backtrace::capture(),
                }
            }
        }
        impl Display for ExecError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(
                    format_args!("{0} at line {1}", self.message, self.range.line),
                )
            }
        }
        type ExecResult<T> = std::result::Result<T, ExecError>;
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
        #[automatically_derived]
        impl ::core::fmt::Debug for EvalCtx {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "variables",
                    "globals",
                    "enclosing",
                    "break_loop",
                    "continue_loop",
                    "return_value",
                    "locals",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.variables,
                    &self.globals,
                    &self.enclosing,
                    &self.break_loop,
                    &self.continue_loop,
                    &self.return_value,
                    &&self.locals,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "EvalCtx",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for EvalCtx {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for EvalCtx {
            #[inline]
            fn eq(&self, other: &EvalCtx) -> bool {
                self.variables == other.variables && self.globals == other.globals
                    && self.enclosing == other.enclosing
                    && self.break_loop == other.break_loop
                    && self.continue_loop == other.continue_loop
                    && self.return_value == other.return_value
                    && self.locals == other.locals
            }
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
                globals
                    .insert(
                        "clock".to_string(),
                        Rc::new(
                            RefCell::new(
                                Literal::Callable(
                                    Box::new(crate::interpreter::eval::lox_callable::Clock),
                                ),
                            ),
                        ),
                    );
                globals
                    .insert(
                        "syscall".to_string(),
                        Rc::new(
                            RefCell::new(
                                Literal::Callable(
                                    Box::new(crate::interpreter::eval::lox_callable::SysCall),
                                ),
                            ),
                        ),
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
                self.variables.borrow_mut().insert(name, Rc::new(RefCell::new(value)));
            }
            pub fn assign(
                &mut self,
                name: &str,
                id: ExprId,
                value: Literal,
            ) -> ExecResult<()> {
                let distance = self.locals.borrow().get(&id).cloned();
                if let Some(distance) = distance {
                    let ctx = self.ancestor(distance);
                    if let Some(scope) = ctx
                        .borrow()
                        .variables
                        .borrow_mut()
                        .get_mut(name)
                    {
                        *scope.borrow_mut() = value;
                        return Ok(());
                    }
                    return Err(ExecError {
                        message: ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Can\'t assign to undefined local variable \'{0}\'",
                                    name,
                                ),
                            );
                            res
                        }),
                        range: SouceCodeRange {
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
                        message: ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Can\'t assign to undefined global variable \'{0}\'",
                                    name,
                                ),
                            );
                            res
                        }),
                        range: SouceCodeRange {
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
                if ctx.get_break_loop() || ctx.get_continue_loop()
                    || ctx.get_return_value().is_some()
                {
                    return Ok(());
                }
                match &self.intern {
                    StmtType::Expr(expr) => {
                        expr.eval(ctx)?;
                        Ok(())
                    }
                    StmtType::Print(expr) => {
                        let value = expr.eval(ctx)?;
                        {
                            ::std::io::_print(format_args!("{0}\n", value));
                        };
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
                        let mut new_ctx = ctx.new_scope();
                        for stmt in stmts {
                            if let Err(e) = stmt.eval(&mut new_ctx) {
                                return Err(e);
                            }
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
                        let function = Literal::Callable(
                            Box::new(LoxFunction {
                                tipe: function_type.clone(),
                                name: name.clone(),
                                args: vec.clone(),
                                body: stmt.clone(),
                                closure: ctx.clone(),
                            }),
                        );
                        ctx.insert(name.clone(), function);
                        Ok(())
                    }
                    StmtType::Class(name, methods) => {
                        let class = Literal::Class(
                            LoxClass::new(name.clone(), methods.clone()),
                        );
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
                        let res = ctx
                            .get(name, self.id)
                            .ok_or(ExecError {
                                message: ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Can\'t get undefined variable {0} \'{1}\'",
                                            self.id,
                                            name,
                                        ),
                                    );
                                    res
                                }),
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
                            _ => {
                                Err(ExecError {
                                    message: "Only instances have properties".to_string(),
                                    range: self.range.clone(),
                                    backtrace: Backtrace::capture(),
                                })
                            }
                        }
                    }
                    ExprType::Set(set, name, value) => {
                        let object = set.eval(ctx)?;
                        match object {
                            Literal::Instance(instance) => {
                                instance.borrow_mut().set(name, value.eval(ctx)?)
                            }
                            _ => {
                                Err(
                                    ExecError::new(
                                        "Only instances have fields".to_string(),
                                        self.range.clone(),
                                    ),
                                )
                            }
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
                                message: ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Expected {0} arguments but got {1}",
                                            callable.print_arity(),
                                            arguments.len(),
                                        ),
                                    );
                                    res
                                }),
                                range: self.callee.range.clone(),
                                backtrace: Backtrace::capture(),
                            });
                        }
                        callable.call(arguments, ctx)
                    }
                    Literal::Class(class) => {
                        let instance = Literal::Instance(
                            Rc::new(RefCell::new(LoxInstance::new(&class))),
                        );
                        let init = class.methods.get("init").cloned();
                        if let Some(init) = init {
                            let mut new_ctx = ctx.new_scope();
                            new_ctx.insert("this".to_string(), instance.clone());
                            init.body.eval(&mut new_ctx)?;
                        }
                        Ok(instance)
                    }
                    _ => {
                        Err(ExecError {
                            message: "Can only call functions and classes".to_string(),
                            range: self.callee.range.clone(),
                            backtrace: Backtrace::capture(),
                        })
                    }
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
                    UnaryType::Neg => {
                        match self.expr.eval(ctx)? {
                            Literal::Number(n) => Ok(Literal::Number(-n)),
                            _ => {
                                Err(ExecError {
                                    message: "Unary minus expects a number".to_string(),
                                    backtrace: Backtrace::capture(),
                                    range: self.expr.range.clone(),
                                })
                            }
                        }
                    }
                    UnaryType::Not => {
                        Ok(Literal::from(!bool::from(self.expr.eval(ctx)?)))
                    }
                }
            }
        }
        impl Eval for Binary {
            fn eval(&self, ctx: &mut EvalCtx) -> ExecResult<Literal> {
                match &self.operator {
                    Operator::Plus => {
                        match (self.left.eval(ctx)?, self.right.eval(ctx)?) {
                            (Literal::Number(l), Literal::Number(r)) => {
                                Ok(Literal::Number(l + r))
                            }
                            (Literal::String(l), Literal::String(r)) => {
                                Ok(
                                    Literal::String(
                                        ::alloc::__export::must_use({
                                            let res = ::alloc::fmt::format(
                                                format_args!("{0}{1}", l, r),
                                            );
                                            res
                                        }),
                                    ),
                                )
                            }
                            (Literal::String(l), other) => {
                                Ok(
                                    Literal::String(
                                        ::alloc::__export::must_use({
                                            let res = ::alloc::fmt::format(
                                                format_args!("{0}{1}", l, other),
                                            );
                                            res
                                        }),
                                    ),
                                )
                            }
                            (other, Literal::String(r)) => {
                                Ok(
                                    Literal::String(
                                        ::alloc::__export::must_use({
                                            let res = ::alloc::fmt::format(
                                                format_args!("{0}{1}", other, r),
                                            );
                                            res
                                        }),
                                    ),
                                )
                            }
                            _ => {
                                Err(ExecError {
                                    message: "Operands must be two numbers or two strings"
                                        .to_string(),
                                    range: self.left.range.merge(&self.right.range),
                                    backtrace: Backtrace::capture(),
                                })
                            }
                        }
                    }
                    Operator::Minus | Operator::Times | Operator::Div => {
                        match (self.left.eval(ctx)?, self.right.eval(ctx)?) {
                            (Literal::Number(l), Literal::Number(r)) => {
                                Ok(
                                    Literal::Number(
                                        match self.operator {
                                            Operator::Minus => l - r,
                                            Operator::Times => l * r,
                                            Operator::Div => l / r,
                                            _ => {
                                                ::core::panicking::panic(
                                                    "internal error: entered unreachable code",
                                                )
                                            }
                                        },
                                    ),
                                )
                            }
                            _ => {
                                Err(ExecError {
                                    message: "Operands must be numbers".to_string(),
                                    range: self.left.range.merge(&self.right.range),
                                    backtrace: Backtrace::capture(),
                                })
                            }
                        }
                    }
                    Operator::Greq
                    | Operator::Greater
                    | Operator::Leq
                    | Operator::Less => {
                        match (self.left.eval(ctx)?, self.right.eval(ctx)?) {
                            (Literal::Number(l), Literal::Number(r)) => {
                                Ok(
                                    Literal::from(
                                        match self.operator {
                                            Operator::Greq => l >= r,
                                            Operator::Greater => l > r,
                                            Operator::Leq => l <= r,
                                            Operator::Less => l < r,
                                            _ => {
                                                ::core::panicking::panic(
                                                    "internal error: entered unreachable code",
                                                )
                                            }
                                        },
                                    ),
                                )
                            }
                            _ => {
                                Err(ExecError {
                                    message: "Operands must be numbers".to_string(),
                                    range: self.left.range.merge(&self.right.range),
                                    backtrace: Backtrace::capture(),
                                })
                            }
                        }
                    }
                    Operator::EqualEqual | Operator::NEqualEqual => {
                        let left = self.left.eval(ctx)?;
                        let right = self.right.eval(ctx)?;
                        Ok(
                            match self.operator {
                                Operator::EqualEqual => Literal::from(left == right),
                                Operator::NEqualEqual => Literal::from(left != right),
                                _ => {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                }
                            },
                        )
                    }
                }
            }
        }
    }
    pub mod lexer {
        pub mod token {
            use std::fmt::Display;
            use strum::{Display, EnumString};
            use crate::interpreter::SouceCodeRange;
            #[allow(dead_code)]
            pub(crate) struct Token {
                pub(crate) inner: TokenType,
                pub(crate) lexeme: String,
                pub(crate) range: SouceCodeRange,
            }
            #[automatically_derived]
            #[allow(dead_code)]
            impl ::core::fmt::Debug for Token {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "Token",
                        "inner",
                        &self.inner,
                        "lexeme",
                        &self.lexeme,
                        "range",
                        &&self.range,
                    )
                }
            }
            #[automatically_derived]
            #[allow(dead_code)]
            impl ::core::clone::Clone for Token {
                #[inline]
                fn clone(&self) -> Token {
                    Token {
                        inner: ::core::clone::Clone::clone(&self.inner),
                        lexeme: ::core::clone::Clone::clone(&self.lexeme),
                        range: ::core::clone::Clone::clone(&self.range),
                    }
                }
            }
            impl Token {
                pub(crate) fn new(
                    inner: TokenType,
                    lexeme: String,
                    line: usize,
                    start_column: usize,
                    length: usize,
                ) -> Self {
                    Self {
                        inner,
                        lexeme,
                        range: SouceCodeRange {
                            line,
                            start_column,
                            length,
                        },
                    }
                }
            }
            #[allow(dead_code)]
            #[strum(serialize_all = "lowercase")]
            pub enum TokenType {
                #[strum(serialize = "(")]
                LeftParen,
                #[strum(serialize = ")")]
                RightParen,
                #[strum(serialize = "{")]
                LeftBrace,
                #[strum(serialize = "}")]
                RightBrace,
                #[strum(serialize = ",")]
                Comma,
                #[strum(serialize = ".")]
                Dot,
                #[strum(serialize = "-")]
                Minus,
                #[strum(serialize = "+")]
                Plus,
                #[strum(serialize = ";")]
                Semicolon,
                #[strum(serialize = "/")]
                Slash,
                #[strum(serialize = "*")]
                Star,
                #[strum(serialize = "!")]
                Bang,
                BangEqual,
                #[strum(serialize = "=")]
                Equal,
                EqualEqual,
                #[strum(serialize = ">")]
                Greater,
                GreaterEqual,
                #[strum(serialize = "<")]
                Less,
                LessEqual,
                #[strum(disabled)]
                Identifier(String),
                #[strum(disabled)]
                String(String),
                #[strum(disabled)]
                Number(f64),
                And,
                Break,
                Continue,
                Class,
                Else,
                False,
                Fun,
                For,
                If,
                Nil,
                Or,
                Print,
                Return,
                Super,
                This,
                True,
                Var,
                While,
                #[strum(disabled)]
                EOF,
            }
            #[automatically_derived]
            #[allow(dead_code)]
            impl ::core::fmt::Debug for TokenType {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        TokenType::LeftParen => {
                            ::core::fmt::Formatter::write_str(f, "LeftParen")
                        }
                        TokenType::RightParen => {
                            ::core::fmt::Formatter::write_str(f, "RightParen")
                        }
                        TokenType::LeftBrace => {
                            ::core::fmt::Formatter::write_str(f, "LeftBrace")
                        }
                        TokenType::RightBrace => {
                            ::core::fmt::Formatter::write_str(f, "RightBrace")
                        }
                        TokenType::Comma => ::core::fmt::Formatter::write_str(f, "Comma"),
                        TokenType::Dot => ::core::fmt::Formatter::write_str(f, "Dot"),
                        TokenType::Minus => ::core::fmt::Formatter::write_str(f, "Minus"),
                        TokenType::Plus => ::core::fmt::Formatter::write_str(f, "Plus"),
                        TokenType::Semicolon => {
                            ::core::fmt::Formatter::write_str(f, "Semicolon")
                        }
                        TokenType::Slash => ::core::fmt::Formatter::write_str(f, "Slash"),
                        TokenType::Star => ::core::fmt::Formatter::write_str(f, "Star"),
                        TokenType::Bang => ::core::fmt::Formatter::write_str(f, "Bang"),
                        TokenType::BangEqual => {
                            ::core::fmt::Formatter::write_str(f, "BangEqual")
                        }
                        TokenType::Equal => ::core::fmt::Formatter::write_str(f, "Equal"),
                        TokenType::EqualEqual => {
                            ::core::fmt::Formatter::write_str(f, "EqualEqual")
                        }
                        TokenType::Greater => {
                            ::core::fmt::Formatter::write_str(f, "Greater")
                        }
                        TokenType::GreaterEqual => {
                            ::core::fmt::Formatter::write_str(f, "GreaterEqual")
                        }
                        TokenType::Less => ::core::fmt::Formatter::write_str(f, "Less"),
                        TokenType::LessEqual => {
                            ::core::fmt::Formatter::write_str(f, "LessEqual")
                        }
                        TokenType::Identifier(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Identifier",
                                &__self_0,
                            )
                        }
                        TokenType::String(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "String",
                                &__self_0,
                            )
                        }
                        TokenType::Number(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Number",
                                &__self_0,
                            )
                        }
                        TokenType::And => ::core::fmt::Formatter::write_str(f, "And"),
                        TokenType::Break => ::core::fmt::Formatter::write_str(f, "Break"),
                        TokenType::Continue => {
                            ::core::fmt::Formatter::write_str(f, "Continue")
                        }
                        TokenType::Class => ::core::fmt::Formatter::write_str(f, "Class"),
                        TokenType::Else => ::core::fmt::Formatter::write_str(f, "Else"),
                        TokenType::False => ::core::fmt::Formatter::write_str(f, "False"),
                        TokenType::Fun => ::core::fmt::Formatter::write_str(f, "Fun"),
                        TokenType::For => ::core::fmt::Formatter::write_str(f, "For"),
                        TokenType::If => ::core::fmt::Formatter::write_str(f, "If"),
                        TokenType::Nil => ::core::fmt::Formatter::write_str(f, "Nil"),
                        TokenType::Or => ::core::fmt::Formatter::write_str(f, "Or"),
                        TokenType::Print => ::core::fmt::Formatter::write_str(f, "Print"),
                        TokenType::Return => {
                            ::core::fmt::Formatter::write_str(f, "Return")
                        }
                        TokenType::Super => ::core::fmt::Formatter::write_str(f, "Super"),
                        TokenType::This => ::core::fmt::Formatter::write_str(f, "This"),
                        TokenType::True => ::core::fmt::Formatter::write_str(f, "True"),
                        TokenType::Var => ::core::fmt::Formatter::write_str(f, "Var"),
                        TokenType::While => ::core::fmt::Formatter::write_str(f, "While"),
                        TokenType::EOF => ::core::fmt::Formatter::write_str(f, "EOF"),
                    }
                }
            }
            #[automatically_derived]
            #[allow(dead_code)]
            impl ::core::clone::Clone for TokenType {
                #[inline]
                fn clone(&self) -> TokenType {
                    match self {
                        TokenType::LeftParen => TokenType::LeftParen,
                        TokenType::RightParen => TokenType::RightParen,
                        TokenType::LeftBrace => TokenType::LeftBrace,
                        TokenType::RightBrace => TokenType::RightBrace,
                        TokenType::Comma => TokenType::Comma,
                        TokenType::Dot => TokenType::Dot,
                        TokenType::Minus => TokenType::Minus,
                        TokenType::Plus => TokenType::Plus,
                        TokenType::Semicolon => TokenType::Semicolon,
                        TokenType::Slash => TokenType::Slash,
                        TokenType::Star => TokenType::Star,
                        TokenType::Bang => TokenType::Bang,
                        TokenType::BangEqual => TokenType::BangEqual,
                        TokenType::Equal => TokenType::Equal,
                        TokenType::EqualEqual => TokenType::EqualEqual,
                        TokenType::Greater => TokenType::Greater,
                        TokenType::GreaterEqual => TokenType::GreaterEqual,
                        TokenType::Less => TokenType::Less,
                        TokenType::LessEqual => TokenType::LessEqual,
                        TokenType::Identifier(__self_0) => {
                            TokenType::Identifier(::core::clone::Clone::clone(__self_0))
                        }
                        TokenType::String(__self_0) => {
                            TokenType::String(::core::clone::Clone::clone(__self_0))
                        }
                        TokenType::Number(__self_0) => {
                            TokenType::Number(::core::clone::Clone::clone(__self_0))
                        }
                        TokenType::And => TokenType::And,
                        TokenType::Break => TokenType::Break,
                        TokenType::Continue => TokenType::Continue,
                        TokenType::Class => TokenType::Class,
                        TokenType::Else => TokenType::Else,
                        TokenType::False => TokenType::False,
                        TokenType::Fun => TokenType::Fun,
                        TokenType::For => TokenType::For,
                        TokenType::If => TokenType::If,
                        TokenType::Nil => TokenType::Nil,
                        TokenType::Or => TokenType::Or,
                        TokenType::Print => TokenType::Print,
                        TokenType::Return => TokenType::Return,
                        TokenType::Super => TokenType::Super,
                        TokenType::This => TokenType::This,
                        TokenType::True => TokenType::True,
                        TokenType::Var => TokenType::Var,
                        TokenType::While => TokenType::While,
                        TokenType::EOF => TokenType::EOF,
                    }
                }
            }
            #[automatically_derived]
            #[allow(dead_code)]
            impl ::core::marker::StructuralPartialEq for TokenType {}
            #[automatically_derived]
            #[allow(dead_code)]
            impl ::core::cmp::PartialEq for TokenType {
                #[inline]
                fn eq(&self, other: &TokenType) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                        && match (self, other) {
                            (
                                TokenType::Identifier(__self_0),
                                TokenType::Identifier(__arg1_0),
                            ) => __self_0 == __arg1_0,
                            (
                                TokenType::String(__self_0),
                                TokenType::String(__arg1_0),
                            ) => __self_0 == __arg1_0,
                            (
                                TokenType::Number(__self_0),
                                TokenType::Number(__arg1_0),
                            ) => __self_0 == __arg1_0,
                            _ => true,
                        }
                }
            }
            #[allow(clippy::use_self)]
            impl ::core::str::FromStr for TokenType {
                type Err = ::strum::ParseError;
                fn from_str(
                    s: &str,
                ) -> ::core::result::Result<
                    TokenType,
                    <Self as ::core::str::FromStr>::Err,
                > {
                    ::core::result::Result::Ok(
                        match s {
                            "(" => TokenType::LeftParen,
                            ")" => TokenType::RightParen,
                            "{" => TokenType::LeftBrace,
                            "}" => TokenType::RightBrace,
                            "," => TokenType::Comma,
                            "." => TokenType::Dot,
                            "-" => TokenType::Minus,
                            "+" => TokenType::Plus,
                            ";" => TokenType::Semicolon,
                            "/" => TokenType::Slash,
                            "*" => TokenType::Star,
                            "!" => TokenType::Bang,
                            "bangequal" => TokenType::BangEqual,
                            "=" => TokenType::Equal,
                            "equalequal" => TokenType::EqualEqual,
                            ">" => TokenType::Greater,
                            "greaterequal" => TokenType::GreaterEqual,
                            "<" => TokenType::Less,
                            "lessequal" => TokenType::LessEqual,
                            "and" => TokenType::And,
                            "break" => TokenType::Break,
                            "continue" => TokenType::Continue,
                            "class" => TokenType::Class,
                            "else" => TokenType::Else,
                            "false" => TokenType::False,
                            "fun" => TokenType::Fun,
                            "for" => TokenType::For,
                            "if" => TokenType::If,
                            "nil" => TokenType::Nil,
                            "or" => TokenType::Or,
                            "print" => TokenType::Print,
                            "return" => TokenType::Return,
                            "super" => TokenType::Super,
                            "this" => TokenType::This,
                            "true" => TokenType::True,
                            "var" => TokenType::Var,
                            "while" => TokenType::While,
                            _ => {
                                return ::core::result::Result::Err(
                                    ::strum::ParseError::VariantNotFound,
                                );
                            }
                        },
                    )
                }
            }
            #[allow(clippy::use_self)]
            impl ::core::convert::TryFrom<&str> for TokenType {
                type Error = ::strum::ParseError;
                fn try_from(
                    s: &str,
                ) -> ::core::result::Result<
                    TokenType,
                    <Self as ::core::convert::TryFrom<&str>>::Error,
                > {
                    ::core::str::FromStr::from_str(s)
                }
            }
            impl TryFrom<char> for TokenType {
                type Error = &'static str;
                fn try_from(value: char) -> Result<Self, Self::Error> {
                    match value {
                        '(' => Ok(Self::LeftParen),
                        ')' => Ok(Self::RightParen),
                        '{' => Ok(Self::LeftBrace),
                        '}' => Ok(Self::RightBrace),
                        ',' => Ok(Self::Comma),
                        '.' => Ok(Self::Dot),
                        '-' => Ok(Self::Minus),
                        '+' => Ok(Self::Plus),
                        ';' => Ok(Self::Semicolon),
                        '/' => Ok(Self::Slash),
                        '*' => Ok(Self::Star),
                        '!' => Ok(Self::Bang),
                        '=' => Ok(Self::Equal),
                        '<' => Ok(Self::Less),
                        '>' => Ok(Self::Greater),
                        _ => Err("Not parseable as a token type from char"),
                    }
                }
            }
            impl TokenType {
                pub(crate) fn is_binary(&self) -> bool {
                    match self {
                        TokenType::EqualEqual
                        | TokenType::BangEqual
                        | TokenType::Greater
                        | TokenType::GreaterEqual
                        | TokenType::Less
                        | TokenType::LessEqual
                        | TokenType::Plus
                        | TokenType::Minus
                        | TokenType::Star
                        | TokenType::Slash => true,
                        _ => false,
                    }
                }
            }
            impl Display for Token {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let literal = match &self.inner {
                        TokenType::String(s) => s.clone(),
                        TokenType::Number(n) => {
                            if n.fract() == 0.0 {
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(format_args!("{0:.1}", n));
                                    res
                                })
                            } else {
                                n.to_string()
                            }
                        }
                        _ => "null".to_string(),
                    };
                    f.write_fmt(
                        format_args!("{0} {1} {2}", self.inner, self.lexeme, literal),
                    )
                }
            }
        }
        use token::{Token, TokenType};
        pub fn tokenize(source: &str) -> Result<Vec<Token>, ()> {
            let mut had_error = false;
            let mut tokens = Vec::new();
            let chars = source.chars().collect::<Vec<char>>();
            let mut line = 0;
            let mut i = 0;
            while i < chars.len() {
                let char = chars[i];
                match char {
                    '(' | ')' | '{' | '}' | '*' | '.' | ',' | '+' | '-' | ';' => {
                        tokens
                            .push(
                                Token::new(
                                    match char.to_string().as_str() {
                                        tmp => {
                                            {
                                                ::std::io::_eprint(
                                                    format_args!(
                                                        "[{0}:{1}:{2}] {3} = {4:#?}\n",
                                                        "src/interpreter/lexer/mod.rs",
                                                        17u32,
                                                        21u32,
                                                        "char.to_string().as_str()",
                                                        &tmp,
                                                    ),
                                                );
                                            };
                                            tmp
                                        }
                                    }
                                        .try_into()
                                        .unwrap(),
                                    char.to_string(),
                                    line + 1,
                                    i + 1,
                                    1,
                                ),
                            );
                        i += 1;
                    }
                    '=' | '<' | '>' | '!' => {
                        let next_char = chars.get(i + 1);
                        if let Some(&next_char) = next_char {
                            let (len, token_type) = match (char, next_char) {
                                ('=', '=') => (2, TokenType::EqualEqual),
                                ('!', '=') => (2, TokenType::BangEqual),
                                ('<', '=') => (2, TokenType::LessEqual),
                                ('>', '=') => (2, TokenType::GreaterEqual),
                                ('=', _) | ('!', _) | ('<', _) | ('>', _) => {
                                    (1, char.try_into().unwrap())
                                }
                                _ => {
                                    {
                                        ::std::io::_eprint(
                                            format_args!(
                                                "[line {0}] Error: Unexpected character\n",
                                                line + 1,
                                            ),
                                        );
                                    };
                                    had_error = true;
                                    (0, TokenType::EOF)
                                }
                            };
                            tokens
                                .push(
                                    Token::new(
                                        token_type,
                                        chars[i..i + len].iter().collect(),
                                        line + 1,
                                        i + 1,
                                        len,
                                    ),
                                );
                            i += len;
                        } else {
                            let token_type = char.try_into().unwrap();
                            tokens
                                .push(
                                    Token::new(token_type, char.to_string(), line + 1, i + 1, 1),
                                );
                            i += 1;
                        }
                    }
                    '/' => {
                        let next_char = chars.get(i + 1);
                        if let Some(&next_char) = next_char {
                            if next_char == '/' {
                                while i < chars.len() && chars[i] != '\n' {
                                    i += 1;
                                }
                                continue;
                            }
                            if next_char == '*' {
                                let mut nestings = 1;
                                i += 2;
                                while i < chars.len() && nestings > 0 {
                                    let char = chars[i];
                                    match (char, chars.get(i + 1)) {
                                        ('*', Some('/')) => {
                                            nestings -= 1;
                                        }
                                        ('/', Some('*')) => {
                                            nestings += 1;
                                        }
                                        ('\n', _) => {
                                            line += 1;
                                        }
                                        _ => {}
                                    }
                                    i += 1;
                                }
                                i += 1;
                                continue;
                            }
                        }
                        tokens
                            .push(
                                Token::new(
                                    TokenType::Slash,
                                    char.to_string(),
                                    line + 1,
                                    i + 1,
                                    1,
                                ),
                            );
                        i += 1;
                    }
                    ' ' | '\t' | '\r' => {
                        i += 1;
                    }
                    '\n' => {
                        line += 1;
                        i += 1;
                    }
                    '"' => {
                        let mut j = i + 1;
                        while j < chars.len() && chars[j] != '"' {
                            if chars[j] == '\n' {
                                line += 1;
                            }
                            j += 1;
                        }
                        if j == chars.len() {
                            {
                                ::std::io::_eprint(
                                    format_args!(
                                        "[line {0}] Error: Unterminated string.\n",
                                        line + 1,
                                    ),
                                );
                            };
                            had_error = true;
                            break;
                        }
                        tokens
                            .push(
                                Token::new(
                                    TokenType::String(chars[i + 1..j].iter().collect()),
                                    chars[i..j + 1].iter().collect(),
                                    line + 1,
                                    i + 1,
                                    j - i + 1,
                                ),
                            );
                        i = j + 1;
                    }
                    '0'..='9' => {
                        let mut j = i + 1;
                        while j < chars.len() && chars[j].is_ascii_digit() {
                            j += 1;
                        }
                        if j < chars.len() && chars[j] == '.'
                            && chars.get(j + 1).map_or(false, |c| c.is_ascii_digit())
                        {
                            j += 1;
                            while j < chars.len() && chars[j].is_ascii_digit() {
                                j += 1;
                            }
                        }
                        let lexeme = chars[i..j].iter().collect::<String>();
                        let parsed_number = lexeme.parse::<f64>().unwrap();
                        tokens
                            .push(
                                Token::new(
                                    TokenType::Number(parsed_number),
                                    lexeme.to_string(),
                                    line + 1,
                                    i + 1,
                                    j - i,
                                ),
                            );
                        i = j;
                    }
                    char => {
                        if char.is_alphabetic() || char == '_' {
                            let mut j = i + 1;
                            while j < chars.len()
                                && (chars[j].is_alphanumeric() || chars[j] == '_')
                            {
                                j += 1;
                            }
                            let lexeme = chars[i..j].iter().collect::<String>();
                            let token_type = lexeme
                                .as_str()
                                .try_into()
                                .unwrap_or(TokenType::Identifier(lexeme.clone()));
                            tokens
                                .push(
                                    Token::new(token_type, lexeme, line + 1, i + 1, j - i),
                                );
                            i = j;
                        } else {
                            {
                                ::std::io::_eprint(
                                    format_args!(
                                        "[line {0}] Error: Unexpected character: {1}\n",
                                        line + 1,
                                        char,
                                    ),
                                );
                            };
                            had_error = true;
                            i += 1;
                        }
                    }
                }
            }
            tokens.push(Token::new(TokenType::EOF, String::new(), 0, 0, 0));
            if had_error { Err(()) } else { Ok(tokens) }
        }
    }
    mod parser {
        use std::{backtrace::Backtrace, fmt::Display};
        use super::token::{Token, TokenType};
        pub(crate) mod ast {
            use std::{cell::RefCell, fmt::Display, rc::Rc};
            use crate::interpreter::{
                eval::{lox_class::LoxClass, lox_instance::LoxInstance, LoxCallable},
                token::TokenType, SouceCodeRange,
            };
            pub(crate) struct Stmt {
                pub intern: StmtType,
                pub range: SouceCodeRange,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Stmt {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Stmt",
                        "intern",
                        &self.intern,
                        "range",
                        &&self.range,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Stmt {
                #[inline]
                fn clone(&self) -> Stmt {
                    Stmt {
                        intern: ::core::clone::Clone::clone(&self.intern),
                        range: ::core::clone::Clone::clone(&self.range),
                    }
                }
            }
            impl Display for Stmt {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.intern))
                }
            }
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
            #[automatically_derived]
            impl ::core::fmt::Debug for StmtType {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        StmtType::Expr(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Expr",
                                &__self_0,
                            )
                        }
                        StmtType::IfStmt(__self_0, __self_1, __self_2) => {
                            ::core::fmt::Formatter::debug_tuple_field3_finish(
                                f,
                                "IfStmt",
                                __self_0,
                                __self_1,
                                &__self_2,
                            )
                        }
                        StmtType::Print(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Print",
                                &__self_0,
                            )
                        }
                        StmtType::Return(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Return",
                                &__self_0,
                            )
                        }
                        StmtType::Var(__self_0, __self_1) => {
                            ::core::fmt::Formatter::debug_tuple_field2_finish(
                                f,
                                "Var",
                                __self_0,
                                &__self_1,
                            )
                        }
                        StmtType::While(__self_0, __self_1) => {
                            ::core::fmt::Formatter::debug_tuple_field2_finish(
                                f,
                                "While",
                                __self_0,
                                &__self_1,
                            )
                        }
                        StmtType::Block(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Block",
                                &__self_0,
                            )
                        }
                        StmtType::Break => ::core::fmt::Formatter::write_str(f, "Break"),
                        StmtType::Continue => {
                            ::core::fmt::Formatter::write_str(f, "Continue")
                        }
                        StmtType::Function(__self_0, __self_1, __self_2, __self_3) => {
                            ::core::fmt::Formatter::debug_tuple_field4_finish(
                                f,
                                "Function",
                                __self_0,
                                __self_1,
                                __self_2,
                                &__self_3,
                            )
                        }
                        StmtType::Class(__self_0, __self_1) => {
                            ::core::fmt::Formatter::debug_tuple_field2_finish(
                                f,
                                "Class",
                                __self_0,
                                &__self_1,
                            )
                        }
                    }
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for StmtType {
                #[inline]
                fn clone(&self) -> StmtType {
                    match self {
                        StmtType::Expr(__self_0) => {
                            StmtType::Expr(::core::clone::Clone::clone(__self_0))
                        }
                        StmtType::IfStmt(__self_0, __self_1, __self_2) => {
                            StmtType::IfStmt(
                                ::core::clone::Clone::clone(__self_0),
                                ::core::clone::Clone::clone(__self_1),
                                ::core::clone::Clone::clone(__self_2),
                            )
                        }
                        StmtType::Print(__self_0) => {
                            StmtType::Print(::core::clone::Clone::clone(__self_0))
                        }
                        StmtType::Return(__self_0) => {
                            StmtType::Return(::core::clone::Clone::clone(__self_0))
                        }
                        StmtType::Var(__self_0, __self_1) => {
                            StmtType::Var(
                                ::core::clone::Clone::clone(__self_0),
                                ::core::clone::Clone::clone(__self_1),
                            )
                        }
                        StmtType::While(__self_0, __self_1) => {
                            StmtType::While(
                                ::core::clone::Clone::clone(__self_0),
                                ::core::clone::Clone::clone(__self_1),
                            )
                        }
                        StmtType::Block(__self_0) => {
                            StmtType::Block(::core::clone::Clone::clone(__self_0))
                        }
                        StmtType::Break => StmtType::Break,
                        StmtType::Continue => StmtType::Continue,
                        StmtType::Function(__self_0, __self_1, __self_2, __self_3) => {
                            StmtType::Function(
                                ::core::clone::Clone::clone(__self_0),
                                ::core::clone::Clone::clone(__self_1),
                                ::core::clone::Clone::clone(__self_2),
                                ::core::clone::Clone::clone(__self_3),
                            )
                        }
                        StmtType::Class(__self_0, __self_1) => {
                            StmtType::Class(
                                ::core::clone::Clone::clone(__self_0),
                                ::core::clone::Clone::clone(__self_1),
                            )
                        }
                    }
                }
            }
            impl Display for StmtType {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        StmtType::Expr(expr) => f.write_fmt(format_args!("{0}", expr)),
                        StmtType::Print(expr) => {
                            f.write_fmt(format_args!("(print {0})", expr))
                        }
                        StmtType::Return(expr) => {
                            f.write_fmt(format_args!("(return {0})", expr))
                        }
                        StmtType::Var(name, initializer) => {
                            match initializer {
                                Some(initializer) => {
                                    f.write_fmt(
                                        format_args!("(var {0} = {1})", name, initializer),
                                    )
                                }
                                None => f.write_fmt(format_args!("(var {0})", name)),
                            }
                        }
                        StmtType::IfStmt(condition, then_branch, else_branch) => {
                            let mut result = String::new();
                            result.push_str("(if ");
                            result
                                .push_str(
                                    &::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!("{0} ", condition),
                                        );
                                        res
                                    }),
                                );
                            result
                                .push_str(
                                    &::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!("{0} ", then_branch),
                                        );
                                        res
                                    }),
                                );
                            if let Some(else_branch) = else_branch {
                                result
                                    .push_str(
                                        &::alloc::__export::must_use({
                                            let res = ::alloc::fmt::format(
                                                format_args!("{0} ", else_branch),
                                            );
                                            res
                                        }),
                                    );
                            }
                            result.push_str(")");
                            f.write_fmt(format_args!("{0}", result))
                        }
                        StmtType::Block(stmts) => {
                            let mut result = String::new();
                            result.push_str("{\n");
                            for stmt in stmts {
                                result
                                    .push_str(
                                        &::alloc::__export::must_use({
                                            let res = ::alloc::fmt::format(format_args!("{0}\n", stmt));
                                            res
                                        }),
                                    );
                            }
                            result.push_str("}");
                            f.write_fmt(format_args!("{0}", result))
                        }
                        StmtType::While(expr, stmt) => {
                            f.write_fmt(format_args!("(while {0} {1})", expr, stmt))
                        }
                        StmtType::Break => f.write_fmt(format_args!("break")),
                        StmtType::Continue => f.write_fmt(format_args!("continue")),
                        StmtType::Function(function, name, ..) => {
                            f.write_fmt(format_args!("{0} {1}", function.tipe(), name))
                        }
                        StmtType::Class(name, methods) => {
                            let mut result = String::new();
                            result
                                .push_str(
                                    &::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!("(class {0} ", name),
                                        );
                                        res
                                    }),
                                );
                            for method in methods {
                                result
                                    .push_str(
                                        &::alloc::__export::must_use({
                                            let res = ::alloc::fmt::format(
                                                format_args!("{0} ", method),
                                            );
                                            res
                                        }),
                                    );
                            }
                            result.push_str(")");
                            f.write_fmt(format_args!("{0}", result))
                        }
                    }
                }
            }
            pub(crate) struct ExprId(pub usize);
            #[automatically_derived]
            impl ::core::fmt::Debug for ExprId {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ExprId",
                        &&self.0,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ExprId {
                #[inline]
                fn clone(&self) -> ExprId {
                    let _: ::core::clone::AssertParamIsClone<usize>;
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for ExprId {}
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ExprId {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ExprId {
                #[inline]
                fn eq(&self, other: &ExprId) -> bool {
                    self.0 == other.0
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ExprId {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<usize>;
                }
            }
            #[automatically_derived]
            impl ::core::hash::Hash for ExprId {
                #[inline]
                fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                    ::core::hash::Hash::hash(&self.0, state)
                }
            }
            impl Display for ExprId {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.0))
                }
            }
            pub(crate) struct Expr {
                pub intern: Box<ExprType>,
                pub range: SouceCodeRange,
                pub id: ExprId,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Expr {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "Expr",
                        "intern",
                        &self.intern,
                        "range",
                        &self.range,
                        "id",
                        &&self.id,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Expr {
                #[inline]
                fn clone(&self) -> Expr {
                    Expr {
                        intern: ::core::clone::Clone::clone(&self.intern),
                        range: ::core::clone::Clone::clone(&self.range),
                        id: ::core::clone::Clone::clone(&self.id),
                    }
                }
            }
            impl Display for Expr {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{0}", self.intern))
                }
            }
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
            #[automatically_derived]
            impl ::core::fmt::Debug for ExprType {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        ExprType::Literal(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Literal",
                                &__self_0,
                            )
                        }
                        ExprType::Grouping(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Grouping",
                                &__self_0,
                            )
                        }
                        ExprType::Unary(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Unary",
                                &__self_0,
                            )
                        }
                        ExprType::Binary(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Binary",
                                &__self_0,
                            )
                        }
                        ExprType::Logical(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Logical",
                                &__self_0,
                            )
                        }
                        ExprType::Variable(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Variable",
                                &__self_0,
                            )
                        }
                        ExprType::Assign(__self_0, __self_1) => {
                            ::core::fmt::Formatter::debug_tuple_field2_finish(
                                f,
                                "Assign",
                                __self_0,
                                &__self_1,
                            )
                        }
                        ExprType::Call(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Call",
                                &__self_0,
                            )
                        }
                        ExprType::Get(__self_0, __self_1) => {
                            ::core::fmt::Formatter::debug_tuple_field2_finish(
                                f,
                                "Get",
                                __self_0,
                                &__self_1,
                            )
                        }
                        ExprType::Set(__self_0, __self_1, __self_2) => {
                            ::core::fmt::Formatter::debug_tuple_field3_finish(
                                f,
                                "Set",
                                __self_0,
                                __self_1,
                                &__self_2,
                            )
                        }
                    }
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ExprType {
                #[inline]
                fn clone(&self) -> ExprType {
                    match self {
                        ExprType::Literal(__self_0) => {
                            ExprType::Literal(::core::clone::Clone::clone(__self_0))
                        }
                        ExprType::Grouping(__self_0) => {
                            ExprType::Grouping(::core::clone::Clone::clone(__self_0))
                        }
                        ExprType::Unary(__self_0) => {
                            ExprType::Unary(::core::clone::Clone::clone(__self_0))
                        }
                        ExprType::Binary(__self_0) => {
                            ExprType::Binary(::core::clone::Clone::clone(__self_0))
                        }
                        ExprType::Logical(__self_0) => {
                            ExprType::Logical(::core::clone::Clone::clone(__self_0))
                        }
                        ExprType::Variable(__self_0) => {
                            ExprType::Variable(::core::clone::Clone::clone(__self_0))
                        }
                        ExprType::Assign(__self_0, __self_1) => {
                            ExprType::Assign(
                                ::core::clone::Clone::clone(__self_0),
                                ::core::clone::Clone::clone(__self_1),
                            )
                        }
                        ExprType::Call(__self_0) => {
                            ExprType::Call(::core::clone::Clone::clone(__self_0))
                        }
                        ExprType::Get(__self_0, __self_1) => {
                            ExprType::Get(
                                ::core::clone::Clone::clone(__self_0),
                                ::core::clone::Clone::clone(__self_1),
                            )
                        }
                        ExprType::Set(__self_0, __self_1, __self_2) => {
                            ExprType::Set(
                                ::core::clone::Clone::clone(__self_0),
                                ::core::clone::Clone::clone(__self_1),
                                ::core::clone::Clone::clone(__self_2),
                            )
                        }
                    }
                }
            }
            impl Display for ExprType {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        ExprType::Literal(literal) => {
                            f.write_fmt(format_args!("{0:?}", literal))
                        }
                        ExprType::Grouping(expression) => {
                            f.write_fmt(format_args!("(group {0})", expression))
                        }
                        ExprType::Unary(unary) => f.write_fmt(format_args!("{0}", unary)),
                        ExprType::Binary(binary) => {
                            f.write_fmt(format_args!("{0}", binary))
                        }
                        ExprType::Variable(name) => {
                            f.write_fmt(format_args!("{0}", name))
                        }
                        ExprType::Assign(name, expr) => {
                            f.write_fmt(format_args!("(assign {0} {1})", name, expr))
                        }
                        ExprType::Logical(logical) => {
                            f.write_fmt(format_args!("{0}", logical))
                        }
                        ExprType::Call(call) => f.write_fmt(format_args!("{0}", call)),
                        ExprType::Get(expr, name) => {
                            f.write_fmt(format_args!("(get {0} {1})", expr, name))
                        }
                        ExprType::Set(expr, name, value) => {
                            f.write_fmt(
                                format_args!("(set {0} {1} {2})", expr, name, value),
                            )
                        }
                    }
                }
            }
            pub(crate) enum FunctionType {
                /// Name, parameters, body
                Function,
                Method,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for FunctionType {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            FunctionType::Function => "Function",
                            FunctionType::Method => "Method",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for FunctionType {
                #[inline]
                fn clone(&self) -> FunctionType {
                    match self {
                        FunctionType::Function => FunctionType::Function,
                        FunctionType::Method => FunctionType::Method,
                    }
                }
            }
            impl FunctionType {
                pub fn tipe(&self) -> String {
                    match self {
                        FunctionType::Function => "function".to_string(),
                        FunctionType::Method => "method".to_string(),
                    }
                }
            }
            pub(crate) struct Call {
                pub callee: Expr,
                pub arguments: Vec<Expr>,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Call {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Call",
                        "callee",
                        &self.callee,
                        "arguments",
                        &&self.arguments,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Call {
                #[inline]
                fn clone(&self) -> Call {
                    Call {
                        callee: ::core::clone::Clone::clone(&self.callee),
                        arguments: ::core::clone::Clone::clone(&self.arguments),
                    }
                }
            }
            impl Display for Call {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let mut result = String::new();
                    result
                        .push_str(
                            &::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!("(call {0} ", self.callee),
                                );
                                res
                            }),
                        );
                    for arg in &self.arguments {
                        result
                            .push_str(
                                &::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(format_args!("{0} ", arg));
                                    res
                                }),
                            );
                    }
                    result.push_str(")");
                    f.write_fmt(format_args!("{0}", result))
                }
            }
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
            #[automatically_derived]
            impl ::core::clone::Clone for Literal {
                #[inline]
                fn clone(&self) -> Literal {
                    match self {
                        Literal::Number(__self_0) => {
                            Literal::Number(::core::clone::Clone::clone(__self_0))
                        }
                        Literal::String(__self_0) => {
                            Literal::String(::core::clone::Clone::clone(__self_0))
                        }
                        Literal::True => Literal::True,
                        Literal::False => Literal::False,
                        Literal::Nil => Literal::Nil,
                        Literal::Callable(__self_0) => {
                            Literal::Callable(::core::clone::Clone::clone(__self_0))
                        }
                        Literal::Class(__self_0) => {
                            Literal::Class(::core::clone::Clone::clone(__self_0))
                        }
                        Literal::Instance(__self_0) => {
                            Literal::Instance(::core::clone::Clone::clone(__self_0))
                        }
                    }
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Literal {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Literal {
                #[inline]
                fn eq(&self, other: &Literal) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                        && match (self, other) {
                            (Literal::Number(__self_0), Literal::Number(__arg1_0)) => {
                                __self_0 == __arg1_0
                            }
                            (Literal::String(__self_0), Literal::String(__arg1_0)) => {
                                __self_0 == __arg1_0
                            }
                            (
                                Literal::Callable(__self_0),
                                Literal::Callable(__arg1_0),
                            ) => __self_0 == __arg1_0,
                            (Literal::Class(__self_0), Literal::Class(__arg1_0)) => {
                                __self_0 == __arg1_0
                            }
                            (
                                Literal::Instance(__self_0),
                                Literal::Instance(__arg1_0),
                            ) => __self_0 == __arg1_0,
                            _ => true,
                        }
                }
            }
            #[automatically_derived]
            impl ::core::default::Default for Literal {
                #[inline]
                fn default() -> Literal {
                    Self::Nil
                }
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
                        Literal::Class(_) => true,
                        Literal::Instance(_) => true,
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
                                f.write_fmt(format_args!("{0:.1}", n))
                            } else {
                                f.write_fmt(format_args!("{0}", n))
                            }
                        }
                        Literal::String(s) => f.write_fmt(format_args!("{0}", s)),
                        Literal::True => f.write_fmt(format_args!("true")),
                        Literal::False => f.write_fmt(format_args!("false")),
                        Literal::Nil => f.write_fmt(format_args!("nil")),
                        Literal::Callable(lox_callable) => {
                            f.write_fmt(format_args!("{0:?}", lox_callable))
                        }
                        Literal::Class(lox_class) => {
                            f.write_fmt(format_args!("{0:?}", lox_class))
                        }
                        Literal::Instance(lox_instance) => {
                            f.write_fmt(format_args!("{0:?}", lox_instance))
                        }
                    }
                }
            }
            impl Display for Literal {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        Literal::Number(n) => f.write_fmt(format_args!("{0}", n)),
                        Literal::String(s) => f.write_fmt(format_args!("{0}", s)),
                        Literal::True => f.write_fmt(format_args!("true")),
                        Literal::False => f.write_fmt(format_args!("false")),
                        Literal::Nil => f.write_fmt(format_args!("nil")),
                        Literal::Callable(lox_callable) => {
                            f.write_fmt(format_args!("{0}", lox_callable))
                        }
                        Literal::Class(lox_class) => {
                            f.write_fmt(format_args!("{0}", lox_class))
                        }
                        Literal::Instance(lox_instance) => {
                            f.write_fmt(format_args!("{0}", lox_instance.borrow()))
                        }
                    }
                }
            }
            pub(crate) struct Unary {
                pub intern: UnaryType,
                pub expr: Expr,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Unary {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Unary",
                        "intern",
                        &self.intern,
                        "expr",
                        &&self.expr,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Unary {
                #[inline]
                fn clone(&self) -> Unary {
                    Unary {
                        intern: ::core::clone::Clone::clone(&self.intern),
                        expr: ::core::clone::Clone::clone(&self.expr),
                    }
                }
            }
            impl Display for Unary {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("({0} {1})", self.intern, self.expr))
                }
            }
            pub(crate) enum UnaryType {
                Not,
                Neg,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for UnaryType {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            UnaryType::Not => "Not",
                            UnaryType::Neg => "Neg",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for UnaryType {
                #[inline]
                fn clone(&self) -> UnaryType {
                    match self {
                        UnaryType::Not => UnaryType::Not,
                        UnaryType::Neg => UnaryType::Neg,
                    }
                }
            }
            impl From<&TokenType> for UnaryType {
                fn from(token: &TokenType) -> Self {
                    match token {
                        TokenType::Bang => UnaryType::Not,
                        TokenType::Minus => UnaryType::Neg,
                        _ => {
                            ::core::panicking::panic_fmt(
                                format_args!("Invalid unary operator"),
                            );
                        }
                    }
                }
            }
            impl Display for UnaryType {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        UnaryType::Not => f.write_fmt(format_args!("!")),
                        UnaryType::Neg => f.write_fmt(format_args!("-")),
                    }
                }
            }
            pub(crate) struct Binary {
                pub left: Expr,
                pub operator: Operator,
                pub right: Expr,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Binary {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "Binary",
                        "left",
                        &self.left,
                        "operator",
                        &self.operator,
                        "right",
                        &&self.right,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Binary {
                #[inline]
                fn clone(&self) -> Binary {
                    Binary {
                        left: ::core::clone::Clone::clone(&self.left),
                        operator: ::core::clone::Clone::clone(&self.operator),
                        right: ::core::clone::Clone::clone(&self.right),
                    }
                }
            }
            impl Display for Binary {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(
                        format_args!(
                            "({0} {1} {2})",
                            self.operator,
                            self.left,
                            self.right,
                        ),
                    )
                }
            }
            pub(crate) struct Logical {
                pub left: Expr,
                pub operator: LogicalOperator,
                pub right: Expr,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Logical {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "Logical",
                        "left",
                        &self.left,
                        "operator",
                        &self.operator,
                        "right",
                        &&self.right,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Logical {
                #[inline]
                fn clone(&self) -> Logical {
                    Logical {
                        left: ::core::clone::Clone::clone(&self.left),
                        operator: ::core::clone::Clone::clone(&self.operator),
                        right: ::core::clone::Clone::clone(&self.right),
                    }
                }
            }
            impl Display for Logical {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(
                        format_args!(
                            "({0} {1} {2})",
                            self.operator,
                            self.left,
                            self.right,
                        ),
                    )
                }
            }
            pub(crate) enum LogicalOperator {
                And,
                Or,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for LogicalOperator {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            LogicalOperator::And => "And",
                            LogicalOperator::Or => "Or",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for LogicalOperator {
                #[inline]
                fn clone(&self) -> LogicalOperator {
                    match self {
                        LogicalOperator::And => LogicalOperator::And,
                        LogicalOperator::Or => LogicalOperator::Or,
                    }
                }
            }
            impl From<&TokenType> for LogicalOperator {
                fn from(token: &TokenType) -> Self {
                    match token {
                        TokenType::And => LogicalOperator::And,
                        TokenType::Or => LogicalOperator::Or,
                        _ => {
                            ::core::panicking::panic_fmt(
                                format_args!("Invalid logical operator"),
                            );
                        }
                    }
                }
            }
            impl Display for LogicalOperator {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        LogicalOperator::And => f.write_fmt(format_args!("and")),
                        LogicalOperator::Or => f.write_fmt(format_args!("or")),
                    }
                }
            }
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
            #[automatically_derived]
            impl ::core::fmt::Debug for Operator {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            Operator::EqualEqual => "EqualEqual",
                            Operator::NEqualEqual => "NEqualEqual",
                            Operator::Less => "Less",
                            Operator::Leq => "Leq",
                            Operator::Greater => "Greater",
                            Operator::Greq => "Greq",
                            Operator::Plus => "Plus",
                            Operator::Minus => "Minus",
                            Operator::Times => "Times",
                            Operator::Div => "Div",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Operator {
                #[inline]
                fn clone(&self) -> Operator {
                    match self {
                        Operator::EqualEqual => Operator::EqualEqual,
                        Operator::NEqualEqual => Operator::NEqualEqual,
                        Operator::Less => Operator::Less,
                        Operator::Leq => Operator::Leq,
                        Operator::Greater => Operator::Greater,
                        Operator::Greq => Operator::Greq,
                        Operator::Plus => Operator::Plus,
                        Operator::Minus => Operator::Minus,
                        Operator::Times => Operator::Times,
                        Operator::Div => Operator::Div,
                    }
                }
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
                        tok => {
                            ::core::panicking::panic_fmt(
                                format_args!("Invalid operator {0:?}", tok),
                            );
                        }
                    }
                }
            }
            impl Display for Operator {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        Operator::EqualEqual => f.write_fmt(format_args!("==")),
                        Operator::NEqualEqual => f.write_fmt(format_args!("!=")),
                        Operator::Less => f.write_fmt(format_args!("<")),
                        Operator::Leq => f.write_fmt(format_args!("<=")),
                        Operator::Greater => f.write_fmt(format_args!(">")),
                        Operator::Greq => f.write_fmt(format_args!(">=")),
                        Operator::Plus => f.write_fmt(format_args!("+")),
                        Operator::Minus => f.write_fmt(format_args!("-")),
                        Operator::Times => f.write_fmt(format_args!("*")),
                        Operator::Div => f.write_fmt(format_args!("/")),
                    }
                }
            }
        }
        use ast::*;
        pub struct ParserError {
            pub(crate) message: String,
            pub(crate) token: Token,
            #[allow(dead_code)]
            pub(crate) backtrace: Backtrace,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ParserError {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "ParserError",
                    "message",
                    &self.message,
                    "token",
                    &self.token,
                    "backtrace",
                    &&self.backtrace,
                )
            }
        }
        impl Display for ParserError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(
                    format_args!("{0} at line {1}", self.message, self.token.range.line),
                )
            }
        }
        type Result<T> = std::result::Result<T, ParserError>;
        struct ExprIdCounter {
            counter: usize,
        }
        impl ExprIdCounter {
            fn new() -> Self {
                Self { counter: 0 }
            }
            fn next(&mut self) -> ExprId {
                self.counter += 1;
                return ExprId(self.counter);
            }
        }
        pub struct ParserInstance {
            pub current: usize,
            pub had_error: bool,
            pub tokens: Vec<Token>,
            exp_id_counter: ExprIdCounter,
        }
        impl ParserInstance {
            #[allow(dead_code)]
            pub fn print_remaining(&self) {
                {
                    ::std::io::_print(
                        format_args!(
                            "Tokens left: {0:?}\n",
                            self.tokens[self.current..].iter().collect::<Vec<_>>(),
                        ),
                    );
                };
            }
            fn error(&mut self, token: &Token, message: &str) {
                match token.inner {
                    TokenType::EOF => self.report(token.range.line, " at end", message),
                    _ => self.report(token.range.line, "", message),
                }
            }
            fn synchronize(&mut self) {
                self.advance();
                while !self.is_at_end() {
                    if self.previous().inner == TokenType::Semicolon {
                        return;
                    }
                    match self.peek().inner {
                        TokenType::Class
                        | TokenType::Fun
                        | TokenType::Var
                        | TokenType::For
                        | TokenType::If
                        | TokenType::While
                        | TokenType::Print
                        | TokenType::Return => return,
                        _ => {}
                    }
                    self.advance();
                }
            }
            fn report(&mut self, line: usize, where_: &str, message: &str) {
                {
                    ::std::io::_eprint(
                        format_args!("[line {0}] Error{1}: {2}\n", line, where_, message),
                    );
                };
                self.had_error = true;
            }
            pub fn new(tokens: Vec<Token>) -> Self {
                Self {
                    current: 0,
                    tokens,
                    had_error: false,
                    exp_id_counter: ExprIdCounter::new(),
                }
            }
            pub fn parse_expr(&mut self) -> Result<Expr> {
                match self.expression() {
                    Ok(expr) => Ok(expr),
                    Err(err) => {
                        self.error(&err.token, &err.message);
                        self.synchronize();
                        return Err(err);
                    }
                }
            }
            pub fn parse(&mut self) -> std::result::Result<Vec<Stmt>, ()> {
                let mut statements = Vec::new();
                while !self.is_at_end() {
                    let declaration = self.declaration();
                    match declaration {
                        Ok(declaration) => statements.push(declaration),
                        Err(err) => {
                            self.error(&err.token, &err.message);
                            self.synchronize();
                        }
                    }
                }
                if self.had_error {
                    return Err(());
                } else {
                    return Ok(statements);
                }
            }
            fn consume(&mut self, tipe: TokenType, message: &str) -> Result<&Token> {
                if self.check(tipe) {
                    return Ok(self.advance());
                }
                let token = self.peek().to_owned();
                self.error(&token, message);
                Err(ParserError {
                    message: "Parse error".to_string(),
                    token,
                    backtrace: Backtrace::force_capture(),
                })
            }
            fn declaration(&mut self) -> Result<Stmt> {
                if self
                    .mtch(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([TokenType::Class]),
                        ),
                    )
                {
                    return self.class_declaration();
                }
                if self
                    .mtch(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([TokenType::Fun]),
                        ),
                    )
                {
                    return self.function(FunctionType::Function);
                }
                if self
                    .mtch(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([TokenType::Var]),
                        ),
                    )
                {
                    return self.var_declaration();
                }
                return self.statement();
            }
            fn class_declaration(&mut self) -> Result<Stmt> {
                let name = match self.peek().inner {
                    TokenType::Identifier(_) => self.advance(),
                    _ => {
                        return Err(ParserError {
                            message: "Expect class name.".to_string(),
                            token: self.peek().to_owned(),
                            backtrace: Backtrace::force_capture(),
                        });
                    }
                };
                let range = name.range.clone();
                let name = if let TokenType::Identifier(name) = &name.inner {
                    name.clone()
                } else {
                    ::core::panicking::panic("internal error: entered unreachable code")
                };
                self.consume(
                    TokenType::LeftBrace,
                    "Expect '{' before class body.".to_string().as_str(),
                )?;
                let mut methods = Vec::new();
                while !self.check(TokenType::RightBrace) && !self.is_at_end() {
                    methods.push(self.function(FunctionType::Method)?);
                }
                self.consume(
                    TokenType::RightBrace,
                    "Expect '}' after class body.".to_string().as_str(),
                )?;
                return Ok(Stmt {
                    range: range.merge(&self.previous().range),
                    intern: StmtType::Class(name, methods),
                });
            }
            fn function(&mut self, kind: FunctionType) -> Result<Stmt> {
                let name = match self.peek().inner {
                    TokenType::Identifier(_) => self.advance(),
                    _ => {
                        return Err(ParserError {
                            message: ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!("Expect {0} name.", kind.tipe()),
                                );
                                res
                            }),
                            token: self.peek().to_owned(),
                            backtrace: Backtrace::force_capture(),
                        });
                    }
                };
                let range = name.range.clone();
                let name = if let TokenType::Identifier(name) = &name.inner {
                    name.clone()
                } else {
                    ::core::panicking::panic("internal error: entered unreachable code")
                };
                self.consume(
                    TokenType::LeftParen,
                    ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("Expect \'(\' after {0} name.", kind.tipe()),
                            );
                            res
                        })
                        .as_str(),
                )?;
                let mut parameters = Vec::new();
                if !self.check(TokenType::RightParen) {
                    loop {
                        if parameters.len() >= 255 {
                            self.error(
                                &self.peek().clone(),
                                "Cannot have more than 255 parameters.",
                            );
                        }
                        let param = match self.peek().inner {
                            TokenType::Identifier(_) => self.advance(),
                            _ => {
                                return Err(ParserError {
                                    message: "Expect parameter name.".to_string(),
                                    token: self.peek().to_owned(),
                                    backtrace: Backtrace::force_capture(),
                                });
                            }
                        };
                        let param = if let TokenType::Identifier(param) = &param.inner {
                            param.clone()
                        } else {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        };
                        parameters.push(param);
                        if !self
                            .mtch(
                                <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([TokenType::Comma]),
                                ),
                            )
                        {
                            break;
                        }
                    }
                }
                self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;
                self.consume(
                    TokenType::LeftBrace,
                    ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("Expect \'{{\' before {0} body.", kind.tipe()),
                            );
                            res
                        })
                        .as_str(),
                )?;
                let body = self.block_statement()?;
                return Ok(Stmt {
                    range: range.merge(&body.range),
                    intern: StmtType::Function(kind, name, parameters, Box::new(body)),
                });
            }
            fn var_declaration(&mut self) -> Result<Stmt> {
                let name = match self.peek().inner {
                    TokenType::Identifier(_) => self.advance(),
                    _ => {
                        return Err(ParserError {
                            message: "Expect variable name.".to_string(),
                            token: self.peek().to_owned(),
                            backtrace: Backtrace::force_capture(),
                        });
                    }
                };
                let range = name.range.clone();
                let name = if let TokenType::Identifier(name) = &name.inner {
                    name.clone()
                } else {
                    ::core::panicking::panic("internal error: entered unreachable code")
                };
                let initializer = if self
                    .mtch(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([TokenType::Equal]),
                        ),
                    )
                {
                    Some(self.expression()?)
                } else {
                    None
                };
                self.consume(
                    TokenType::Semicolon,
                    "Expect ';' after variable declaration.",
                )?;
                return Ok(Stmt {
                    range,
                    intern: StmtType::Var(name, initializer),
                });
            }
            fn statement(&mut self) -> Result<Stmt> {
                if self
                    .mtch(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([TokenType::If]),
                        ),
                    )
                {
                    return self.if_statement();
                }
                if self
                    .mtch(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([TokenType::Print]),
                        ),
                    )
                {
                    return self.print_statement();
                }
                if self
                    .mtch(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([TokenType::Return]),
                        ),
                    )
                {
                    return self.return_statement();
                }
                if self
                    .mtch(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([TokenType::For]),
                        ),
                    )
                {
                    return self.for_statement();
                }
                if self
                    .mtch(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([TokenType::While]),
                        ),
                    )
                {
                    return self.while_statement();
                }
                if self
                    .mtch(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([TokenType::LeftBrace]),
                        ),
                    )
                {
                    return self.block_statement();
                }
                if self
                    .mtch(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([TokenType::Break]),
                        ),
                    )
                {
                    return self.break_statement();
                }
                if self
                    .mtch(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([TokenType::Continue]),
                        ),
                    )
                {
                    return self.continue_statement();
                }
                return self.expression_statement();
            }
            fn break_statement(&mut self) -> Result<Stmt> {
                self.consume(TokenType::Semicolon, "Expect ';' after 'break'.")?;
                return Ok(Stmt {
                    range: self.previous().range.clone(),
                    intern: StmtType::Break,
                });
            }
            fn continue_statement(&mut self) -> Result<Stmt> {
                self.consume(TokenType::Semicolon, "Expect ';' after 'continue'.")?;
                return Ok(Stmt {
                    range: self.previous().range.clone(),
                    intern: StmtType::Continue,
                });
            }
            fn while_statement(&mut self) -> Result<Stmt> {
                self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
                let condition = self.expression()?;
                self.consume(TokenType::RightParen, "Expect ')' after condition.")?;
                let body = Box::new(self.statement()?);
                return Ok(Stmt {
                    range: condition.range.merge(&body.range),
                    intern: StmtType::While(condition, body),
                });
            }
            fn for_statement(&mut self) -> Result<Stmt> {
                self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;
                let initializer = if self
                    .mtch(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([TokenType::Semicolon]),
                        ),
                    )
                {
                    None
                } else if self
                    .mtch(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([TokenType::Var]),
                        ),
                    )
                {
                    Some(self.var_declaration()?)
                } else {
                    Some(self.expression_statement()?)
                };
                let condition = if !self.check(TokenType::Semicolon) {
                    self.expression()?
                } else {
                    Expr {
                        range: self.previous().range.clone(),
                        intern: Box::new(ExprType::Literal(Literal::True)),
                        id: self.exp_id_counter.next(),
                    }
                };
                self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;
                let increment = if !self.check(TokenType::RightParen) {
                    Some(self.expression()?)
                } else {
                    None
                };
                self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;
                let mut body = self.statement()?;
                if let Some(increment) = increment {
                    body = Stmt {
                        range: body.range.merge(&increment.range),
                        intern: StmtType::Block(
                            <[_]>::into_vec(
                                #[rustc_box]
                                ::alloc::boxed::Box::new([
                                    body,
                                    Stmt {
                                        range: increment.range.clone(),
                                        intern: StmtType::Expr(increment),
                                    },
                                ]),
                            ),
                        ),
                    };
                }
                body = Stmt {
                    range: condition.range.merge(&body.range),
                    intern: StmtType::While(condition, Box::new(body)),
                };
                if let Some(initializer) = initializer {
                    body = Stmt {
                        range: initializer.range.merge(&body.range),
                        intern: StmtType::Block(
                            <[_]>::into_vec(
                                #[rustc_box]
                                ::alloc::boxed::Box::new([initializer, body]),
                            ),
                        ),
                    };
                }
                return Ok(body);
            }
            fn if_statement(&mut self) -> Result<Stmt> {
                self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
                let condition = self.expression()?;
                self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;
                let then_branch = Box::new(self.statement()?);
                let else_branch = if self
                    .mtch(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([TokenType::Else]),
                        ),
                    )
                {
                    Some(Box::new(self.statement()?))
                } else {
                    None
                };
                return Ok(Stmt {
                    range: condition
                        .range
                        .merge(&then_branch.range)
                        .merge(&self.previous().range),
                    intern: StmtType::IfStmt(condition, then_branch, else_branch),
                });
            }
            fn block_statement(&mut self) -> Result<Stmt> {
                let mut statements = Vec::new();
                while !self.check(TokenType::RightBrace) && !self.is_at_end() {
                    statements.push(self.declaration()?);
                }
                self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
                return Ok(Stmt {
                    range: self.previous().range.clone(),
                    intern: StmtType::Block(statements),
                });
            }
            fn return_statement(&mut self) -> Result<Stmt> {
                let keyword = self.previous().to_owned();
                let value = if self.check(TokenType::Semicolon) {
                    Expr {
                        range: keyword.range.clone(),
                        intern: Box::new(ExprType::Literal(Literal::Nil)),
                        id: self.exp_id_counter.next(),
                    }
                } else {
                    self.expression()?
                };
                self.consume(TokenType::Semicolon, "Expect ';' after return value.")?;
                return Ok(Stmt {
                    range: keyword.range.merge(&value.range),
                    intern: StmtType::Return(value),
                });
            }
            fn print_statement(&mut self) -> Result<Stmt> {
                let value = self.expression()?;
                self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
                return Ok(Stmt {
                    range: value.range.clone(),
                    intern: StmtType::Print(value),
                });
            }
            fn expression_statement(&mut self) -> Result<Stmt> {
                let expr = self.expression()?;
                self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
                return Ok(Stmt {
                    range: expr.range.clone(),
                    intern: StmtType::Expr(expr),
                });
            }
            fn expression(&mut self) -> Result<Expr> {
                return self.assignment();
            }
            fn assignment(&mut self) -> Result<Expr> {
                let expr = self.or()?;
                if self
                    .mtch(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([TokenType::Equal]),
                        ),
                    )
                {
                    let equals = self.previous().to_owned();
                    let value = self.assignment()?;
                    if let ExprType::Variable(ref name) = *expr.intern {
                        return Ok(Expr {
                            range: equals.range.merge(&value.range),
                            intern: Box::new(ExprType::Assign(name.clone(), value)),
                            id: self.exp_id_counter.next(),
                        });
                    } else if let ExprType::Get(ref obj, ref name) = *expr.intern {
                        return Ok(Expr {
                            range: equals.range.merge(&value.range),
                            intern: Box::new(
                                ExprType::Set(obj.clone(), name.clone(), value),
                            ),
                            id: self.exp_id_counter.next(),
                        });
                    }
                    self.error(&equals, "Invalid assignment target.");
                }
                return Ok(expr);
            }
            fn or(&mut self) -> Result<Expr> {
                let mut expr = self.and()?;
                while self
                    .mtch(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([TokenType::Or]),
                        ),
                    )
                {
                    let operator = self.previous().inner.clone();
                    let right = self.and()?;
                    expr = Expr {
                        range: expr
                            .range
                            .merge(&right.range)
                            .merge(&self.previous().range),
                        intern: Box::new(
                            ExprType::Logical(Logical {
                                left: expr,
                                operator: (&operator).into(),
                                right: right,
                            }),
                        ),
                        id: self.exp_id_counter.next(),
                    };
                }
                return Ok(expr);
            }
            fn and(&mut self) -> Result<Expr> {
                let mut expr = self.equality()?;
                while self
                    .mtch(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([TokenType::And]),
                        ),
                    )
                {
                    let operator = self.previous().inner.clone();
                    let right = self.equality()?;
                    expr = Expr {
                        range: expr
                            .range
                            .merge(&right.range)
                            .merge(&self.previous().range),
                        intern: Box::new(
                            ExprType::Logical(Logical {
                                left: expr,
                                operator: (&operator).into(),
                                right: right,
                            }),
                        ),
                        id: self.exp_id_counter.next(),
                    };
                }
                return Ok(expr);
            }
            fn equality(&mut self) -> Result<Expr> {
                let mut expr = self.comparison()?;
                while self
                    .mtch(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                TokenType::BangEqual,
                                TokenType::EqualEqual,
                            ]),
                        ),
                    )
                {
                    let operator = self.previous().inner.clone();
                    let right = self.comparison()?;
                    expr = Expr {
                        range: expr
                            .range
                            .merge(&right.range)
                            .merge(&self.previous().range),
                        intern: Box::new(
                            ExprType::Binary(Binary {
                                left: expr,
                                operator: (&operator).into(),
                                right: right,
                            }),
                        ),
                        id: self.exp_id_counter.next(),
                    };
                }
                return Ok(expr);
            }
            fn comparison(&mut self) -> Result<Expr> {
                let mut expr = match self.term() {
                    Ok(expr) => expr,
                    Err(err) => {
                        self.advance();
                        while self
                            .mtch(
                                <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        TokenType::Greater,
                                        TokenType::GreaterEqual,
                                        TokenType::Less,
                                        TokenType::LessEqual,
                                    ]),
                                ),
                            )
                        {
                            let right = self.term()?;
                            {
                                ::std::io::_print(
                                    format_args!("Discarding Right: {0}\n", right),
                                );
                            };
                        }
                        if err.token.inner.is_binary() {
                            return Err(ParserError {
                                message: "binary operator appearing at the beginning of an expression."
                                    .to_string(),
                                token: err.token,
                                backtrace: Backtrace::force_capture(),
                            });
                        }
                        return Err(err);
                    }
                };
                while self
                    .mtch(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                TokenType::Greater,
                                TokenType::GreaterEqual,
                                TokenType::Less,
                                TokenType::LessEqual,
                            ]),
                        ),
                    )
                {
                    let operator = self.previous().inner.clone();
                    let right = self.term()?;
                    expr = Expr {
                        range: expr
                            .range
                            .merge(&right.range)
                            .merge(&self.previous().range),
                        intern: Box::new(
                            ExprType::Binary(Binary {
                                left: expr,
                                operator: (&operator).into(),
                                right: right,
                            }),
                        ),
                        id: self.exp_id_counter.next(),
                    };
                }
                return Ok(expr);
            }
            fn term(&mut self) -> Result<Expr> {
                let mut expr = self.factor()?;
                while self
                    .mtch(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([TokenType::Minus, TokenType::Plus]),
                        ),
                    )
                {
                    let operator = self.previous().inner.clone();
                    let right = self.factor()?;
                    expr = Expr {
                        range: expr
                            .range
                            .merge(&right.range)
                            .merge(&self.previous().range),
                        intern: Box::new(
                            ExprType::Binary(Binary {
                                left: expr,
                                operator: (&operator).into(),
                                right: right,
                            }),
                        ),
                        id: self.exp_id_counter.next(),
                    };
                }
                return Ok(expr);
            }
            fn factor(&mut self) -> Result<Expr> {
                let mut expr = self.unary()?;
                while self
                    .mtch(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([TokenType::Slash, TokenType::Star]),
                        ),
                    )
                {
                    let operator = self.previous().inner.clone();
                    let right = self.unary()?;
                    expr = Expr {
                        range: expr
                            .range
                            .merge(&right.range)
                            .merge(&self.previous().range),
                        intern: Box::new(
                            ExprType::Binary(Binary {
                                left: expr,
                                operator: (&operator).into(),
                                right: right,
                            }),
                        ),
                        id: self.exp_id_counter.next(),
                    };
                }
                return Ok(expr);
            }
            fn unary(&mut self) -> Result<Expr> {
                if self
                    .mtch(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([TokenType::Bang, TokenType::Minus]),
                        ),
                    )
                {
                    let operator = self.previous().inner.clone();
                    let right = self.unary()?;
                    return Ok(Expr {
                        range: right.range.merge(&self.previous().range),
                        intern: Box::new(
                            ExprType::Unary(Unary {
                                intern: (&operator).into(),
                                expr: right,
                            }),
                        ),
                        id: self.exp_id_counter.next(),
                    });
                }
                return self.call();
            }
            fn call(&mut self) -> Result<Expr> {
                let mut expr = self.primary()?;
                loop {
                    if self
                        .mtch(
                            <[_]>::into_vec(
                                #[rustc_box]
                                ::alloc::boxed::Box::new([TokenType::LeftParen]),
                            ),
                        )
                    {
                        expr = self.finish_call(expr)?;
                    }
                    if self
                        .mtch(
                            <[_]>::into_vec(
                                #[rustc_box]
                                ::alloc::boxed::Box::new([TokenType::Dot]),
                            ),
                        )
                    {
                        let name = match self.peek().inner {
                            TokenType::Identifier(_) => self.advance(),
                            _ => {
                                return Err(ParserError {
                                    message: "Expect property name after '.'.".to_string(),
                                    token: self.peek().to_owned(),
                                    backtrace: Backtrace::force_capture(),
                                });
                            }
                        };
                        let name_str = if let TokenType::Identifier(name) = &name.inner {
                            name.clone()
                        } else {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        };
                        expr = Expr {
                            range: expr.range.merge(&name.range),
                            intern: Box::new(ExprType::Get(expr, name_str)),
                            id: self.exp_id_counter.next(),
                        };
                    } else {
                        break;
                    }
                }
                return Ok(expr);
            }
            fn finish_call(&mut self, callee: Expr) -> Result<Expr> {
                let mut arguments = Vec::new();
                if !self.check(TokenType::RightParen) {
                    arguments.push(self.expression()?);
                    while self
                        .mtch(
                            <[_]>::into_vec(
                                #[rustc_box]
                                ::alloc::boxed::Box::new([TokenType::Comma]),
                            ),
                        )
                    {
                        arguments.push(self.expression()?);
                        if arguments.len() >= 255 {
                            self.error(
                                &self.peek().clone(),
                                "Cannot have more than 255 arguments.",
                            );
                        }
                    }
                }
                let paren = self
                    .consume(TokenType::RightParen, "Expect ')' after arguments.")?;
                return Ok(Expr {
                    range: callee.range.merge(&paren.range),
                    intern: Box::new(ExprType::Call(Call { callee: callee, arguments })),
                    id: self.exp_id_counter.next(),
                });
            }
            fn primary(&mut self) -> Result<Expr> {
                if self
                    .mtch(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([TokenType::False]),
                        ),
                    )
                {
                    return Ok(Expr {
                        range: self.previous().range.clone(),
                        intern: Box::new(ExprType::Literal(Literal::False)),
                        id: self.exp_id_counter.next(),
                    });
                }
                if self
                    .mtch(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([TokenType::True]),
                        ),
                    )
                {
                    return Ok(Expr {
                        range: self.previous().range.clone(),
                        intern: Box::new(ExprType::Literal(Literal::True)),
                        id: self.exp_id_counter.next(),
                    });
                }
                if self
                    .mtch(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([TokenType::Nil]),
                        ),
                    )
                {
                    return Ok(Expr {
                        range: self.previous().range.clone(),
                        intern: Box::new(ExprType::Literal(Literal::Nil)),
                        id: self.exp_id_counter.next(),
                    });
                }
                match self.peek().inner.clone() {
                    TokenType::Number(n) => {
                        self.advance();
                        return Ok(Expr {
                            range: self.previous().range.clone(),
                            intern: Box::new(ExprType::Literal(Literal::Number(n))),
                            id: self.exp_id_counter.next(),
                        });
                    }
                    TokenType::String(s) => {
                        self.advance();
                        return Ok(Expr {
                            range: self.previous().range.clone(),
                            intern: Box::new(ExprType::Literal(Literal::String(s))),
                            id: self.exp_id_counter.next(),
                        });
                    }
                    TokenType::Identifier(s) => {
                        self.advance();
                        return Ok(Expr {
                            range: self.previous().range.clone(),
                            intern: Box::new(ExprType::Variable(s)),
                            id: self.exp_id_counter.next(),
                        });
                    }
                    _ => {}
                }
                if self
                    .mtch(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([TokenType::LeftParen]),
                        ),
                    )
                {
                    let expr = self.expression()?;
                    self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
                    return Ok(Expr {
                        range: self.previous().range.clone(),
                        intern: Box::new(ExprType::Grouping(expr)),
                        id: self.exp_id_counter.next(),
                    });
                }
                Err(ParserError {
                    message: "Expect expression.".to_string(),
                    token: self.peek().to_owned(),
                    backtrace: Backtrace::force_capture(),
                })
            }
            fn mtch(&mut self, types: Vec<TokenType>) -> bool {
                for tipe in types {
                    if self.check(tipe) {
                        self.advance();
                        return true;
                    }
                }
                return false;
            }
            fn check(&self, tipe: TokenType) -> bool {
                if self.is_at_end() {
                    return false;
                }
                return self.peek().inner == tipe;
            }
            fn advance(&mut self) -> &Token {
                if !self.is_at_end() {
                    self.current += 1;
                }
                return self.previous();
            }
            fn is_at_end(&self) -> bool {
                return match self.peek().inner {
                    TokenType::EOF => true,
                    _ => false,
                };
            }
            fn peek(&self) -> &Token {
                return self.tokens.get(self.current).unwrap();
            }
            fn previous(&self) -> &Token {
                return self.tokens.get(self.current - 1).unwrap();
            }
        }
    }
    mod resolver {
        use std::collections::HashMap;
        use super::{parser::ast::*, Expr, SouceCodeRange, Stmt};
        enum FunctionType {
            None,
            Function,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for FunctionType {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        FunctionType::None => "None",
                        FunctionType::Function => "Function",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for FunctionType {
            #[inline]
            fn clone(&self) -> FunctionType {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for FunctionType {}
        #[allow(dead_code)]
        pub(crate) enum ResolverError {
            DoubleDeclare(String, SouceCodeRange),
            ReturnOutsideFunction(SouceCodeRange),
            BreakOutsideLoop(SouceCodeRange),
            ContinueOutsideLoop(SouceCodeRange),
        }
        #[automatically_derived]
        #[allow(dead_code)]
        impl ::core::fmt::Debug for ResolverError {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    ResolverError::DoubleDeclare(__self_0, __self_1) => {
                        ::core::fmt::Formatter::debug_tuple_field2_finish(
                            f,
                            "DoubleDeclare",
                            __self_0,
                            &__self_1,
                        )
                    }
                    ResolverError::ReturnOutsideFunction(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "ReturnOutsideFunction",
                            &__self_0,
                        )
                    }
                    ResolverError::BreakOutsideLoop(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "BreakOutsideLoop",
                            &__self_0,
                        )
                    }
                    ResolverError::ContinueOutsideLoop(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "ContinueOutsideLoop",
                            &__self_0,
                        )
                    }
                }
            }
        }
        pub(crate) type ResolverResult<T> = Result<T, ResolverError>;
        pub(crate) struct Resolver {
            scopes: Vec<HashMap<String, bool>>,
            current_function: FunctionType,
            is_in_loop: bool,
            resolved_exprs: HashMap<ExprId, usize>,
        }
        impl std::fmt::Display for Resolver {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(format_args!("Resolver"))?;
                for (id, scope) in self.resolved_exprs.iter() {
                    f.write_fmt(format_args!("\nExprId: {0:?}, Scope: {1}", id, scope))?;
                }
                Ok(())
            }
        }
        impl Resolver {
            pub(crate) fn new() -> Self {
                Self {
                    scopes: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([HashMap::new()]),
                    ),
                    current_function: FunctionType::None,
                    is_in_loop: false,
                    resolved_exprs: HashMap::new(),
                }
            }
            pub(crate) fn into_resolved_exprs(self) -> HashMap<ExprId, usize> {
                self.resolved_exprs
            }
            pub(crate) fn resolve(&mut self, stmts: &[Stmt]) -> ResolverResult<()> {
                for stmt in stmts {
                    self.resolve_stmt(stmt)?;
                }
                Ok(())
            }
            fn resolve_stmt(&mut self, stmt: &Stmt) -> ResolverResult<()> {
                match stmt.intern {
                    StmtType::Block(ref stmts) => {
                        self.begin_scope();
                        self.resolve(stmts)?;
                        self.end_scope();
                    }
                    StmtType::Var(ref name, ref init) => {
                        self.declare(name, &stmt.range)?;
                        if let Some(init) = init {
                            self.resolve_expr(init)?;
                        }
                        self.define(name);
                    }
                    StmtType::Function(_, ref name, ref args, ref body) => {
                        self.declare(name, &stmt.range)?;
                        self.define(name);
                        self.resolve_function(args, body)?;
                    }
                    StmtType::Class(ref name, ref methods) => {
                        self.declare(name, &stmt.range)?;
                        self.define(name);
                    }
                    StmtType::Expr(ref expr) => {
                        self.resolve_expr(expr)?;
                    }
                    StmtType::IfStmt(ref cond, ref then_branch, ref else_branch) => {
                        self.resolve_expr(cond)?;
                        self.resolve_stmt(then_branch)?;
                        if let Some(else_branch) = else_branch {
                            self.resolve_stmt(else_branch)?;
                        }
                    }
                    StmtType::Print(ref expr) => {
                        self.resolve_expr(expr)?;
                    }
                    StmtType::Return(ref expr) => {
                        if let FunctionType::None = self.current_function {
                            return Err(
                                ResolverError::ReturnOutsideFunction(stmt.range.clone()),
                            );
                        }
                        self.resolve_expr(expr)?;
                    }
                    StmtType::While(ref cond, ref body) => {
                        let enclosing_loop = self.is_in_loop;
                        self.is_in_loop = true;
                        self.resolve_expr(cond)?;
                        self.resolve_stmt(body)?;
                        self.is_in_loop = enclosing_loop;
                    }
                    StmtType::Break => {
                        if !self.is_in_loop {
                            return Err(
                                ResolverError::BreakOutsideLoop(stmt.range.clone()),
                            );
                        }
                    }
                    StmtType::Continue => {
                        if !self.is_in_loop {
                            return Err(
                                ResolverError::ContinueOutsideLoop(stmt.range.clone()),
                            );
                        }
                    }
                }
                Ok(())
            }
            pub fn resolve_expr(&mut self, expr: &Expr) -> ResolverResult<()> {
                match *expr.intern {
                    ExprType::Variable(ref name) => {
                        if let Some(scope) = self.scopes.last() {
                            if let Some(&false) = scope.get(name) {
                                {
                                    ::std::io::_print(
                                        format_args!(
                                            "Cannot read local variable in its own initializer.\n",
                                        ),
                                    );
                                };
                            }
                        }
                        self.resolve_local(expr, name);
                    }
                    ExprType::Assign(ref name, ref value) => {
                        self.resolve_expr(value)?;
                        self.resolve_local(expr, name);
                    }
                    ExprType::Binary(ref binary) => {
                        self.resolve_expr(&binary.left)?;
                        self.resolve_expr(&binary.right)?;
                    }
                    ExprType::Call(ref call) => {
                        self.resolve_expr(&call.callee)?;
                        for arg in &call.arguments {
                            self.resolve_expr(arg)?;
                        }
                    }
                    ExprType::Grouping(ref expr) => {
                        self.resolve_expr(expr)?;
                    }
                    ExprType::Literal(_) => {}
                    ExprType::Logical(ref logical) => {
                        self.resolve_expr(&logical.left)?;
                        self.resolve_expr(&logical.right)?;
                    }
                    ExprType::Unary(ref unary) => {
                        self.resolve_expr(&unary.expr)?;
                    }
                    ExprType::Get(ref get, ref _name) => {
                        self.resolve_expr(&get)?;
                    }
                    ExprType::Set(ref set, ref _name, ref value) => {
                        self.resolve_expr(value)?;
                        self.resolve_expr(&set)?;
                    }
                }
                Ok(())
            }
            fn resolve_local(&mut self, expr: &Expr, name: &str) {
                for (i, scope) in self.scopes.iter().rev().enumerate() {
                    if scope.contains_key(name) {
                        self.resolved_exprs.insert(expr.id, i);
                        return;
                    }
                }
            }
            fn resolve_function(
                &mut self,
                args: &[String],
                body: &Stmt,
            ) -> ResolverResult<()> {
                let enclosing_function = self.current_function;
                self.current_function = FunctionType::Function;
                self.begin_scope();
                for arg in args {
                    self.declare(arg, &body.range)?;
                    self.define(arg);
                }
                self.resolve_stmt(body)?;
                self.end_scope();
                self.current_function = enclosing_function;
                Ok(())
            }
            fn begin_scope(&mut self) {
                self.scopes.push(HashMap::new());
            }
            fn end_scope(&mut self) {
                self.scopes.pop();
            }
            fn declare(
                &mut self,
                name: &str,
                range: &SouceCodeRange,
            ) -> ResolverResult<()> {
                if let Some(scope) = self.scopes.last_mut() {
                    if scope.contains_key(name) {
                        return Err(
                            ResolverError::DoubleDeclare(name.to_string(), range.clone()),
                        );
                    } else {
                        scope.insert(name.to_string(), false);
                    }
                }
                Ok(())
            }
            fn define(&mut self, name: &str) {
                if let Some(scope) = self.scopes.last_mut() {
                    scope.insert(name.to_string(), true);
                }
            }
        }
    }
    use std::fmt::Display;
    pub(crate) use lexer::token;
    use lexer::tokenize;
    use parser::ast::{Expr, Stmt};
    pub(crate) struct SouceCodeRange {
        pub(crate) line: usize,
        pub(crate) start_column: usize,
        pub(crate) length: usize,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for SouceCodeRange {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "SouceCodeRange",
                "line",
                &self.line,
                "start_column",
                &self.start_column,
                "length",
                &&self.length,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for SouceCodeRange {
        #[inline]
        fn clone(&self) -> SouceCodeRange {
            SouceCodeRange {
                line: ::core::clone::Clone::clone(&self.line),
                start_column: ::core::clone::Clone::clone(&self.start_column),
                length: ::core::clone::Clone::clone(&self.length),
            }
        }
    }
    impl SouceCodeRange {
        pub(crate) fn merge(&self, other: &Self) -> Self {
            let line = self.line.min(other.line);
            let start_column = self.start_column.min(other.start_column);
            let length = self.length + other.length;
            Self { line, start_column, length }
        }
    }
    pub(crate) enum InterpreterError {
        LexError,
        ParseError(()),
        ResolverError(resolver::ResolverError),
        ExecError(()),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for InterpreterError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                InterpreterError::LexError => {
                    ::core::fmt::Formatter::write_str(f, "LexError")
                }
                InterpreterError::ParseError(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ParseError",
                        &__self_0,
                    )
                }
                InterpreterError::ResolverError(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ResolverError",
                        &__self_0,
                    )
                }
                InterpreterError::ExecError(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ExecError",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl Display for InterpreterError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                InterpreterError::LexError => f.write_fmt(format_args!("Lex error")),
                InterpreterError::ResolverError(err) => {
                    f.write_fmt(format_args!("Resolver error: {0:?}", err))
                }
                InterpreterError::ParseError(_) => {
                    f.write_fmt(format_args!("Parse error"))
                }
                InterpreterError::ExecError(_) => f.write_fmt(format_args!("Exec error")),
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
        parser.parse_expr().map_err(|_| InterpreterError::ParseError(()))
    }
    pub fn eval(input: &str) -> Result<parser::ast::Literal, InterpreterError> {
        let expr = parse_expr(input)?;
        let mut resolver = resolver::Resolver::new();
        resolver.resolve_expr(&expr).map_err(InterpreterError::ResolverError)?;
        let mut ctx = eval::EvalCtx::new_globals(resolver.into_resolved_exprs());
        eval::Eval::eval(&expr, &mut ctx).map_err(|_| InterpreterError::ExecError(()))
    }
    pub fn run(input: &str) -> Result<(), InterpreterError> {
        let stmts = parse(input).map_err(|_| InterpreterError::ParseError(()))?;
        let mut resolver = resolver::Resolver::new();
        resolver.resolve(&stmts).map_err(InterpreterError::ResolverError)?;
        let mut ctx = eval::EvalCtx::new_globals(resolver.into_resolved_exprs());
        for stmt in &stmts {
            let result = stmt.eval(&mut ctx);
            if let Err(err) = result {
                {
                    ::std::io::_eprint(format_args!("ExecError: {0}\n", err));
                };
                return Err(InterpreterError::ExecError(()));
            }
        }
        Ok(())
    }
}
use std::env;
use std::fs;
use std::io::{self, Write};
use interpreter::lexer::tokenize;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        io::stderr()
            .write_fmt(format_args!("Usage: {0} tokenize <filename>\n", args[0]))
            .unwrap();
        return;
    }
    let command = &args[1];
    let filename = &args[2];
    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename)
                .unwrap_or_else(|_| {
                    io::stderr()
                        .write_fmt(format_args!("Failed to read file {0}\n", filename))
                        .unwrap();
                    String::new()
                });
            let tokens = tokenize(&file_contents);
            match tokens {
                Ok(tokens) => {
                    for token in tokens {
                        {
                            ::std::io::_print(format_args!("{0}\n", token));
                        };
                    }
                }
                Err(()) => {
                    {
                        ::std::io::_print(format_args!("Failed to tokenize input\n"));
                    };
                    std::process::exit(65);
                }
            }
        }
        "parse" => {
            let file_contents = fs::read_to_string(filename)
                .unwrap_or_else(|_| {
                    io::stderr()
                        .write_fmt(format_args!("Failed to read file {0}\n", filename))
                        .unwrap();
                    String::new()
                });
            let expr = interpreter::parse_expr(&file_contents);
            match expr {
                Ok(expr) => {
                    {
                        ::std::io::_print(format_args!("{0}\n", expr));
                    };
                }
                Err(err) => {
                    {
                        ::std::io::_eprint(format_args!("{0}\n", err));
                    };
                    std::process::exit(65);
                }
            }
        }
        "evaluate" => {
            let file_contents = fs::read_to_string(filename)
                .unwrap_or_else(|_| {
                    io::stderr()
                        .write_fmt(format_args!("Failed to read file {0}\n", filename))
                        .unwrap();
                    String::new()
                });
            let result = interpreter::eval(&file_contents);
            match result {
                Ok(result) => {
                    {
                        ::std::io::_print(format_args!("{0}\n", result));
                    };
                }
                Err(err) => {
                    {
                        ::std::io::_eprint(format_args!("{0}\n", err));
                    };
                    std::process::exit(70);
                }
            }
        }
        "run" => {
            let file_contents = fs::read_to_string(filename)
                .unwrap_or_else(|_| {
                    io::stderr()
                        .write_fmt(format_args!("Failed to read file {0}\n", filename))
                        .unwrap();
                    String::new()
                });
            let result = interpreter::run(&file_contents);
            if let Err(err) = result {
                {
                    ::std::io::_eprint(format_args!("{0}\n", err));
                };
                let code = match err {
                    interpreter::InterpreterError::LexError => 65,
                    interpreter::InterpreterError::ParseError(_) => 65,
                    interpreter::InterpreterError::ResolverError(_) => 75,
                    interpreter::InterpreterError::ExecError(_) => 70,
                };
                std::process::exit(code);
            }
        }
        _ => {
            io::stderr()
                .write_fmt(format_args!("Unknown command: {0}\n", command))
                .unwrap();
            return;
        }
    }
}
