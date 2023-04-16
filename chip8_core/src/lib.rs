/*
    Emulation : Fetch -> Decode -> Execute
 */

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

        // Decode

        // Execute
    }

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
                // BEEP
            }
            self.st -= 1;
        }
    }
}