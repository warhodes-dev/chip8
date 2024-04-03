use rand::{
    thread_rng, 
    Rng
};
use crate::emu::{
    frame::{
        FB_SIZE,
        Frame,
    },
    keypad::Keypad,
    font::FONT,
};

#[derive(Debug)]
pub struct CPU {
    pub v: [u8; 16],      // Registers
    pub mem: [u8; 4096],  // Memory
    pub stack: [u16; 16], // Stack
    pub dt: u8,           // Delay timer
    pub st: u8,           // Sound timer
    pub i: u16,           // Address register
    pub pc: u16,          // Program counter
    pub sp: u8,           // Stack pointer
    pub kp: Keypad,       // Keypad
    pub fb: Frame,        // Frame
}

#[allow(clippy::new_without_default)]
impl CPU {
    pub fn initialize() -> Self {
        let mut cpu = CPU {
            v: [0; 16],
            mem: [0; 4096],
            stack: [0; 16],
            dt: 0,
            st: 0,
            i:  0,
            pc: 0,
            sp: 0,
            kp: Keypad::new(),
            fb: Frame::new(),
        };
        
        cpu.reset();
        cpu
    }

    pub fn reset(&mut self) {
        let mut mem = [0u8; 4096];
        let (font_region, _) = mem.split_at_mut(0x50);
        font_region.copy_from_slice(&FONT);

        self.v     = [0; 16];
        self.mem   = mem;
        self.stack = [0; 16];
        self.dt    = 0;
        self.st    = 0;
        self.i     = 0;
        self.pc    = 0x200;
        self.sp    = 0;
        self.kp.reset();
        self.fb.reset();
    }

    /// Loads a program's data into mem for execution
    pub fn load(&mut self, rom: &[u8]) {
        let (_, proc_region) = self.mem.split_at_mut(0x200);
        proc_region.copy_from_slice(rom);

        /* ========= CHIP-8 TEST SUITE DEBUG PARAMETERS ========= *
         * ----------- Timendus' chip8-test-suite.ch8 ----------- *
         *                                                        *
         * Set the following memory addresses to configure tests: *
         *                                                        *
         * (0x1FF) Test Select:                                   *
         *     1: IBM Logo                                        *
         *     2: Corax89's opcode test                           *
         *     3: Flags test                                      *
         *     4: Quirks test                                     *
         *     5: Keypad test                                     *
         *                                                        *
         *  (0x1FE) Quirk HW Target:                              *
         *      1: Chip-8                                         *
         *      2: SuperChip 1.1                                  *
         *      3: XO-Chip                                        *
         *                                                        *
         *  (0x1FE) Keypad Test Type:                             *
         *      1: ex9e - Down                                    *
         *      2: ex9e - Up                                      *
         *      3: fx0a - Get key                                 *
         *                                                        *
         *  (Example) Quirk test for std. Chip-8:                 *
         *                                                        *
         *      self.mem[0x1FF] = 4;                              *
         *      self.mem[0x1FE] = 1;                              *
         *                                                        *
         * ==== DEBUGGING PURPOSES ONLY * REMEMBER TO REMOVE ==== */

        //self.mem[0x1ff] = 3;
    }

    /// Fetches opcode, decodes and executes instruction
    pub fn step(&mut self) {
        // Check if we need to block for keypad input first
        if self.kp.block {
            for (key_idx, &key_state) in self.kp.state.iter().enumerate() {
                if key_state {
                    log::trace!("key {} pressed!", key_idx);
                    self.v[self.kp.block_reg] = key_idx as u8;
                    self.kp.block = false;
                    break;
                }
            }
        } else {
            let opcode = self.fetch();
            self.decode_and_execute(opcode);
        }
    }

    /// Progresses the sound and delay timers by 1
    pub fn tick(&mut self) {
        self.dt = self.dt.saturating_sub(1);
        self.st = self.st.saturating_sub(1);
    }

    /// Gets the current speaker state of the cpu
    pub fn sound_state(&self) -> bool {
        self.st > 0 
    }

