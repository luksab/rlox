use std::collections::LinkedList;

use crate::{
    compiler::{disassembler::disassemble_instruction, Chunk, OpCode, Value},
    interpreter::SourceCodeRange,
};

pub(crate) struct VM {
    stack: Vec<Value>,
    chunk: Chunk,
    /// Instruction Pointer. Points to the next instruction to be executed
    ip: usize,
    debug: bool,
    object_heap: LinkedList<Value>,
}

#[derive(Debug)]
pub struct InterpreterError {
    pub(crate) error_type: InterpretErrorType,
    pub(crate) range: SourceCodeRange,
}

#[derive(Debug)]
pub(crate) enum InterpretErrorType {
    StackUnderflow,
    InvalidInstruction,
    InvalidData(String),
}

impl InterpreterError {
    pub(crate) fn new(error_type: InterpretErrorType, range: SourceCodeRange) -> Self {
        Self { error_type, range }
    }
}

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            stack: Vec::new(),
            chunk,
            ip: 0,
            debug: false,
            object_heap: LinkedList::new(),
        }
    }

    fn free_objects(&mut self) {
        self.object_heap.clear();
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

    fn runtime_error(&self, current_ip: usize, error: InterpretErrorType) -> InterpreterError {
        InterpreterError::new(error, self.chunk.lines[current_ip])
    }

    pub fn run(&mut self) -> Result<(), InterpreterError> {
        loop {
            if self.debug {
                println!("Stack: {:?}", self.stack);
                disassemble_instruction(&self.chunk, self.ip);
            }
            let current_ip = self.ip;
            let instruction: OpCode = self.read_byte().try_into().map_err(|_| {
                self.runtime_error(current_ip, InterpretErrorType::InvalidInstruction)
            })?;
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
                OpNil => self.stack.push(Value::Nil),
                OpFalse => self.stack.push(Value::Bool(false)),
                OpTrue => self.stack.push(Value::Bool(true)),
                OpPrint => {
                    if let Some(val) = self.stack.pop() {
                        println!("{}", val);
                    } else {
                        return Err(
                            self.runtime_error(current_ip, InterpretErrorType::StackUnderflow)
                        );
                    }
                }
                OpPop => {
                    self.stack.pop().ok_or_else(|| {
                        self.runtime_error(current_ip, InterpretErrorType::StackUnderflow)
                    })?;
                }
                OpDefineGlobal => {
                    let mut pointer_address = 0;
                    for i in 0..std::mem::size_of::<usize>() {
                        pointer_address |= (self.read_byte() as usize) << (i * 8);
                    }
                    let value = self.stack.last().ok_or_else(|| {
                        self.runtime_error(current_ip, InterpretErrorType::StackUnderflow)
                    })?;
                    let key = unsafe { std::mem::transmute::<usize, ustr::Ustr>(pointer_address) };
                    if let Some(_) = self.chunk.globals.get(&key) {
                        return Err(self.runtime_error(
                            current_ip,
                            InterpretErrorType::InvalidData("Global already defined".to_string()),
                        ));
                    }
                    self.chunk.globals.insert(key, value.clone());
                }
                OpGetGlobal => {
                    let mut pointer_address = 0;
                    for i in 0..std::mem::size_of::<usize>() {
                        pointer_address |= (self.read_byte() as usize) << (i * 8);
                    }
                    let ustring =
                        unsafe { std::mem::transmute::<usize, ustr::Ustr>(pointer_address) };
                    if let Some(value) = self.chunk.globals.get(&ustring) {
                        self.stack.push(value.clone());
                    } else {
                        return Err(self.runtime_error(
                            current_ip,
                            InterpretErrorType::InvalidData("Global not found".to_string()),
                        ));
                    }
                }
                OpSetGlobal => {
                    let mut pointer_address = 0;
                    for i in 0..std::mem::size_of::<usize>() {
                        pointer_address |= (self.read_byte() as usize) << (i * 8);
                    }
                    let value = self.stack.pop().ok_or_else(|| {
                        self.runtime_error(current_ip, InterpretErrorType::StackUnderflow)
                    })?;
                    self.chunk.globals.insert(
                        unsafe { std::mem::transmute::<usize, ustr::Ustr>(pointer_address) },
                        value,
                    );
                }
                OpNegate => {
                    if let Value::Number(num) = self.stack.pop().ok_or_else(|| {
                        self.runtime_error(current_ip, InterpretErrorType::StackUnderflow)
                    })? {
                        self.stack.push(Value::Number(-num));
                    } else {
                        return Err(self.runtime_error(
                            current_ip,
                            InterpretErrorType::InvalidData("Expected number".to_string()),
                        ));
                    }
                }
                OpNot => {
                    let val = self.stack.pop().ok_or_else(|| {
                        self.runtime_error(current_ip, InterpretErrorType::StackUnderflow)
                    })?;
                    let bool: bool = (&val).into();
                    self.stack.push(Value::Bool(!bool));
                }
                OpEq => {
                    let (a, b) = (
                        self.stack.pop().ok_or_else(|| {
                            self.runtime_error(current_ip, InterpretErrorType::StackUnderflow)
                        })?,
                        self.stack.pop().ok_or_else(|| {
                            self.runtime_error(current_ip, InterpretErrorType::StackUnderflow)
                        })?,
                    );
                    self.stack.push(Value::Bool(a == b));
                }
                OpLess | OpGreater => {
                    if let (Value::Number(b), Value::Number(a)) = (
                        self.stack.pop().ok_or_else(|| {
                            self.runtime_error(current_ip, InterpretErrorType::StackUnderflow)
                        })?,
                        self.stack.pop().ok_or_else(|| {
                            self.runtime_error(current_ip, InterpretErrorType::StackUnderflow)
                        })?,
                    ) {
                        let result = match instruction {
                            OpEq => a == b,
                            OpLess => a < b,
                            OpGreater => a > b,
                            _ => unreachable!(),
                        };
                        self.stack.push(Value::Bool(result));
                    } else {
                        return Err(self.runtime_error(
                            current_ip,
                            InterpretErrorType::InvalidData("Expected number".to_string()),
                        ));
                    }
                }
                OpAdd => {
                    match (
                        self.stack.pop().ok_or_else(|| {
                            self.runtime_error(current_ip, InterpretErrorType::StackUnderflow)
                        })?,
                        self.stack.pop().ok_or_else(|| {
                            self.runtime_error(current_ip, InterpretErrorType::StackUnderflow)
                        })?,
                    ) {
                        (Value::Number(b), Value::Number(a)) => {
                            let result = match instruction {
                                OpAdd => a + b,
                                OpSubtract => a - b,
                                OpMultiply => a * b,
                                OpDivide => a / b,
                                _ => unreachable!(),
                            };
                            self.stack.push(Value::Number(result));
                        }
                        (b, Value::String(a)) => {
                            let result = match instruction {
                                OpAdd => a + &b.to_string(),
                                _ => {
                                    return Err(self.runtime_error(
                                        current_ip,
                                        InterpretErrorType::InvalidData(
                                            "Invalid operation on strings".to_string(),
                                        ),
                                    ));
                                }
                            };
                            self.stack.push(Value::String(result));
                        }
                        _ => {
                            return Err(self.runtime_error(
                                current_ip,
                                InterpretErrorType::InvalidData("Expected number".to_string()),
                            ));
                        }
                    }
                }
                OpSubtract | OpMultiply | OpDivide => {
                    if let (Value::Number(b), Value::Number(a)) = (
                        self.stack.pop().ok_or_else(|| {
                            self.runtime_error(current_ip, InterpretErrorType::StackUnderflow)
                        })?,
                        self.stack.pop().ok_or_else(|| {
                            self.runtime_error(current_ip, InterpretErrorType::StackUnderflow)
                        })?,
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
                        return Err(self.runtime_error(
                            current_ip,
                            InterpretErrorType::InvalidData("Expected number".to_string()),
                        ));
                    }
                }
            }
        }
    }
}
