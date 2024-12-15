use instructions::Instruction;
use op_codes::OpCode;
use values::Value;

use crate::interpreter::SouceCodeRange;

mod disassembler;
mod instructions;
mod op_codes;
mod values;

pub fn compile(input: &str) {
    let mut chunk = Chunk::new();
    // let idx = chunk.addConstant(Value::Number(1.2));
    // chunk.writeChunk(OpCode::OpConstant as u8, SouceCodeRange::new(0));
    // chunk.writeChunk(idx as u8, SouceCodeRange::new(0));
    // chunk.writeChunk(OpCode::OpReturn as u8, SouceCodeRange::new(1));
    // chunk.writeChunk(OpCode::OpReturn as u8, SouceCodeRange::new(1));
    for i in 0..300 {
        chunk.addInstruction(
            Instruction::Constant(Value::Number(i as f64)),
            SouceCodeRange::new(0),
        );
    }
    chunk.addInstruction(Instruction::Return, SouceCodeRange::new(1));
    disassembler::disassemble_chunk(&chunk, "test");
}

pub struct Chunk {
    code_array: Vec<u8>,
    constant_pool: Vec<Value>,
    lines: Vec<SouceCodeRange>,
}

impl Chunk {
    // pub fn addConstant(&mut self, value: Value) -> usize {
    //     self.constant_pool.push(value);
    //     self.constant_pool.len() - 1
    // }

    // pub fn writeChunk(&mut self, byte: u8, range: SouceCodeRange) {
    //     self.code_array.push(byte);
    //     self.lines.push(range);
    // }

    pub fn addInstruction(&mut self, instruction: Instruction, range: SouceCodeRange) {
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
            Instruction::Return => {
                self.push_code(OpCode::OpReturn as u8, range);
            }
        }
    }

    fn new() -> Self {
        Self {
            code_array: Vec::new(),
            constant_pool: Vec::new(),
            lines: Vec::new(),
        }
    }

    fn push_code(&mut self, code: u8, line: SouceCodeRange) {
        self.code_array.push(code);
        self.lines.push(line);
    }
}
