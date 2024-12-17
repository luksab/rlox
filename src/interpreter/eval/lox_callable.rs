use std::backtrace::Backtrace;
use std::usize;

use crate::interpreter::eval::ExecError;

use crate::interpreter::eval::EvalCtx;

use super::Literal;

pub(crate) trait LoxCallable:
    LoxCallableClone + std::fmt::Debug + std::fmt::Display
{
    fn call(&self, args: Vec<Literal>, ctx: &mut EvalCtx) -> Result<Literal, ExecError>;
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

#[derive(Clone)]
pub(crate) struct Clock;

impl LoxCallable for Clock {
    fn call(&self, _args: Vec<Literal>, _ctx: &mut EvalCtx) -> Result<Literal, ExecError> {
        Ok(Literal::Number(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
        ))
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
        write!(f, "<native clock fn>")
    }
}

impl std::fmt::Debug for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native clock fn>")
    }
}

#[derive(Clone)]
pub(crate) struct SysCall;

impl LoxCallable for SysCall {
    fn call(&self, args: Vec<Literal>, _ctx: &mut EvalCtx) -> Result<Literal, ExecError> {
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
                                _ => Err(ExecError {
                                    message: format!("Expected number as argument to exit syscall"),
                                    range: super::SourceCodeRange {
                                        line: 0,
                                        start_column: 0,
                                        length: 0,
                                    },
                                    backtrace: Backtrace::force_capture(),
                                }),
                            })
                            .unwrap_or(Ok(0))?;
                        std::process::exit(code);
                    }
                    _ => Err(ExecError {
                        message: format!("Unknown syscall: {}", syscall),
                        range: super::SourceCodeRange {
                            line: 0,
                            start_column: 0,
                            length: 0,
                        },
                        backtrace: Backtrace::force_capture(),
                    }),
                }
            }
            _ => Err(ExecError {
                message: format!("Expected string as first argument to syscall"),
                range: super::SourceCodeRange {
                    line: 0,
                    start_column: 0,
                    length: 0,
                },
                backtrace: Backtrace::force_capture(),
            }),
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
        write!(f, "<native syscall fn>")
    }
}

impl std::fmt::Debug for SysCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native syscall fn>")
    }
}
