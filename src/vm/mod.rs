use crate::compiler::{disassembler::disassemble_instruction, Chunk, OpCode, Value};

pub(crate) struct VM {
    stack: Vec<Value>,
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
            stack: Vec::new(),
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

    fn read_constant(&mut self) -> Value {
        let idx = self.read_byte() as usize;
        self.chunk.constant_pool[idx].clone()
    }

    fn read_constant_long(&mut self) -> Value {
        let idx = (self.read_byte() as usize) << 16
            | (self.read_byte() as usize) << 8
            | self.read_byte() as usize;
        self.chunk.constant_pool[idx].clone()
    }

    pub fn run(&mut self) -> Result<(), InterpretError> {
        loop {
            if self.debug {
                println!("Stack: {:?}", self.stack);
                disassemble_instruction(&self.chunk, self.ip);
            }
            let instruction: OpCode = self
                .read_byte()
                .try_into()
                .map_err(|_| InterpretError::InvalidInstruction)?;
            use OpCode::*;
            match instruction {
                OpReturn => return Ok(()),
                OpConstant => {
                    let constant = self.read_constant();
                    self.stack.push(constant);
                }
                OpConstantLong => {
                    let constant = self.read_constant_long();
                    self.stack.push(constant);
                }
                OpNegate => {
                    if let Value::Number(num) =
                        self.stack.pop().ok_or(InterpretError::StackUnderflow)?
                    {
                        self.stack.push(Value::Number(-num));
                    } else {
                        return Err(InterpretError::InvalidInstruction);
                    }
                }
                OpAdd | OpSubtract | OpMultiply | OpDivide => {
                    if let (Value::Number(b), Value::Number(a)) = (
                        self.stack.pop().ok_or(InterpretError::StackUnderflow)?,
                        self.stack.pop().ok_or(InterpretError::StackUnderflow)?,
                    ) {
                        let result = match instruction {
                            OpAdd => a + b,
                            OpSubtract => a - b,
                            OpMultiply => a * b,
                            OpDivide => a / b,
                            _ => unreachable!(),
                        };
                        self.stack.push(Value::Number(result));
                    } else {
                        return Err(InterpretError::InvalidInstruction);
                    }
                }
            }
        }
    }
}
