pub struct Chip8 {
    // ram
    memory: [u8; 4096],
    stack: [u8; 16],
    sp: u8,

    // cpu
    registers: [u8; 16],
    i: u8,
    pc: u8,
    delay_timer: u8,
    sound_timer: u8,

    // display
    gfx: [[u8; 64]; 32],

    // keyboard
    key: [u8; 16],
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            memory: [0; 4096],
            stack: [0; 16],
            sp: 0,
            registers: [0; 16],
            i: 0x000,
            pc: 0x000,
            delay_timer: 0,
            sound_timer: 0,
            gfx: [[0; 64]; 32],
            key: [0; 16]
        }
    }

    pub fn emulate_cycle() {
        // fetch opcode
        // decode opcode
        // execute opcode
        // update timers
    }
}