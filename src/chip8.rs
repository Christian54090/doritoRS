pub struct Chip8 {
    // ram
    memory: [u8; 4096],

    // cpu
    registers: [u8; 16],
    i: u8,
    pc: u8,

    // display
    gfx: [[i16; 64]; 32],
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            memory: [0; 4096],
            registers: [0; 16],
            i: 0x000,
            pc: 0x000,
            gfx: [[0; 64]; 32],
        }
    }

    pub fn emulate_cycle() {
        // fetch opcode
        // decode opcode
        // execute opcode
        // update timers
    }
}