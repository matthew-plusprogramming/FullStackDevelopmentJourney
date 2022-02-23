use rlox::disassemble_chunk;

extern crate rlox;

fn main() {
    // Basic test
    let mut test = rlox::Chunk::new(Vec::new(), Vec::new(), Vec::new(), String::from("Test"));
    test.write(rlox::OpCode::Return.into(), 0);
    test.write(rlox::OpCode::Return.into(), 0);
    test.write(rlox::OpCode::Return.into(), 1);
    test.write(rlox::OpCode::Return.into(), 1);
    test.write(rlox::OpCode::Return.into(), 1);
    test.write(rlox::OpCode::Return.into(), 10);
    test.write_constant(10.0, 10);
    let mut count = 0;

    loop {
        count += 1;
        test.write_constant(10.0, 10);
        if count > 256 {
            break;
        }
    }
    disassemble_chunk(&test)
}
