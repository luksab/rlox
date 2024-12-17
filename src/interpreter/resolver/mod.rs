use std::collections::HashMap;

use super::{parser::ast::*, Expr, SourceCodeRange, Stmt};

#[derive(Debug, Clone, Copy)]
enum FunctionType {
    None,
    Function,
}

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum ResolverError {
    DoubleDeclare(String, SourceCodeRange),
    ReturnOutsideFunction(SourceCodeRange),
    BreakOutsideLoop(SourceCodeRange),
    ContinueOutsideLoop(SourceCodeRange),
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
        write!(f, "Resolver")?;
        for (id, scope) in self.resolved_exprs.iter() {
            write!(f, "\nExprId: {:?}, Scope: {}", id, scope)?;
        }
        Ok(())
    }
}

impl Resolver {
    pub(crate) fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
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
                    return Err(ResolverError::ReturnOutsideFunction(stmt.range.clone()));
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
                    return Err(ResolverError::BreakOutsideLoop(stmt.range.clone()));
                }
            }
            StmtType::Continue => {
                if !self.is_in_loop {
                    return Err(ResolverError::ContinueOutsideLoop(stmt.range.clone()));
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
                        println!("Cannot read local variable in its own initializer.");
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
                // println!("Resolved {name}({:?}) at scope {i}", expr.id);
                self.resolved_exprs.insert(expr.id, i);
                return;
            }
        }
    }

    fn resolve_function(&mut self, args: &[String], body: &Stmt) -> ResolverResult<()> {
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

    fn declare(&mut self, name: &str, range: &SourceCodeRange) -> ResolverResult<()> {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(name) {
                return Err(ResolverError::DoubleDeclare(
                    name.to_string(),
                    range.clone(),
                ));
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
