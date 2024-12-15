use crate::compiler::op_codes::OpCode;

use super::Chunk;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.code_array.len() {
        offset = disassemble_instruction(chunk, &mut offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: &mut usize) -> usize {
    print!("{:04} ", offset);
    if *offset > chunk.code_array.len() {
        println!("End of chunk");
        return *offset;
    }
    if *offset > 0 && chunk.lines[*offset].line == chunk.lines[*offset - 1].line {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.lines[*offset].line);
    }

    let instruction: Result<OpCode, _> = chunk.code_array[*offset].try_into();
    match instruction {
        Ok(OpCode::OpReturn) => simple_instruction("OP_RETURN", offset),
        Ok(OpCode::OpConstant) => const_instruction(chunk, offset),
        Ok(OpCode::OpConstantLong) => const_long_instruction(chunk, offset),
        Err(_) => {
            println!("Unknown opcode {}", chunk.code_array[*offset]);
            *offset + 1
        }
    }
}

fn const_instruction(chunk: &Chunk, offset: &mut usize) -> usize {
    let constant_idx = chunk.code_array[*offset + 1];
    let constant = &chunk.constant_pool[constant_idx as usize];
    println!("OP_CONSTANT {} '{}'", constant_idx, constant);
    *offset + 2
}

fn const_long_instruction(chunk: &Chunk, offset: &mut usize) -> usize {
    // the next 24 bits interpreted as a u32
    let constant_idx = (chunk.code_array[*offset + 1] as u32) << 16
        | (chunk.code_array[*offset + 2] as u32) << 8
        | chunk.code_array[*offset + 3] as u32;
    let constant = &chunk.constant_pool[constant_idx as usize];
    println!("OP_CONSTANT_LONG {} '{}'", constant_idx, constant);
    *offset + 4
}

fn simple_instruction(name: &str, offset: &mut usize) -> usize {
    println!("{}", name);
    *offset + 1
}