    fn fetch(&mut self) -> u16 {
        let mem = self.mem;
        let pc = self.pc as usize;
        let opcode = (mem[pc] as u16) << 8 | mem[pc+1] as u16;
        self.pc += 2;
        opcode
    }

    fn decode_and_execute(&mut self, opcode: u16) {
        let op = (
            ((opcode & 0xF000) >> 12) as u8,
            ((opcode & 0x0F00) >> 8) as u8,
            ((opcode & 0x00F0) >> 4) as u8,
            ((opcode & 0x000F) as u8),
        );

        let x   = ((opcode & 0x0F00) >> 8) as usize;
        let y   = ((opcode & 0x00F0) >> 4) as usize;
        let n   = (opcode & 0x000F) as usize;
        let nn  = (opcode & 0x00FF) as usize;
        let nnn = (opcode & 0x0FFF) as usize;

        match op {
            (0x0, 0x0, 0xE, 0x0) => { self.op_00e0() }
            (0x0, 0x0, 0xE, 0xE) => { self.op_00ee() }
            (0x0, _, _, _) => { self.op_0nnn(nnn) }
            (0x1, _, _, _) => { self.op_1nnn(nnn) }
            (0x2, _, _, _) => { self.op_2nnn(nnn) }
            (0x3, _, _, _) => { self.op_3xnn(x, nn) }
            (0x4, _, _, _) => { self.op_4xnn(x, nn) }
            (0x5, _, _, 0x0) => { self.op_5xy0(x, y) }
            (0x6, _, _, _) => { self.op_6xnn(x, nn) }
            (0x7, _, _, _) => { self.op_7xnn(x, nn) }
            (0x8, _, _, 0x0) => { self.op_8xy0(x, y) }
            (0x8, _, _, 0x1) => { self.op_8xy1(x, y) }
            (0x8, _, _, 0x2) => { self.op_8xy2(x, y) }
            (0x8, _, _, 0x3) => { self.op_8xy3(x, y) }
            (0x8, _, _, 0x4) => { self.op_8xy4(x, y) }
            (0x8, _, _, 0x5) => { self.op_8xy5(x, y) }
            (0x8, _, _, 0x6) => { self.op_8xy6(x, y) }
            (0x8, _, _, 0x7) => { self.op_8xy7(x, y) }
            (0x8, _, _, 0xE) => { self.op_8xye(x, y) }
            (0x9, _, _, 0x0) => { self.op_9xy0(x, y) }
            (0xA, _, _, _) => { self.op_annn(nnn) }
            (0xB, _, _, _) => { self.op_bnnn(nnn) }
            (0xC, _, _, _) => { self.op_cxnn(x, nn) }
            (0xD, _, _, _) => { self.op_dxyn(x, y, n) }
            (0xE, _, 0x9, 0xE) => { self.op_ex9e(x) }
            (0xE, _, 0xA, 0x1) => { self.op_exa1(x) }
            (0xF, _, 0x0, 0x7) => { self.op_fx07(x) }
            (0xF, _, 0x0, 0xa) => { self.op_fx0a(x) }
            (0xF, _, 0x1, 0x5) => { self.op_fx15(x) }
            (0xF, _, 0x1, 0x8) => { self.op_fx18(x) }
            (0xF, _, 0x1, 0xE) => { self.op_fx1e(x) }
            (0xF, _, 0x2, 0x9) => { self.op_fx29(x) }
            (0xF, _, 0x3, 0x3) => { self.op_fx33(x) }
            (0xF, _, 0x5, 0x5) => { self.op_fx55(x) }
            (0xF, _, 0x6, 0x5) => { self.op_fx65(x) }
            _ => { panic!("op {:#06x} is unrecognized", opcode); }
        };
    }

    /* Instructions */

    /// OP: Call machine code routine at NNN
    fn op_0nnn(&mut self, _nnn: usize) {
        panic!("opcode 0x0nnn is not supported.")
    }

    /// OP: Clears the screen
    fn op_00e0(&mut self) {
        self.fb.data = [false; FB_SIZE.y * FB_SIZE.x];
        self.fb.update = true;
    }

