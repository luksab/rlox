use std::fmt::Display;

pub enum OpCode {
    OpReturn = 0,
    OpConstant = 1,
    OpConstantLong = 2,
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OpCode::OpReturn => write!(f, "OP_RETURN"),
            OpCode::OpConstant => write!(f, "OP_CONSTANT"),
            OpCode::OpConstantLong => write!(f, "OP_CONSTANT_LONG"),
        }
    }
}

impl TryFrom<u8> for OpCode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const OP_RETURN: u8 = OpCode::OpReturn as u8;
        const OP_CONSTANT: u8 = OpCode::OpConstant as u8;
        const OP_CONSTANT_LONG: u8 = OpCode::OpConstantLong as u8;
        match value {
            OP_RETURN => Ok(OpCode::OpReturn),
            OP_CONSTANT => Ok(OpCode::OpConstant),
            OP_CONSTANT_LONG => Ok(OpCode::OpConstantLong),
            _ => Err(()),
        }
    }
}