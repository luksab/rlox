use std::fmt::Display;

use super::{EvalCtx, ExecError, FunctionType, Literal, LoxCallable, Stmt};

#[derive(Debug, Clone)]
pub(crate) struct LoxFunction {
    // FunctionType, String, Vec<String>, Box<Stmt>
    pub tipe: FunctionType,
    pub name: String,
    pub args: Vec<String>,
    pub body: Box<Stmt>,
    pub closure: EvalCtx,
}

impl LoxCallable for LoxFunction {
    fn call(&self, args: Vec<Literal>, _ctx: &mut EvalCtx) -> Result<Literal, ExecError> {
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
        write!(f, "<fn>")
    }
}