    /// OP: Returns from a subroutine
    fn op_00ee(&mut self) {
        self.pc = self.stack[self.sp as usize];
        self.sp -= 1;
    }

    /// OP: Jump to addres NNN
    fn op_1nnn(&mut self, nnn: usize) {
        self.pc = nnn as u16;
    }

    /// OP: Calls subroutine at NNN
    fn op_2nnn(&mut self, nnn: usize) {
        self.sp += 1;
        self.stack[self.sp as usize] = self.pc;
        self.pc = nnn as u16;
    }

    /// OP: Skips the next instruction if VX == NN
    fn op_3xnn(&mut self, x: usize, nn: usize) {
        if self.v[x] == nn as u8 {
            self.pc += 2;
        }
    }

    /// OP: Skips the next instruction if VX != NN
    fn op_4xnn(&mut self, x: usize, nn: usize) {
        if self.v[x] != nn as u8 {
            self.pc += 2;
        }
    }

    /// OP: Skips the next instruction if VX == VY
    fn op_5xy0(&mut self, x: usize, y: usize) {
        if self.v[x] == self.v[y] {
            self.pc += 2;
        }
    }

    /// OP: Set VX to NN
    fn op_6xnn(&mut self, x: usize, nn: usize) {
        self.v[x] = nn as u8;
    }

    /// OP: Add NN to VX
    fn op_7xnn(&mut self, x: usize, nn: usize) {
        self.v[x] = self.v[x].wrapping_add(nn as u8);
    }

