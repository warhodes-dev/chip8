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

        let nn  = (opcode & 0x00FF) as u8;
        let nnn = (opcode & 0x0FFF) as u16;

        match op {
            (0x0, 0x0, 0xE, 0x0) => { self.op_00e0() }
            (0x0, 0x0, 0xE, 0xE) => { self.op_00ee() }
            (0x0, _, _, _) => { self.op_0nnn(nnn) }

            (0x1, _, _, _) => { self.op_1nnn(nnn) }
            (0x2, _, _, _) => { self.op_2nnn(nnn) }
            (0x3, x, _, _) => { self.op_3xnn(x, nn) }
            (0x4, x, _, _) => { self.op_3xnn(x, nn) }
            (0x5, x, y, 0x0) => { self.op_5xy0(x, y) }
            (0x6, x, _, _) => { self.op_6xnn(x, nn) }
            (0x7, x, _, _) => { self.op_7xnn(x, nn) }
            (0x8, x, y, 0x0) => { self.op_8xy0(x, y) }
            (0x8, x, y, 0x1) => { self.op_8xy1(x, y) }
            (0x8, x, y, 0x2) => { self.op_8xy2(x, y) }
            (0x8, x, y, 0x3) => { self.op_8xy3(x, y) }
            (0x8, x, y, 0x4) => { self.op_8xy4(x, y) }
            (0x8, x, y, 0x5) => { self.op_8xy5(x, y) }
            (0x8, x, y, 0x6) => { self.op_8xy6(x, y) }
            (0x8, x, y, 0x7) => { self.op_8xy7(x, y) }
            (0x8, x, y, 0xE) => { self.op_8xye(x, y) }
            (0x9, x, y, 0x0) => { self.op_9xy0(x, y) }
            (0xA, _, _, _) => { self.op_annn(nnn) }
            (0xB, _, _, _) => { self.op_bnnn(nnn) }
            (0xC, x, _, _) => { self.op_cxnn(x, nn) }
            (0xD, x, y, n) => { self.op_dxyn(x, y, n) }
            (0xE, x, 0x9, 0xE) => { self.op_ex9e(x) }
            (0xE, x, 0xA, 0x1) => { self.op_exa1(x) }
            (0xF, x, 0x0, 0x7) => { self.op_fx07(x) }
            (0xF, x, 0x0, 0xa) => { self.op_fx0a(x) }
            (0xF, x, 0x1, 0x5) => { self.op_fx15(x) }
            (0xF, x, 0x1, 0x8) => { self.op_fx18(x) }
            (0xF, x, 0x1, 0xE) => { self.op_fx1e(x) }
            (0xF, x, 0x2, 0x9) => { self.op_fx29(x) }
            (0xF, x, 0x3, 0x3) => { self.op_fx33(x) }
            (0xF, x, 0x5, 0x5) => { self.op_fx55(x) }
            (0xF, x, 0x6, 0x5) => { self.op_fx65(x) }
            _ => { log::error!("op {:#06x} not implemented", opcode); }
        }

    }

    /* Instructions */

    /// OP: Call machine code routine at NNN
    fn op_0nnn(&mut self, nnn: u16) {
        unimplemented!()
    }

    /// OP: Clears the screen
    fn op_00e0(&mut self) {
        self.frame.buf = [[false; FB_SIZE.y]; FB_SIZE.x];
        self.frame_update = true;
    }

    /// OP: Returns from a subroutine
    fn op_00ee(&mut self) {
        unimplemented!()
    }

    /// OP: Jump to addres NNN
    fn op_1nnn(&mut self, nnn: u16) {
        self.program_counter = nnn;
    }

    /// OP: Calls subroutine at NNN
    fn op_2nnn(&mut self, nnn: u16) {
        unimplemented!()
    }

    /// OP: Skips the next instruction if VX == NN
    fn op_3xnn(&mut self, x: u8, nn: u8) {
        unimplemented!()
    }

    /// OP: Skips the next instruction if VX != NN
    fn op_4xnn(&mut self, x: u8, nn: u8) {
        unimplemented!()
    }

    /// OP: Skips the next instruction if VX == VY
    fn op_5xy0(&mut self, x: u8, y: u8) {
        unimplemented!()
    }

    /// OP: Set VX to NN
    fn op_6xnn(&mut self, x: u8, nn: u8) {
        self.registers[x as usize] = nn as u8;
    }

    /// OP: Add NN to VX
    fn op_7xnn(&mut self, x: u8, nn: u8) {
        self.registers[x as usize] += nn as u8;
    }

    /// OP: Sets VX to VY
    fn op_8xy0(&mut self, x: u8, y: u8) {
        unimplemented!()
    }

    /// OP: Sets VX to (VX OR VY)
    fn op_8xy1(&mut self, x: u8, y: u8) {
        unimplemented!()
    }

    /// OP: Sets VX to (VX AND VY)
    fn op_8xy2(&mut self, x: u8, y: u8) {
        unimplemented!()
    }
    
    /// OP: Sets VX to (VX XOR VY) 
    fn op_8xy3(&mut self, x: u8, y: u8) {
        unimplemented!()
    }

    /// OP: Adds VY to VX.
    ///     VF is set to 1 if there's a carry, and 0 if not
    fn op_8xy4(&mut self, x: u8, y: u8) {
        unimplemented!()
    }

    /// OP: Subtracts VY from VX
    ///     VF is set to 1 if there's a borrow, and 0 if not
    fn op_8xy5(&mut self, x: u8, y: u8) {
        unimplemented!()
    }

    /// OP: Shifts VX left by 1
    ///     Stores least signifigant bit of VX in VF
    fn op_8xy6(&mut self, x: u8, y: u8) {
        unimplemented!()
    }

    /// OP: Sets VX to VY - VX
    ///     VF is set to 0 when there's a borrow, and 0 if not
    fn op_8xy7(&mut self, x: u8, y: u8) {
        unimplemented!()
    }

    /// OP: Shifts VX right by 1
    ///     Stores most signifigant bit of VX in VF
    fn op_8xye(&mut self, x: u8, y: u8) {
        unimplemented!()
    }

    /// OP: Skips next instruction if VX != VY
    fn op_9xy0(&mut self, x: u8, y: u8) {
        unimplemented!()
    }
    
    /// OP: Set index register to NNN
    fn op_annn(&mut self, nnn: u16) {
        self.index_register = nnn;
    }

    /// OP: Jump to address (NNN + V0)
    fn op_bnnn(&mut self, nnn: u16) {
        unimplemented!()
    }

    /// OP: Set VX to (RNG AND NN)
    fn op_cxnn(&mut self, x: u8, nn: u8) {
        unimplemented!()
    }

    /// OP: Draw sprite to framebuffer
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

    /// OP: Skips the next instruction if key in VX is pressed
    fn op_ex9e(&mut self, x: u8) {
        unimplemented!()
    }

    /// OP: Skips the next instruction if key in VX is not pressed
    fn op_exa1(&mut self, x: u8) {
        unimplemented!()
    }
    
    /// OP: Sets VX to delay timer
    fn op_fx07(&mut self, x: u8) {
        unimplemented!()
    }

    /// OP: Wait for key press and store in VX [Blocking operation]
    fn op_fx0a(&mut self, x: u8) {
        unimplemented!()
    }

    /// OP: Set delay timer to VX
    fn op_fx15(&mut self, x: u8) {
        unimplemented!()
    }

    /// OP: Set sound timer to VX
    fn op_fx18(&mut self, x: u8) {
        unimplemented!()
    }

    /// OP: Adds VX to address register
    fn op_fx1e(&mut self, x: u8) {
        unimplemented!()
    }

    /// OP: Sets address register to location of sprite for character in VX
    //      ( Accesses the font data )
    fn op_fx29(&mut self, x: u8) {
        unimplemented!()
    }

    /// OP: Stores the binary-coded decimal representation of VX, with the most
    ///     significant of 3 digits at the address in the address register, the
    ///     middle digit at the address register + 1, and the least significant
    ///     digit at the address register + 2.
    //      In other words, take the decimal representation of VX, place the
    //      hundreds digit in memory at location in I, the tens digit in at
    //      location I+1, and the ones digit at location I+2
    fn op_fx33(&mut self, x: u8) {
        unimplemented!()
    }

    /// OP: Stores from V0 to VX in memory starting at address register
    ///     I is left unmodified
    fn op_fx55(&mut self, x: u8) {
        unimplemented!()
    }

    /// OP: Fills from V0 to VX with values from memory, starting at address register
    ///     I is left unmodified
    fn op_fx65(&mut self, x: u8) {
        unimplemented!()
    }
}
