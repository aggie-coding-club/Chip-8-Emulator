//Emulated CPU state class

struct State{
    opcode: u16,        //current opcode
    v: [u8; 16],        //the sixteen 8-bit registers
    i: u16,             //index register
    sound_timer: u8,    //sound timer
    delay_timer: u8,    //delay timer
    pc: u16,            //program counter
    sp: u8,             //stack pointer
    memory: [u8, 4096]  //4kb of memory
}
