use instructions::Instruction;
pub use op_codes::OpCode;
pub use values::Value;

use crate::interpreter::SouceCodeRange;

pub(crate) mod disassembler;
mod instructions;
mod op_codes;
mod values;

#[derive(Debug)]
pub enum CompileError {
    LexError,
    ParseError(()),
    // ResolverError(resolver::ResolverError),
    ExecError(()),
}

pub fn compile(input: &str) -> Result<Chunk, CompileError> {
    let mut chunk = Chunk::new();
    // let idx = chunk.addConstant(Value::Number(1.2));
    // chunk.writeChunk(OpCode::OpConstant as u8, SouceCodeRange::new(0));
    // chunk.writeChunk(idx as u8, SouceCodeRange::new(0));
    // chunk.writeChunk(OpCode::OpReturn as u8, SouceCodeRange::new(1));
    // chunk.writeChunk(OpCode::OpReturn as u8, SouceCodeRange::new(1));
    for i in 0..10 {
        chunk.add_instruction(
            Instruction::Constant(Value::Number(i as f64)),
            SouceCodeRange::new(0),
        );
    }
    chunk.add_instruction(Instruction::Return, SouceCodeRange::new(1));
    Ok(chunk)
}

pub struct Chunk {
    pub(crate) code_array: Vec<u8>,
    pub(crate) constant_pool: Vec<Value>,
    pub(crate) lines: Vec<SouceCodeRange>,
}

impl Chunk {
    pub fn add_instruction(&mut self, instruction: Instruction, range: SouceCodeRange) {
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
