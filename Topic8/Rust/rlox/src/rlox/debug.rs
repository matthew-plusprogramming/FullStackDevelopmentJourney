use crate::Chunk;
use crate::OpCode;
use std::convert::TryFrom;

pub fn disassemble_chunk(chunk: &Chunk) {
    print!("== {} ==\n", chunk.name);
    let mut offset: usize = 0;
    loop {
        offset = disassemble_instruction(chunk, offset);
        if offset >= chunk.bytes.len() {
            break;
        }
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    print!("{}\n", name);
    offset + 1
}
fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant_index: usize = chunk.bytes[offset + 1].into();
    print!(
        "{:-16} {:4} '{}'\n",
        name, constant_index, chunk.values[constant_index]
    );
    offset + 2
}
fn constant_long_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant_index: usize = (chunk.bytes[offset + 1] as u32
        | (chunk.bytes[offset + 2] as u32) << 8
        | (chunk.bytes[offset + 3] as u32) << 16) as usize;
    print!(
        "{:-16} {:4} '{}'\n",
        name, constant_index, chunk.values[constant_index]
    );
    offset + 4
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);
    if offset > 0 && chunk.get_line(offset) == chunk.get_line(offset - 1) {
        print!("   | ");
    } else {
        print!("{:04} ", chunk.get_line(offset));
    }
    let current_instruction = OpCode::try_from(chunk.bytes[offset]).unwrap_or(OpCode::None);

    match current_instruction {
        OpCode::Return => return simple_instruction("OP_RETURN", offset),
        OpCode::Constant => return constant_instruction("OP_CONSTANT", chunk, offset),
        OpCode::ConstantLong => {
            return constant_long_instruction("OP_CONSTANT_LONG", chunk, offset)
        }
        OpCode::None => {}
    }
    0
}