    /// OP: Sets VX to VY
    fn op_8xy0(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];
    }

    /// OP: Sets VX to (VX OR VY)
    fn op_8xy1(&mut self, x: usize, y: usize) {
        self.v[x] |= self.v[y];
        self.v[0xf] = 0;
    }

    /// OP: Sets VX to (VX AND VY)
    fn op_8xy2(&mut self, x: usize, y: usize) {
        self.v[x] &= self.v[y];
        self.v[0xf] = 0;
    }
    
    /// OP: Sets VX to (VX XOR VY) 
    fn op_8xy3(&mut self, x: usize, y: usize) {
        self.v[x] ^= self.v[y];
        self.v[0xf] = 0;
    }

    /// OP: Adds VY to VX.
    ///     VF = carry
    fn op_8xy4(&mut self, x: usize, y: usize) {
        let (result, wrapped) = self.v[x].overflowing_add(self.v[y]);
        self.v[0xf] = if wrapped { 1 } else { 0 };
        self.v[x] = result;
    }

    /// OP: Subtracts VY from VX
    ///     VF = NOT borrow
    fn op_8xy5(&mut self, x: usize, y: usize) {
        let (result, wrapped) = self.v[x].overflowing_sub(self.v[y]);
        self.v[0xf] = if wrapped { 0 } else { 1 };
        self.v[x] = result;
    }

    /// OP: Shifts VX right by 1
    ///     Stores least signifigant bit of VX in VF
    fn op_8xy6(&mut self, x: usize, _y: usize) {
        let overflow = (self.v[x] & 1) != 0;
        self.v[x] >>= 1;
        self.v[0xf] = if overflow { 1 } else { 0 };
    }

    /// OP: Sets VX to VY - VX
    ///     VF = NOT borrow
    fn op_8xy7(&mut self, x: usize, y: usize) {
        let (result, wrapped) = self.v[y].overflowing_sub(self.v[x]);
        self.v[0xf] = if wrapped { 0 } else { 1 };
        self.v[x] = result;
    }

    /// OP: Shifts VX left by 1
    ///     Stores most signifigant bit of VX in VF
    fn op_8xye(&mut self, x: usize, _y: usize) {
        let overflow = (self.v[x] & (1 << 7)) != 0;
        self.v[x] <<= 1;
        self.v[0xf] = if overflow { 1 } else { 0 };
    }

    /// OP: Skips next instruction if VX != VY
    fn op_9xy0(&mut self, x: usize, y: usize) {
        if self.v[x] != self.v[y] {
            self.pc += 2;
        }
    }
    
    /// OP: Set index register to NNN
    fn op_annn(&mut self, nnn: usize) {
        self.i = nnn as u16;
    }

    /// OP: Jump to address (NNN + V0)
    fn op_bnnn(&mut self, nnn: usize) {
        self.pc = self.v[0] as u16 + nnn as u16;
    }

    /// OP: Set VX to (RNG AND NN)
    fn op_cxnn(&mut self, x: usize, nn: usize) {
        let rand: u8 = thread_rng().gen();
        self.v[x] = rand & nn as u8;
    }

    /// OP: Draw sprite to framebuffer
    ///     Display n-byte sprite starting at register I at (VX, VY), then
    ///     set VF = collision
    fn op_dxyn(&mut self, x: usize, y: usize, n: usize) {
        self.v[0xf] = 0;
        for byte_idx in 0..n as u8 {

            let y = ((self.v[y] + byte_idx) % FB_SIZE.y as u8) as usize;
            
            let byte = self.mem[(self.i + byte_idx as u16) as usize];

            for bit_idx in 0..8 {
                let x = ((self.v[x] + bit_idx) % FB_SIZE.x as u8) as usize;
                let pixel = (byte & (1 << (7 - bit_idx))) != 0;
                if pixel && self.fb.data[y * FB_SIZE.x + x] {
                    self.v[0xf] = 1;
                }
                self.fb.data[y * FB_SIZE.x + x] ^= pixel;
            }
        }
        self.fb.update = true;
    }

    /// OP: Skips the next instruction if key in VX is pressed
    fn op_ex9e(&mut self, x: usize) {
        let key_idx = self.v[x] as usize;
        if self.kp.state[key_idx] {
            self.pc += 2;
        }
    }

    /// OP: Skips the next instruction if key in VX is not pressed
    fn op_exa1(&mut self, x: usize) {
        let key_idx = self.v[x] as usize;
        if !self.kp.state[key_idx] {
            self.pc += 2;
        }
    }
    
    /// OP: Sets VX to delay timer
    fn op_fx07(&mut self, x: usize) {
        self.v[x] = self.dt;
    }

    /// OP: Wait for key press and store in VX [Blocking operation]
    fn op_fx0a(&mut self, x: usize) {
        self.kp.block = true;
        self.kp.block_reg = x;
    }

    /// OP: Set delay timer to VX
    fn op_fx15(&mut self, x: usize) {
        self.dt = self.v[x];
    }

    /// OP: Set sound timer to VX
    fn op_fx18(&mut self, x: usize) {
        self.st = self.v[x];
    }

    /// OP: Adds VX to address register
    fn op_fx1e(&mut self, x: usize) {
        self.i += self.v[x] as u16;
    }

    /// OP: Sets address register to location of sprite for character in VX
    //  These are font characters, and have a height of 5
    fn op_fx29(&mut self, x: usize) {
        self.i = (self.v[x] * 5) as u16;
    }

    /// OP: Stores the binary-coded decimal representation of VX, with the most
    ///     significant of 3 digits at the address in mem[i], the middle digit 
    ///     at mem[i+1], and the least significant digit at mem[i+2].
    fn op_fx33(&mut self, x: usize) {
        let vx = self.v[x];
        for (idx, digit) in vx.to_string().chars().map(|c| c.to_digit(10).unwrap() as u8).enumerate() {
            self.mem[self.i as usize + idx] = digit;
        }
    }

    /// OP: Stores from V0 to VX in mem starting at address register
    ///     I is left unmodified
    fn op_fx55(&mut self, x: usize) {
        for idx in 0..(x+1) {
            self.mem[self.i as usize + idx] = self.v[idx];
        }
    }

    /// OP: Fills from V0 to VX with values from mem, starting at address register
    ///     I is left unmodified
    fn op_fx65(&mut self, x: usize) {
        for idx in 0..(x+1) {
            self.v[idx] = self.mem[self.i as usize + idx];
        }
    }
}
