const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

#[derive(Debug)]
pub struct Chip {
    pc: u16,
    ram: [u8; RAM_SIZE],
    display: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    v_regs: [u8; NUM_REGS],
    i_reg: u16,
    stack: [u16; STACK_SIZE],
    sp: u16,
    keys: [bool; NUM_KEYS],
    dt: u8,
    st: u8,
}

const START_ADDR: u16 = 0x200;
const FONT_START_ADDR: u16 = 0x50;
const FONT_SIZE: usize = 80;
const FONT: [u8; FONT_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

type Op = [u8; 4];

trait Addr {
    fn nn(&self) -> u8;
    fn nnn(&self) -> u16;
}

impl Addr for Op {
    fn nn(&self) -> u8 {
        self[2] << 4 | self[3]
    }
    fn nnn(&self) -> u16 {
        (self[1] as u16) << 8 | (self[2] as u16) << 4 | self[3] as u16
    }
}

impl Chip {
    pub fn new() -> Self {
        let mut chip = Self {
            pc: START_ADDR,
            ram: ([0; RAM_SIZE]),
            display: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            v_regs: [0; NUM_REGS],
            i_reg: 0,
            stack: [0; STACK_SIZE],
            sp: 0,
            keys: [false; NUM_KEYS],
            dt: 0,
            st: 0,
        };
        chip.load_font();

        chip
    }

    pub fn reset(&mut self) {
        self.pc = START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.display = [false; DISPLAY_WIDTH * DISPLAY_HEIGHT];
        self.v_regs = [0; NUM_REGS];
        self.i_reg = 0;
        self.stack = [0; STACK_SIZE];
        self.sp = 0;
        self.keys = [false; NUM_KEYS];
        self.dt = 0;
        self.st = 0;
        self.load_font();
    }

    // The function returns a boolean to indicate a beep
    pub fn tick_timers(&mut self) -> bool {
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            self.st -= 1;
            // Beep
            true
        } else {
            // No beep
            false
        }
    }

    pub fn tick(&mut self) {
        let op = self.fetch();
        // Increment PC to point to the next instruction
        self.pc += 2;
        // Execute the current instruction
        self.execute(op);
    }

    fn fetch(&mut self) -> Op {
        let hi = self.ram[self.pc as usize];
        let lo = self.ram[(self.pc + 1) as usize];
        [(hi & 0xF0) >> 4, hi & 0x0F, (lo & 0xF0) >> 4, lo & 0x0F]
    }

    fn execute(&mut self, op: Op) {
        match op {
            // No-op
            [0, 0, 0, 0] => (),
            // Clear screen
            [0, 0, 0xE, 0] => self.display = [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            // Return from subroutine
            [0, 0, 0xE, 0xE] => self.pc = self.pop(),
            // Jump
            [1, _, _, _] => self.pc = op.nnn(),
            // Call subroutine
            [2, _, _, _] => {
                self.push(self.pc);
                self.pc = op.nnn();
            }
            // Skip next if VX == NN
            [3, x, _, _] => {
                if self.v_regs[x as usize] == op.nn() {
                    self.pc += 2;
                }
            }
            // Skip next if VX != NN
            [4, x, _, _] => {
                if self.v_regs[x as usize] != op.nn() {
                    self.pc += 2;
                }
            }
            // Skip next if VX == VY
            [5, x, y, 0] => {
                if self.v_regs[x as usize] == self.v_regs[y as usize] {
                    self.pc += 2;
                }
            }
            // VX = NN
            [6, x, _, _] => self.v_regs[x as usize] = op.nn(),
            // VX += NN
            [7, x, _, _] => self.v_regs[x as usize] = self.v_regs[x as usize].wrapping_add(op.nn()),
            // VX = VY
            [8, x, y, 0] => self.v_regs[x as usize] = self.v_regs[y as usize],
            // VX |= VY
            [8, x, y, 1] => {
                self.v_regs[x as usize] |= self.v_regs[y as usize];
                self.v_regs[0xF] = 0;
            }
            // VX &= VY
            [8, x, y, 2] => {
                self.v_regs[x as usize] &= self.v_regs[y as usize];
                self.v_regs[0xF] = 0;
            }
            // VX ^= VY
            [8, x, y, 3] => {
                self.v_regs[x as usize] ^= self.v_regs[y as usize];
                self.v_regs[0xF] = 0;
            }
            // VX += VY
            [8, x, y, 4] => {
                let (val, carry) = self.v_regs[x as usize].overflowing_add(self.v_regs[y as usize]);
                self.v_regs[x as usize] = val;
                self.v_regs[0xF] = if carry { 1 } else { 0 };
            }
            // VX - VY
            [8, x, y, 5] => {
                let (val, borrow) =
                    self.v_regs[x as usize].overflowing_sub(self.v_regs[y as usize]);
                self.v_regs[x as usize] = val;
                self.v_regs[0xF] = if borrow { 0 } else { 1 };
            }
            // VX >> 1
            [8, x, y, 6] => {
                self.v_regs[x as usize] = self.v_regs[y as usize] >> 1;
                self.v_regs[0xF] = self.v_regs[y as usize] & 1;
            }
            // VX = VY - VX
            [8, x, y, 7] => {
                let (val, borrow) =
                    self.v_regs[y as usize].overflowing_sub(self.v_regs[x as usize]);
                self.v_regs[x as usize] = val;
                self.v_regs[0xF] = if borrow { 0 } else { 1 };
            }
            // VX << 1
            [8, x, y, 0xE] => {
                self.v_regs[x as usize] = self.v_regs[y as usize] << 1;
                self.v_regs[0xF] = (self.v_regs[y as usize] >> 7) & 1;
            }
            // Skip next if VX != VY
            [9, x, y, 0] => {
                if self.v_regs[x as usize] != self.v_regs[y as usize] {
                    self.pc += 2;
                }
            }
            // I = NNN
            [0xA, _, _, _] => self.i_reg = op.nnn(),
            // Jump to V0 + NNN
            [0xB, _, _, _] => self.pc = self.v_regs[0] as u16 + op.nnn(),
            // VX = rand() & NN
            [0xC, x, _, _] => self.v_regs[x as usize] = rand::random::<u8>() & op.nn(),
            // Draw at (VX, VY)
            [0xD, x, y, rows] => {
                let mut inverted = false;
                // If the entire sprite is located beyond the screen,
                // then it is wrapped around and rendered.
                let x_start = self.v_regs[x as usize] as usize % DISPLAY_WIDTH;
                let y_start = self.v_regs[y as usize] as usize % DISPLAY_HEIGHT;
                for row in 0..rows {
                    let addr = self.i_reg + row as u16;
                    let pixels = self.ram[addr as usize];
                    for col in 0..8 {
                        if (pixels & (0b1000_0000) >> col) != 0 {
                            let x_offset = x_start + col;
                            let y_offset = y_start + row as usize;
                            // If the pixel is beyond the screen, then it is clipped
                            if x_offset < DISPLAY_WIDTH && y_offset < DISPLAY_HEIGHT {
                                let idx = x_offset + DISPLAY_WIDTH * y_offset;
                                inverted |= self.display[idx];
                                self.display[idx] ^= true;
                            }
                        }
                    }
                }

                self.v_regs[0xF] = if inverted { 1 } else { 0 };
            }
            // Skip if keypress
            [0xE, x, 9, 0xE] => {
                if self.keys[self.v_regs[x as usize] as usize] {
                    self.pc += 2
                }
            }
            // Skip if not keypress
            [0xE, x, 0xA, 1] => {
                if !self.keys[self.v_regs[x as usize] as usize] {
                    self.pc += 2
                }
            }
            // VX = DT
            [0xF, x, 0, 7] => self.v_regs[x as usize] = self.dt,
            // Wait for keypress, store index in VX
            [0xF, x, 0, 0xA] => {
                let mut pressed = false;
                for i in 0..self.keys.len() {
                    if self.keys[i] {
                        self.v_regs[x as usize] = i as u8;
                        pressed = true;
                        break;
                    }
                }
                if !pressed {
                    // Repeat instruction
                    self.pc -= 2;
                }
            }
            // DT = VX
            [0xF, x, 1, 5] => self.dt = self.v_regs[x as usize],
            // ST = VX
            [0xF, x, 1, 8] => self.st = self.v_regs[x as usize],
            // I += VX
            [0xF, x, 1, 0xE] => {
                self.i_reg = self.i_reg.wrapping_add(self.v_regs[x as usize] as u16)
            }
            // I = FONT_ADDR
            // Each font character is 5 bytes, and the offset is simply 5 * value
            [0xF, x, 2, 9] => self.i_reg = FONT_START_ADDR + self.v_regs[x as usize] as u16 * 5,
            [0xF, x, 3, 3] => {
                let val = self.v_regs[x as usize];
                (
                    self.ram[self.i_reg as usize],
                    self.ram[(self.i_reg + 1) as usize],
                    self.ram[(self.i_reg + 2) as usize],
                ) = (val / 100, (val / 10) % 10, val % 10);
            }
            // Store V0-VX at I
            [0xF, x, 5, 5] => {
                for i in 0..=x as usize {
                    self.ram[self.i_reg as usize] = self.v_regs[i];
                    self.i_reg += 1;
                }
            }
            // Load I into V0-VX
            [0xF, x, 6, 5] => {
                for i in 0..=x as usize {
                    self.v_regs[i] = self.ram[self.i_reg as usize];
                    self.i_reg += 1;
                }
            }
            [_, _, _, _] => unimplemented!("invalid opcode"),
        }
    }

    pub fn display(&self) -> &[bool] {
        &self.display
    }

    // The function returns a boolean to indicate if the
    // CPU should wait for the key to be released before
    // proceeding to the next instruction. This is used
    // for the FX0A instruction, which completes execution
    // upon a key being released, rather than pressed.
    pub fn key_down(&mut self, idx: usize) -> bool {
        self.keys[idx] = true;
        if matches!(self.fetch(), [0xF, _, 0, 0xA]) {
            true
        } else {
            false
        }
    }

    pub fn key_up(&mut self, idx: usize) {
        self.keys[idx] = false;
    }

    pub fn load(&mut self, data: &[u8]) {
        self.ram[START_ADDR as usize..START_ADDR as usize + data.len()].copy_from_slice(data);
    }

    fn load_font(&mut self) {
        self.ram[FONT_START_ADDR as usize..FONT_START_ADDR as usize + FONT_SIZE]
            .copy_from_slice(&FONT);
    }

    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }
}
