use crate::emu::{
    frame::{
        FB_SIZE,
        Frame,
    },
    keypad::Keypad,
};

pub struct CPU {
    registers: [u8; 16],
    memory: [u8; 4096],
    stack: [u16; 16],
    delay_timer: u8,
    sound_timer: u8,
    index_register: u16,
    program_counter: u16,
    stack_pointer: u8,
    pub keypad: Keypad,
    pub frame: Frame,
    pub frame_update: bool,
}

#[allow(clippy::new_without_default)]
impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: [0; 16],
            memory: [0; 4096],
            stack: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            index_register:  0x200,
            program_counter: 0x200,
            stack_pointer: 0,
            keypad: Keypad::new(),
            frame: Frame::new(),
            frame_update: false,
        }
    }

    /// Loads a program's data into memory for execution
    pub fn load(&mut self, rom: &[u8]) {
        let (_, proc_region) = self.memory.split_at_mut(0x200);
        proc_region.copy_from_slice(rom);
    }

    /// Fetches opcode, decodes and executes instruction
    pub fn step(&mut self) {
        let opcode = self.fetch_opcode();
        self.decode_and_execute(opcode);
    }

    fn fetch_opcode(&mut self) -> u16 {
        let mem = self.memory;
        let pc = self.program_counter as usize;
        let opcode = (mem[pc] as u16) << 8 | mem[pc+1] as u16;
        self.program_counter += 2;
        opcode
    }

    fn decode_and_execute(&mut self, opcode: u16) {
        let op = (
            ((opcode & 0xF000) >> 12) as u8,
            ((opcode & 0x0F00) >> 8) as u8,
            ((opcode & 0x00F0) >> 4) as u8,
            ((opcode & 0x000F) as u8),
        );

        let n   = (opcode & 0x000F) as u8;
        let nn  = (opcode & 0x00FF) as u8;
        let nnn = (opcode & 0x0FFF) as u16;
        
        match op {
            (0x0, 0x0, 0xE, 0x0) => { self.op_00e0() }
            (0x1, _, _, _) => { self.op_1nnn(nnn) }
            (0x6, x, _, _) => { self.op_6xnn(x, nn) }
            (0x7, x, _, _) => { self.op_7xnn(x, nn) }
            (0xA, _, _, _) => { self.op_annn(nnn) }
            (0xD, x, y, _) => { self.op_dxyn(x, y, n) }
            _ => { log::error!("op {:#06x} not implemented", opcode); }
        }

    }

    /* Instructions */

    /// OP: Clear screen
    fn op_00e0(&mut self) {
        self.frame.buf = [[false; FB_SIZE.y]; FB_SIZE.x];
        self.frame_update = true;
    }

    /// OP: Jump
    fn op_1nnn(&mut self, nnn: u16) {
        self.program_counter = nnn;
    }

    /// OP: Set VX
    fn op_6xnn(&mut self, x: u8, nn: u8) {
        self.registers[x as usize] = nn as u8;
    }

    /// OP: Add to VX
    fn op_7xnn(&mut self, x: u8, nn: u8) {
        self.registers[x as usize] += nn as u8;
    }

    /// OP: Set index register
    fn op_annn(&mut self, nnn: u16) {
        self.index_register = nnn;
    }

    /// OP: Draw sprite to framebuffer
    // TODO: wrap with mod?
    fn op_dxyn(&mut self, x: u8, y: u8, n: u8) {
        // reset collision register
        self.registers[0xf] = 0;
        for byte_idx in 0..n {
            let vy = (self.registers[y as usize] + byte_idx) % FB_SIZE.y as u8;
            let byte = self.memory[(self.index_register + byte_idx as u16) as usize];
            for bit_idx in 0..8 {
                let vx = (self.registers[x as usize] + bit_idx) % FB_SIZE.x as u8;
                let pixel = (byte & (1 << (7 - bit_idx))) != 0;
                if pixel && self.frame.buf[vx as usize][vy as usize] {
                    self.registers[0xf] = 1;
                }
                self.frame.buf[vx as usize][vy as usize] ^= pixel;
            }
        }
        self.frame_update = true;
    }
}










//
