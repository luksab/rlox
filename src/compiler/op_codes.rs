use std::fmt::Display;

use super::{Instruction, Value};

pub enum OpCode {
    OpReturn = 0,
    OpPrint,
    OpPop,
    OpConstant,
    /// Has a pointer on the Host machine after it as the index into the constant pool
    OpDefineGlobal,
    /// Has a pointer on the Host machine after it as the index into the constant pool
    OpGetGlobal,
    /// Has a pointer on the Host machine after it as the index into the constant pool
    OpSetGlobal,
    /// Has a pointer on the Host machine after it as the index into the constant pool
    OpGetLocal,
    /// Has a pointer on the Host machine after it as the index into the constant pool
    OpSetLocal,
    OpJumpIfFalse,
    OpJump,
    OpNil,
    OpFalse,
    OpTrue,
    OpConstantLong,
    OpNot,
    OpNegate,
    OpEq,
    OpGreater,
    OpLess,
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
            Instruction::Pop => Ok(OpCode::OpPop),
            Instruction::Not => Ok(OpCode::OpNot),
            Instruction::Negate => Ok(OpCode::OpNegate),
            Instruction::Equal => Ok(OpCode::OpEq),
            Instruction::Greater => Ok(OpCode::OpGreater),
            Instruction::Less => Ok(OpCode::OpLess),
            Instruction::Add => Ok(OpCode::OpAdd),
            Instruction::Subtract => Ok(OpCode::OpSubtract),
            Instruction::Multiply => Ok(OpCode::OpMultiply),
            Instruction::Divide => Ok(OpCode::OpDivide),
            Instruction::Return => Ok(OpCode::OpReturn),
            Instruction::Print => Ok(OpCode::OpPrint),
            Instruction::Constant(_) => Err(()),
            Instruction::DefineGlobal(_) => Err(()),
            Instruction::GetGlobal(_) => Err(()),
            Instruction::SetGlobal(_) => Err(()),
            Instruction::GetLocal(_) => Err(()),
            Instruction::SetLocal(_) => Err(()),
            Instruction::Jump(_) => Err(()),
            Instruction::JumpIfFalse(_) => Err(()),
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OpCode::OpReturn => write!(f, "OP_RETURN"),
            OpCode::OpPrint => write!(f, "OP_PRINT"),
            OpCode::OpPop => write!(f, "OP_POP"),
            OpCode::OpConstant => write!(f, "OP_CONSTANT"),
            OpCode::OpDefineGlobal => write!(f, "OP_DEFINE_GLOBAL"),
            OpCode::OpGetGlobal => write!(f, "OP_GET_GLOBAL"),
            OpCode::OpSetGlobal => write!(f, "OP_SET_GLOBAL"),
            OpCode::OpGetLocal => write!(f, "OP_GET_LOCAL"),
            OpCode::OpSetLocal => write!(f, "OP_SET_LOCAL"),
            OpCode::OpNot => write!(f, "OP_NOT"),
            OpCode::OpNil => write!(f, "OP_NIL"),
            OpCode::OpFalse => write!(f, "OP_FALSE"),
            OpCode::OpTrue => write!(f, "OP_TRUE"),
            OpCode::OpConstantLong => write!(f, "OP_CONSTANT_LONG"),
            OpCode::OpNegate => write!(f, "OP_NEGATE"),
            OpCode::OpEq => write!(f, "OP_EQ"),
            OpCode::OpGreater => write!(f, "OP_GREATER"),
            OpCode::OpLess => write!(f, "OP_LESS"),
            OpCode::OpAdd => write!(f, "OP_ADD"),
            OpCode::OpSubtract => write!(f, "OP_SUBTRACT"),
            OpCode::OpMultiply => write!(f, "OP_MULTIPLY"),
            OpCode::OpDivide => write!(f, "OP_DIVIDE"),
            OpCode::OpJumpIfFalse => write!(f, "OP_JUMP_IF_FALSE"),
            OpCode::OpJump => write!(f, "OP_JUMP"),
        }
    }
}

impl TryFrom<u8> for OpCode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const OP_RETURN: u8 = OpCode::OpReturn as u8;
        const OP_PRINT: u8 = OpCode::OpPrint as u8;
        const OP_POP: u8 = OpCode::OpPop as u8;
        const OP_CONSTANT: u8 = OpCode::OpConstant as u8;
        const OP_CONSTANT_LONG: u8 = OpCode::OpConstantLong as u8;
        const OP_DEFINE_GLOBAL: u8 = OpCode::OpDefineGlobal as u8;
        const OP_GET_GLOBAL: u8 = OpCode::OpGetGlobal as u8;
        const OP_SET_GLOBAL: u8 = OpCode::OpSetGlobal as u8;
        const OP_GET_LOCAL: u8 = OpCode::OpGetLocal as u8;
        const OP_SET_LOCAL: u8 = OpCode::OpSetLocal as u8;
        const OP_NOT: u8 = OpCode::OpNot as u8;
        const OP_NIL: u8 = OpCode::OpNil as u8;
        const OP_FALSE: u8 = OpCode::OpFalse as u8;
        const OP_TRUE: u8 = OpCode::OpTrue as u8;
        const OP_NEGATE: u8 = OpCode::OpNegate as u8;
        const OP_EQ: u8 = OpCode::OpEq as u8;
        const OP_GREATER: u8 = OpCode::OpGreater as u8;
        const OP_LESS: u8 = OpCode::OpLess as u8;
        const OP_ADD: u8 = OpCode::OpAdd as u8;
        const OP_SUBTRACT: u8 = OpCode::OpSubtract as u8;
        const OP_MULTIPLY: u8 = OpCode::OpMultiply as u8;
        const OP_DIVIDE: u8 = OpCode::OpDivide as u8;
        const OP_JUMP_IF_FALSE: u8 = OpCode::OpJumpIfFalse as u8;
        const OP_JUMP: u8 = OpCode::OpJump as u8;
        match value {
            OP_RETURN => Ok(OpCode::OpReturn),
            OP_PRINT => Ok(OpCode::OpPrint),
            OP_POP => Ok(OpCode::OpPop),
            OP_CONSTANT => Ok(OpCode::OpConstant),
            OP_CONSTANT_LONG => Ok(OpCode::OpConstantLong),
            OP_DEFINE_GLOBAL => Ok(OpCode::OpDefineGlobal),
            OP_GET_GLOBAL => Ok(OpCode::OpGetGlobal),
            OP_SET_GLOBAL => Ok(OpCode::OpSetGlobal),
            OP_GET_LOCAL => Ok(OpCode::OpGetLocal),
            OP_SET_LOCAL => Ok(OpCode::OpSetLocal),
            OP_NOT => Ok(OpCode::OpNot),
            OP_NIL => Ok(OpCode::OpNil),
            OP_FALSE => Ok(OpCode::OpFalse),
            OP_TRUE => Ok(OpCode::OpTrue),
            OP_NEGATE => Ok(OpCode::OpNegate),
            OP_EQ => Ok(OpCode::OpEq),
            OP_GREATER => Ok(OpCode::OpGreater),
            OP_LESS => Ok(OpCode::OpLess),
            OP_ADD => Ok(OpCode::OpAdd),
            OP_SUBTRACT => Ok(OpCode::OpSubtract),
            OP_MULTIPLY => Ok(OpCode::OpMultiply),
            OP_DIVIDE => Ok(OpCode::OpDivide),
            OP_JUMP_IF_FALSE => Ok(OpCode::OpJumpIfFalse),
            OP_JUMP => Ok(OpCode::OpJump),
            _ => Err(()),
        }
    }
}
