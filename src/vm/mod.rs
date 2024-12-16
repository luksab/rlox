use crate::compiler::{disassembler::disassemble_instruction, Chunk, OpCode, Value};

pub(crate) struct VM {
    chunk: Chunk,
    /// Instruction Pointer. Points to the next instruction to be executed
    ip: usize,
    debug: bool,
}

#[derive(Debug)]
pub(crate) enum InterpretError {
    StackOverflow,
    StackUnderflow,
    InvalidInstruction,
}

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            chunk,
            ip: 0,
            debug: false,
        }
    }

    pub(crate) fn enable_debug(&mut self) {
        self.debug = true;
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.chunk.code_array[self.ip];
        self.ip += 1;
        byte
    }

    fn read_constant(&mut self) -> &mut Value {
        let idx = self.read_byte() as usize;
        &mut self.chunk.constant_pool[idx]
    }

    fn read_constant_long(&mut self) -> &mut Value {
        let idx = (self.read_byte() as usize) << 16
            | (self.read_byte() as usize) << 8
            | self.read_byte() as usize;
        &mut self.chunk.constant_pool[idx]
    }

    pub fn run(&mut self) -> Result<(), InterpretError> {
        loop {
            if self.debug {
                disassemble_instruction(&self.chunk, self.ip);
            }
            let instruction: OpCode = self
                .read_byte()
                .try_into()
                .map_err(|_| InterpretError::InvalidInstruction)?;
            match instruction {
                OpCode::OpReturn => return Ok(()),
                OpCode::OpConstant => {
                    let constant = self.read_constant();
                }
                OpCode::OpConstantLong => {
                    let constant = self.read_constant_long();
                }
            }
        }
    }
}
