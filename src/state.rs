//Emulated CPU state class

use crate::constants;

struct State{
    opcode: u16,        //current opcode
    v: [u8; 16],        //the sixteen 8-bit registers
    i: u16,             //index register
    sound_timer: u8,    //sound timer
    delay_timer: u8,    //delay timer
    pc: u16,            //program counter
    stack: [u16; 16],   //stack
    sp: u8,             //stack pointer
    memory: [u8; 4096], //4kb of memory
    keypad: [u8; 16],   //keypad input
    video: [u32; 64*32] //vram
}

impl State {

    // Read u16 from memory and increment PC twice
    pub fn read_cycle(&mut self) -> u16 {
        self.opcode = self.memory[self.pc as usize] as u16 | self.memory[(self.pc + 1) as usize] as u16;
        self.pc += 2;
        self.opcode
    }

    // return short value for an operation
    pub fn get_u16(&self) -> u16 {
        self.opcode & 0x0FFF
    }

    // return byte value for an operation
    pub fn get_u8(&self) -> u8 {
        (self.opcode & 0xFF) as u8
    }

    // return left register for an operation
    pub fn get_vx(&self) -> u8 {
        ((self.opcode & 0x0F00) >> 8) as u8
    }

    // return right register for an operation
    pub fn get_vy(&self) -> u8 {
        ((self.opcode & 0x00F0) >> 4) as u8
    }
}

impl Default for State {
    fn default() -> State {
        let mut state = State {
            v: [0x0; 16],
            memory: [0x0; 4096],
            i: 0,
            pc: 0x200,
            sp: 0,
            stack: [0x0; 16],
            delay_timer: 0,
            sound_timer: 0,
            keypad: [0; 16],
            video: [0; 64 * 32],
            opcode: 0
        };
        for i in 0..constants::FONTSET.len() {
            state.memory[constants::FONTSET_START + i] = constants::FONTSET[i];
        }
        state
    }
}