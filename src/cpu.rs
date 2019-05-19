use crate::display::Display;
use crate::ram::Ram;

pub struct Cpu {
    delay_timer: u8,
    sound_timer: u8,
    registers: [u8; 16],
    pub pc: u16,
    i: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            delay_timer: 0,
            sound_timer: 0,
            registers: [0; 16],
            pc: 0x200,
            i: 0x000,
        }
    }

    pub fn execute_opcode(mut self, opcode: u16, mut display: Display, mut ram: Ram) {
        match opcode & 0xF000 {
            0x0000 => {
                match opcode & 0x000F {
                    0x0000 => {
                        display.clear();
                        self.pc += 2;
                    },
                    0x000E => self.pc = ram.stack.pop().unwrap(),
                    _ => panic!("Unknown opcode [0x0000]: {:#X}", opcode),
                }
            },
            0x1000 => self.pc = opcode & 0x0FFF,
            0x2000 => {
                ram.stack[ram.sp as usize] = self.pc;
                ram.sp += 1;
                self.pc = opcode & 0x0FFF;
            },
            0x3000 => {
                let vx = ram.v[(opcode & 0x0F00) as usize];
                let nn = opcode & 0x00FF;
                if vx == nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x4000 => {
                let vx = ram.v[(opcode & 0x0F00) as usize];
                let nn = opcode & 0x00FF;
                if vx != nn {
                    self.pc += 4
                } else {
                    self.pc += 2
                }
            },
            0x5000 => {
                let vx = ram.v[(opcode & 0x0F00) as usize];
                let vy = ram.v[(opcode & 0x00F0) as usize];
                if vx == vy {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x6000 => {
                ram.v[(opcode & 0x00F0) as usize] = (opcode & 0x00FF);
                self.pc += 2;
            },
            0x7000 => {
                let x = (opcode & 0x0F00) as usize;
                ram.v[x] = ram.v[x].wrapping_add(opcode & 0x00FF);
                self.pc += 2;
            },
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
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                print!("BEEP!\n");
            }
            self.sound_timer -= 1;
        }
    }
}
