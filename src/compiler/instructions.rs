use super::values::Value;

/// high level instructions (eg. no constantLong)
pub(crate) enum Instruction {
    Constant(Value),
    Return,
}
