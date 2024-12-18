use std::collections::HashMap;

use crate::compiler::OpCode;

use super::{Instruction, SourceCodeRange, Value};

pub struct Chunk {
    pub(crate) code_array: Vec<u8>,
    pub(crate) constant_pool: Vec<Value>,
    pub(crate) globals: HashMap<ustr::Ustr, Value>,
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
                DefineGlobal(u) => {
                    self.push_code(OpCode::OpDefineGlobal as u8, range);
                    let pointer_address = u.as_ptr() as usize;
                    // push all the bytes of the pointer address
                    for i in 0..std::mem::size_of::<usize>() {
                        self.push_code((pointer_address >> (i * 8)) as u8, range);
                    }
                }
                GetGlobal(u) => {
                    self.push_code(OpCode::OpGetGlobal as u8, range);
                    let pointer_address = u.as_ptr() as usize;
                    // push all the bytes of the pointer address
                    for i in 0..std::mem::size_of::<usize>() {
                        self.push_code((pointer_address >> (i * 8)) as u8, range);
                    }
                }
                SetGlobal(u) => {
                    self.push_code(OpCode::OpSetGlobal as u8, range);
                    let pointer_address = u.as_ptr() as usize;
                    // push all the bytes of the pointer address
                    for i in 0..std::mem::size_of::<usize>() {
                        self.push_code((pointer_address >> (i * 8)) as u8, range);
                    }
                }
                instr => {
                    unreachable!(
                        "All instructions should either be try_from or handled in the match block. Got {:?}",
                        instr
                    )
                }
            }
        }
    }

    pub fn new() -> Self {
        Self {
            code_array: Vec::new(),
            constant_pool: Vec::new(),
            globals: HashMap::new(),
            lines: Vec::new(),
        }
    }

    pub fn push_code(&mut self, code: u8, line: SourceCodeRange) {
        self.code_array.push(code);
        self.lines.push(line);
    }
}
