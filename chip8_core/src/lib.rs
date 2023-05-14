/*
    Emulation : Fetch -> Decode -> Execute
 */
use rand::random;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4096;   // 4KB RAM SIZE
const NUM_REGS: usize = 16;     // 16 V Registers
const STACK_SIZE: usize = 16;   // Stack Size
const NUM_KEYS: usize = 16;    // 16 Keys
const FONTSET_SIZE: usize = 80; // 5 Bytes x 16 characters

const FONTSET: [u8; FONTSET_SIZE] = [
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
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

const START_ADDR: u16 = 0x200;  // Application Execution Start Address

// Emulator Core Structure / Object
pub struct Emu {
    pc: u16,                                        // 16bit Program Counter
    ram: [u8; RAM_SIZE],                            // 4KB Memory [Array]
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],   // Screen Data
    v_reg: [u8; NUM_REGS],                          // V Registers
    i_reg: u16,                                     // I Register for Mem Ops
    sp: u16,                                        // Stack pointer
    stack: [u16; STACK_SIZE],                       // CPU Stack for Subroutine
    keys: [bool; NUM_KEYS],                         // Keys
    dt: u8,                                         // Delay Timer
    st: u8,                                         // Sound Timer
}

impl Emu {
    // Initialization function
    pub fn new() -> Self {
        let mut new_emu = Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            dt: 0,
            st: 0,
        };

