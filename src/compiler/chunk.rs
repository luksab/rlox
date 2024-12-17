use crate::compiler::OpCode;

use super::{Instruction, SourceCodeRange, Value};

pub struct Chunk {
    pub(crate) code_array: Vec<u8>,
    pub(crate) constant_pool: Vec<Value>,
    pub(crate) lines: Vec<SourceCodeRange>,
}

impl Chunk {
    pub fn add_instruction(&mut self, instruction: Instruction, range: SourceCodeRange) {
        if let Ok(op) = OpCode::try_from(&instruction) {
            self.push_code(op as u8, range);
        } else {
            use Instruction::*;
            match instruction {
                Constant(value) => {
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
                _ => {
                    unreachable!(
                        "All instructions should either be try_from or handled in the match"
                    )
                }
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

    pub fn push_code(&mut self, code: u8, line: SourceCodeRange) {
        self.code_array.push(code);
        self.lines.push(line);
    }
}
