use std::fmt::Display;

use super::{Stmt, StmtType};

pub struct StmtFormatter<'a> {
    pub stmt: &'a Stmt,
    pub print_block: bool,
}

impl Display for StmtFormatter<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.stmt.intern {
            StmtType::Expr(expr) => write!(f, "{};", expr),
            StmtType::Print(expr) => write!(f, "print {};", expr),
            StmtType::Return(expr) => write!(f, "return {};", expr),
            StmtType::Var(name, initializer) => match initializer {
                Some(initializer) => {
                    write!(f, "var {} = {};", name, initializer)
                }
                None => write!(f, "var {};", name),
            },
            StmtType::IfStmt(condition, then_branch, else_branch) => {
                let mut result = String::new();
                write!(f, "if ({}) {{\n", condition)?;
                result.push_str(&format!("{}", then_branch.into_format_no_block()));
                if let Some(else_branch) = else_branch {
                    write!(f, "}}\nelse {{\n")?;
                    result.push_str(&format!("{}", else_branch.into_format_no_block()));
                }
                for line in result.lines() {
                    write!(f, "  {}\n", line)?;
                }
                write!(f, "}}")
            }
            StmtType::Block(stmts) => {
                if self.print_block {
                    write!(f, "{{\n")?;
                }
                let result = stmts
                    .iter()
                    .map(|stmt| format!("{}", stmt.into_format()))
                    .collect::<Vec<_>>()
                    .join("\n");
                for line in result.lines() {
                    write!(f, "  {}\n", line)?;
                }
                if self.print_block {
                    write!(f, "}}")
                } else {
                    Ok(())
                }
            }
            StmtType::While(expr, stmt) => {
                write!(f, "while ({}) {{\n", expr)?;
                let mut result = String::new();
                result.push_str(&format!("{}", stmt.into_format_no_block()));
                for line in result.lines() {
                    write!(f, "  {}\n", line)?;
                }
                write!(f, "}}")
            }
            StmtType::Break => write!(f, "break;"),
            StmtType::Continue => write!(f, "continue;"),
            StmtType::Function(function_type, name, args, inner) => {
                let mut result = String::new();
                let function_type_str = match function_type {
                    super::FunctionType::Function => "function",
                    super::FunctionType::Method => "method",
                };
                write!(f, "{} {}(", function_type_str, name)?;
                let args = args
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{}) {{\n", args)?;
                result.push_str(&format!("{}", inner.into_format_no_block()));
                for line in result.lines() {
                    write!(f, "  {}\n", line)?;
                }
                write!(f, "}}\n")
            }
            StmtType::Class(name, methods) => {
                write!(f, "class {} {{\n", name)?;
                let result = methods
                    .iter()
                    .map(|method| format!("{}", method.into_format()))
                    .collect::<Vec<_>>()
                    .join("\n");
                for line in result.lines() {
                    write!(f, "  {}\n", line)?;
                }
                write!(f, "}}")
            }
        }
    }
}
