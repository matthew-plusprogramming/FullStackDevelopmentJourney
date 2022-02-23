use crate::rlox::chunk::Chunk;

pub struct VM<'a> {
    pub ip: Option<&'a u8>,
    pub chunk: Option<&'a Chunk>,
}
pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

impl<'a> VM<'a> {
    pub fn new() -> VM<'a> {
        VM {
            ip: None,
            chunk: None,
        }
    }
    pub fn interpret() -> InterpretResult {
        InterpretResult::InterpretOk
    }
    fn run() -> InterpretResult {
        InterpretResult::InterpretOk
    }
}
