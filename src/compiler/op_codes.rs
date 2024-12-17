use std::fmt::Display;

use super::{Instruction, Value};

pub enum OpCode {
    OpReturn = 0,
    OpConstant,
    OpNil,
    OpFalse,
    OpTrue,
    OpConstantLong,
    OpNegate,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
}

impl TryFrom<&Instruction> for OpCode {
    type Error = ();

    fn try_from(value: &Instruction) -> Result<Self, Self::Error> {
        match value {
            Instruction::Constant(Value::Nil) => Ok(OpCode::OpNil),
            Instruction::Constant(Value::Bool(false)) => Ok(OpCode::OpFalse),
            Instruction::Constant(Value::Bool(true)) => Ok(OpCode::OpTrue),
            Instruction::Negate => Ok(OpCode::OpNegate),
            Instruction::Add => Ok(OpCode::OpAdd),
            Instruction::Subtract => Ok(OpCode::OpSubtract),
            Instruction::Multiply => Ok(OpCode::OpMultiply),
            Instruction::Divide => Ok(OpCode::OpDivide),
            Instruction::Return => Ok(OpCode::OpReturn),
            Instruction::Constant(_) => Err(()),
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OpCode::OpReturn => write!(f, "OP_RETURN"),
            OpCode::OpConstant => write!(f, "OP_CONSTANT"),
            OpCode::OpNil => write!(f, "OP_NIL"),
            OpCode::OpFalse => write!(f, "OP_FALSE"),
            OpCode::OpTrue => write!(f, "OP_TRUE"),
            OpCode::OpConstantLong => write!(f, "OP_CONSTANT_LONG"),
            OpCode::OpNegate => write!(f, "OP_NEGATE"),
            OpCode::OpAdd => write!(f, "OP_ADD"),
            OpCode::OpSubtract => write!(f, "OP_SUBTRACT"),
            OpCode::OpMultiply => write!(f, "OP_MULTIPLY"),
            OpCode::OpDivide => write!(f, "OP_DIVIDE"),
        }
    }
}

impl TryFrom<u8> for OpCode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const OP_RETURN: u8 = OpCode::OpReturn as u8;
        const OP_CONSTANT: u8 = OpCode::OpConstant as u8;
        const OP_CONSTANT_LONG: u8 = OpCode::OpConstantLong as u8;
        const OP_NIL: u8 = OpCode::OpNil as u8;
        const OP_FALSE: u8 = OpCode::OpFalse as u8;
        const OP_TRUE: u8 = OpCode::OpTrue as u8;
        const OP_NEGATE: u8 = OpCode::OpNegate as u8;
        const OP_ADD: u8 = OpCode::OpAdd as u8;
        const OP_SUBTRACT: u8 = OpCode::OpSubtract as u8;
        const OP_MULTIPLY: u8 = OpCode::OpMultiply as u8;
        const OP_DIVIDE: u8 = OpCode::OpDivide as u8;
        match value {
            OP_RETURN => Ok(OpCode::OpReturn),
            OP_CONSTANT => Ok(OpCode::OpConstant),
            OP_CONSTANT_LONG => Ok(OpCode::OpConstantLong),
            OP_NIL => Ok(OpCode::OpNil),
            OP_FALSE => Ok(OpCode::OpFalse),
            OP_TRUE => Ok(OpCode::OpTrue),
            OP_NEGATE => Ok(OpCode::OpNegate),
            OP_ADD => Ok(OpCode::OpAdd),
            OP_SUBTRACT => Ok(OpCode::OpSubtract),
            OP_MULTIPLY => Ok(OpCode::OpMultiply),
            OP_DIVIDE => Ok(OpCode::OpDivide),
            _ => Err(()),
        }
    }
}
