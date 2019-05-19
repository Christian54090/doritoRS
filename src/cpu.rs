pub struct Cpu {
    registers: [u8; 16],
    i: u8,
    pc: u8,
    delay_timer: u8,
    sound_timer: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: [0; 16],
            i: 0x000,
            pc: 0x000,
            delay_timer: 0,
            sound_timer: 0,
        }
    }
}