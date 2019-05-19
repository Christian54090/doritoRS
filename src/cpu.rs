pub struct Cpu {
    registers: [u8; 16],
    i: u8,
    pub pc: u16,
    delay_timer: u8,
    sound_timer: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: [0; 16],
            i: 0x000,
            pc: 0x200,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn emulate_cycle(&self, opcode: u16) {
        match opcode & 0xF000 {
            0x0000 => {},
            0x1000 => {},
            0x2000 => {},
            0x3000 => {},
            0x4000 => {},
            0x5000 => {},
            0x6000 => {},
            0x7000 => {},
            0x8000 => {},
            0x9000 => {},
            0xA000 => {},
            0xB000 => {},
            0xC000 => {},
            0xD000 => {},
            0xE000 => {},
            0xF000 => {},
            _ => panic!("Unknown opcode: {:#X}", opcode),
        }
        // update timers
    }
}