pub struct Cpu {
    registers: [u8; 16],
    pub pc: u16,
    i: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: [0; 16],
            pc: 0x200,
            i: 0x000,
        }
    }

    pub fn execute_opcode(&self, opcode: u16, ram: Ram) {
        match opcode & 0xF000 {
            0x0000 => {
                match opcode & 0x000F {
                    0x0000 => {},
                    0x000E => {},
                    _ => panic!("Unknown opcode [0x0000]: {:#X}", opcode),
                }
            },
            0x1000 => {},
            0x2000 => {},
            0x3000 => {},
            0x4000 => {},
            0x5000 => {},
            0x6000 => {},
            0x7000 => {},
            0x8000 => {
                match opcode & 0x000F {
                    0x0000 => {},
                    0x0001 => {},
                    0x0002 => {},
                    0x0003 => {},
                    0x0004 => {},
                    0x0005 => {},
                    0x0006 => {},
                    0x0007 => {},
                    0x000E => {},
                    _ => panic!("Unknown opcode [0x8000]: {:#X}", opcode),
                }
            },
            0x9000 => {},
            0xA000 => {},
            0xB000 => {},
            0xC000 => {},
            0xD000 => {},
            0xE000 => {
                match opcode & 0x000F {
                    0x000E => {},
                    0x0001 => {},
                    _ => panic!("Unknown opcode [0xE000]: {:#X}", opcode),
                }
            },
            0xF000 => {
                match opcode & 0x00FF {
                    0x0007 => {},
                    0x000A => {},
                    0x0015 => {},
                    0x0018 => {},
                    0x001E => {},
                    0x0029 => {},
                    0x0033 => {},
                    0x0055 => {},
                    0x0065 => {},
                    _ => panic!("Unknown opcode [0xF000]: {:#X}", opcode),
                }
            },
            _ => panic!("Unknown opcode: {:#X}", opcode),
        }
        // update timers
    }
}
