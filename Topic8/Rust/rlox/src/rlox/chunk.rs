use crate::rlox::value::Value;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum OpCode {
    None = 0b00000000,
    Return,
    Constant,
    ConstantLong,
}

pub struct Chunk {
    pub bytes: Vec<u8>,
    pub values: Vec<Value>,
    line_information: Vec<(u16, u16)>,
    pub name: String,
}

impl Chunk {
    pub fn new(
        bytes: Vec<u8>,
        values: Vec<Value>,
        line_information: Vec<(u16, u16)>,
        name: String,
    ) -> Chunk {
        Chunk {
            bytes: bytes,
            values: values,
            line_information: line_information,
            name: name,
        }
    }
    pub fn write(&mut self, byte: u8, line: u16) {
        self.bytes.push(byte);

        // Update chunk line information
        let last_line_information = self.line_information.last();
        match last_line_information {
            Some(line_information) => {
                if line_information.0.ne(&line) {
                    self.line_information.push((line, 0));
                }
            }
            None => {
                self.line_information.push((line, 0));
            }
        }
        self.line_information.last_mut().unwrap().1 += 1;
    }
    fn add_constant(&mut self, value: Value) -> usize {
        self.values.push(value);
        self.values.len() - 1
    }
    pub fn write_constant(&mut self, value: Value, line: u16) {
        if self.values.len() > 255 {
            let constant_index = self.add_constant(value) as u32;
            self.write(OpCode::ConstantLong.into(), line);
            self.write((constant_index & 0b000000000000000011111111) as u8, line);
            self.write(
                ((constant_index & 0b000000001111111100000000) >> 8) as u8,
                line,
            );
            self.write(
                ((constant_index & 0b111111110000000000000000) >> 16) as u8,
                line,
            );
        } else {
            let constant_index = self.add_constant(value) as u8;
            self.write(OpCode::Constant.into(), line);
            self.write(constant_index, line);
        }
    }
    pub fn get_line(&self, byte_offset: usize) -> u16 {
        let mut current_byte_offset: usize = 0;
        for line_tuple in &self.line_information {
            current_byte_offset += usize::from(line_tuple.1);

            if current_byte_offset > byte_offset {
                return line_tuple.0;
            }
        }
        0
    }
}
