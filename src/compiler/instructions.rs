use super::values::Value;

/// high level instructions (eg. no constantLong)
#[derive(Debug)]
pub(crate) enum Instruction {
    Constant(Value),
    Return,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
}