        new_emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);  // Copy sprite data to ram before returning

        new_emu                                                        
    }

    // Push function for CPU Stack
    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    // Pop function for CPU Stack
    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    pub fn reset(&mut self) {
        self.pc = START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.v_reg = [0; NUM_REGS];
        self.i_reg = 0;
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.dt = 0;
        self.st = 0;
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    // Tick runs every CPU cycle
    pub fn tick(&mut self) {
        // Fetch
        let op = self.fetch();

        // Decode & Execute
        self.execute(op);
    }

    // Opcode fetch
    fn fetch(&mut self) -> u16 {
        let higher_byte = self.ram[self.pc as usize] as u16;
        let lower_byte = self.ram[(self.pc + 1) as usize] as u16;
        let op = (higher_byte << 8) | lower_byte;
        self.pc += 2;
        op
    }

    // Modified every frame
    pub fn tick_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            if self.st == 1 {
                // BEEP : Not doing as part of the tutorial
            }
            self.st -= 1;
        }
    }

    // Decode and execute function
    fn execute(&mut self, op: u16){
        // Separate each digit of opcode
        let digit1 = (op & 0xF000) >> 12;
        let digit2 = (op & 0x0F00) >> 8;
        let digit3 = (op & 0x00F0) >> 4;
        let digit4 = op & 0x000F;

        match (digit1, digit2, digit3, digit4) {
            //NOP
            (0, 0, 0, 0) => return,

            //CLS
            (0, 0, 0xE, 0) => {
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT]
            },

            // RET
            (0, 0, 0xE, 0xE) => {
                let ret_addr = self.pop();  // Pop from CPU stack for function call
                self.pc = ret_addr;
            },

            // JMP NNN
            (1, _, _, _) => {
                let nnn = op & 0xFFF;       // Get the addr from the opcode
                self.pc = nnn;                   // Set PC to addr
            },  

            // CALL NNN
            (2, _, _, _) => {
                let nnn = op & 0xFFF;       
                self.push(self.pc);         // Push PC to Stack
                self.pc = nnn;                   // Set PC to addr
            },

            // SKIP VX == NN
            (3, _, _, _) => {
                let x = digit2 as usize; // Indexing array in rust should be in usize
                let nn = (op & 0xFF) as u8;
                if self.v_reg[x] == nn {
                    self.pc += 2;               // Skip the next opcode i.e., skip 2 bytes
                }
            },

            // SKIP VX != NN
            (4, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                if self.v_reg[x] != nn {
                    self.pc += 2;               // Skip the next opcode i.e., skip 2 bytes
                }
            },

            // SKIP VX == VY
            (5, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.v_reg[x] == self.v_reg[y] {
                    self.pc += 2;
                }
            },

            // VX = NN
            (6, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                self.v_reg[x] = nn;
            },

            // VX += NN
            (7, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                self.v_reg[x] = self.v_reg[x].wrapping_add(nn); // We don't want carry. And overflow will cause panic.
            },

            // VX = VY
            (8, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] = self.v_reg[y];
            },

            // VX |= VY
            (8, _, _, 1) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] |= self.v_reg[y];
            },

            // VX &= VY
            (8, _, _, 2) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] &= self.v_reg[y];
            },

            // VX ^= VY
            (8, _, _, 3) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] ^= self.v_reg[y];
            },

            // VX += VY
            (8, _, _, 4) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                let (new_vx, carry) = self.v_reg[x].overflowing_add(self.v_reg[y]); // Returns wrapping sum if carry is generated
                let new_vf = if carry { 1 } else { 0 };                                   // Value to update carry flag

                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            },

            // VX -= VY
            (8, _, _, 5) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                let (new_vx, borrow) = self.v_reg[x].overflowing_sub(self.v_reg[y]); // Returns wrapping sum if carry is generated
                let new_vf = if borrow { 0 } else { 1 };                                   // Value to update carry flag

                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            }

            // VX >>= 1
            (8, _, _, 6) => {
                let x = digit2 as usize;

                let lsb = self.v_reg[x] & 1;                // Store the LSB before Right Shift
                self.v_reg[x] >>= 1;                            
                self.v_reg[0xF] = lsb;                          // Store the LSB which got dropped into the flag reg
            },

            // VX = VY - VX
            (8, _, _, 7) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                let (new_vx, borrow) = self.v_reg[y].overflowing_sub(self.v_reg[x]); // Returns wrapping sub if carry is generated
                let new_vf = if borrow { 0 } else { 1 }; 

                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            },

            // VX <<= 1
            (8, _, _, 0xE) => {
                let x = digit2 as usize;

                let msb = (self.v_reg[x] >> 7) & 1;         // Store the MSB before Left Shift
                self.v_reg[x] <<= 1;                            
                self.v_reg[0xF] = msb;                          // Store the MSB which got dropped into the flag reg
            },

            // SKIP VX != VY
            (9, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                if self.v_reg[x] != self.v_reg[y] {
                    self.pc += 2;
                }
            },

            // I - NNN
            (0xA, _, _, _) => {
                let nnn = op & 0xFFF;
                self.i_reg = nnn;
            },

            // JMP V0 + NNN
            (0xB, _, _, _) => {
                let nnn = op & 0xFFF;
                self.pc = (self.v_reg[0] as u16) + nnn;
            },

            // VX = rand() & NN
            (0xC, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                let rng: u8 = random();
                self.v_reg[x] = rng & nn;
            },

            // Draw Sprite at (VX, VY) of Height N
            (0xD, _, _, _) => {
                // Get (x,y) coordinate for our sprite
                let x_coord = self.v_reg[digit2 as usize] as u16;
                let y_coord = self.v_reg[digit3 as usize] as u16;

                // The last digit determines height of the sprite
                let num_rows = digit4;

                // Keep track of flipped pixels
                let mut flipped = false;

                // Iterate over each row of our sprite
                for y_line in 0..num_rows {
                    // Determine memory address containing this row data
                    let addr = self.i_reg + y_line as u16;
                    let pixels = self.ram[addr as usize];

                    // Iterate over each column in our row
                    for x_line in 0..8 {
                        // Use a mask to fetch current pixel's bit. Only flip if a 1
                        if (pixels & (0b1000_0000 >> x_line)) != 0 {
                            // Sprites should wrap around screen, so apply modulo
                            let x = (x_coord + x_line) as usize % SCREEN_WIDTH;
                            let y = (y_coord + y_line) as usize % SCREEN_HEIGHT;

                            // Get pixels's index for our 1D screen array
                            let idx = x + SCREEN_WIDTH * y;

                            // Check if we are about to flip the pixel and set
                            flipped |= self.screen[idx];
                            self.screen[idx] ^= true;
                        }
                    }
                }

                // Populate VF register
                if flipped {
                    self.v_reg[0xF] = 1;
                }
                else{
                    self.v_reg[0xF] = 0;
                }
            },

            // SKIP KEY PRESS
            (0xE, _, 0x9, 0xE) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x];
                let key = self.keys[vx as usize];
                if key {
                    self.pc += 2;
                }
            },

            // SKIP KEY RELEASE
            (0xE, _, 0xA, 1) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x];
                let key = self.keys[vx as usize];
                if !key {
                    self.pc += 2;
                }
            },

            // VX = DT
            (0xF, _, 0, 7) => {
                let x = digit2 as usize;
                self.v_reg[x] = self.dt;
            },

            // WAIT KEY
            (0xF, _, 0, 0xA) => {
                let x = digit2 as usize;
                let mut pressed = false;
                for i in 0..self.keys.len() {
                    if self.keys[i] {
                        self.v_reg[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }

                if !pressed {
                    // Redo opcode
                    self.pc -= 2;                       // Resetting to last instruction because we want to capture the user input. Simply running a loop would cause it to never end
                }
            },

            // DT = VX
            (0xF, _, 1, 5) => {
                let x = digit2 as usize;
                self.dt = self.v_reg[x];
            },

            // ST = VX
            (0xF, _, 1, 8) => {
                let x = digit2 as usize;
                self.st = self.v_reg[x];
            },

            // I += VX
            (0xF, _, 1, 0xE) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x] as u16;
                self.i_reg = self.i_reg.wrapping_add(vx);
            },

            // I = FONT
            (0xF, _, 2, 9) => {
                let x = digit2 as usize;
                let c = self.v_reg[x] as u16;
                self.i_reg = c * 5;                     // Our numbers are stored from 0 address and each number sprite is 5 bytes
            },

            // BCD
            (0xF, _, 3, 3) => {                         // This is not the best BCD algorithm. Just for understanding.
                let x = digit2 as usize;
                let vx = self.v_reg[x] as f32;

                // Fetch the hundreds digit
                let hundreds = (vx / 100.0).floor() as u8;

                // Fetch the tens digit
                let tens = ((vx / 10.0) % 10.0).floor() as u8;

                // Fetch the ones digit
                let ones = (vx % 10.0) as u8;

                self.ram[self.i_reg as usize] = hundreds;
                self.ram[(self.i_reg + 1) as usize] = tens;
                self.ram[(self.i_reg + 2) as usize] = ones;
            },

            // STORE V0 - VX
            (0xF, _, 5, 5) => {
                let x = digit2 as usize;
                let i = self.i_reg as usize;
                for idx in 0..=x {
                    self.ram[i + idx] = self.v_reg[idx];
                }
            },

            // LOAD V0 - VX
            (0xF, _, 6, 5) => {
                let x = digit2 as usize;
                let i = self.i_reg as usize;
                for idx in 0..=x {
                    self.v_reg[idx] = self.ram[i + idx];
                }
            },

            //Unimplemented Case : Mandatory for RUST
            (_, _, _, _) => unimplemented!("Unimplemented opcode: {}", op),
        }
    }

    // Returns display array to frontend
    pub fn get_display(&self) -> &[bool] {
        &self.screen
    }

    // Handles keypress
    pub fn keypress(&mut self, idx: usize, pressed: bool) {
        self.keys[idx] = pressed;
    }

    pub fn load(&mut self, data: &[u8]) {
        let start = START_ADDR as usize;
        let end = (START_ADDR as usize) + data.len();
        self.ram[start..end].copy_from_slice(data);
    }
}
