use std::io;
use std::io::prelude::*;
use std::fs::File;
fn main() {

    let mut file = File::open("roms/test_opcode.ch8");

    let mut buf: Vec<u8> = Vec::new();

    let a = match file {
        Ok(mut f) => f.read_to_end(&mut buf),
        Err(e) => Err(e)
    };





    for b in buf {
        print!("{:x} ", b);
    }
}
