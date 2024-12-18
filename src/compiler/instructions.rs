use ustr::Ustr;

use super::values::Value;

/// high level instructions (eg. no constantLong)
#[derive(Debug)]
pub(crate) enum Instruction {
    Constant(Value),
    Return,
    Print,
    Pop,
    DefineGlobal(Ustr),
    GetGlobal(Ustr),
    SetGlobal(Ustr),
    Not,
    Negate,
    Equal,
    Greater,
    Less,
    Add,
    Subtract,
    Multiply,
    Divide,
}
