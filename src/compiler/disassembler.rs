use crate::compiler::op_codes::OpCode;

use super::Chunk;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.code_array.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);
    if offset > chunk.code_array.len() {
        println!("End of chunk");
        return offset;
    }
    if offset > 0 && chunk.lines[offset].line == chunk.lines[offset - 1].line {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.lines[offset].line);
    }

    let instruction: Result<OpCode, _> = chunk.code_array[offset].try_into();
    use OpCode::*;
    match instruction {
        Ok(OpReturn) => simple_instruction("OP_RETURN", offset),
        Ok(OpConstant) => const_instruction(chunk, offset),
        Ok(OpConstantLong) => const_long_instruction(chunk, offset),
        Ok(OpNegate) => simple_instruction("OP_NEGATE", offset),
        Ok(
            OpAdd | OpSubtract | OpMultiply | OpDivide | OpNil | OpFalse | OpTrue | OpNot | OpEq
            | OpGreater | OpLess,
        ) => simple_instruction(&instruction.unwrap().to_string(), offset),
        Err(_) => {
            println!("Unknown opcode {}", chunk.code_array[offset]);
            offset + 1
        }
    }
}

fn const_instruction(chunk: &Chunk, offset: usize) -> usize {
    let constant_idx = chunk.code_array[offset + 1];
    let constant = &chunk.constant_pool[constant_idx as usize];
    println!("OP_CONSTANT {} '{}'", constant_idx, constant);
    offset + 2
}

fn const_long_instruction(chunk: &Chunk, offset: usize) -> usize {
    // the next 24 bits interpreted as a u32
    let constant_idx = (chunk.code_array[offset + 1] as u32) << 16
        | (chunk.code_array[offset + 2] as u32) << 8
        | chunk.code_array[offset + 3] as u32;
    let constant = &chunk.constant_pool[constant_idx as usize];
    println!("OP_CONSTANT_LONG {} '{}'", constant_idx, constant);
    offset + 4
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}
