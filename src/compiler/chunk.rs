use crate::compiler::OpCode;

use super::{Instruction, SouceCodeRange, Value};


pub struct Chunk {
    pub(crate) code_array: Vec<u8>,
    pub(crate) constant_pool: Vec<Value>,
    pub(crate) lines: Vec<SouceCodeRange>,
}

impl Chunk {
    pub fn add_instruction(&mut self, instruction: Instruction, range: SouceCodeRange) {
        use Instruction::*;
        match instruction {
            Instruction::Constant(value) => {
                let idx = self.constant_pool.len();
                self.constant_pool.push(value);

                if idx > 255 {
                    self.push_code(OpCode::OpConstantLong as u8, range);
                    self.push_code((idx >> 16) as u8, range);
                    self.push_code((idx >> 8) as u8, range);
                    self.push_code(idx as u8, range);
                    return;
                } else {
                    self.push_code(OpCode::OpConstant as u8, range);
                    self.push_code(idx as u8, range);
                }
            }
            Return | Negate | Add | Subtract | Multiply | Divide => {
                self.push_code(
                    TryInto::<OpCode>::try_into(instruction).unwrap() as u8,
                    range,
                );
            }
        }
    }

    pub fn new() -> Self {
        Self {
            code_array: Vec::new(),
            constant_pool: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn push_code(&mut self, code: u8, line: SouceCodeRange) {
        self.code_array.push(code);
        self.lines.push(line);
    }
}
