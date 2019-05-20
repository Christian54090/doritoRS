use crate::keyboard::Keyboard;
use crate::display::Display;
use crate::ram::Ram;
use rand::prelude::*;

pub struct Cpu {
    delay_timer: u16,
    sound_timer: u16,
    registers: [u8; 16],
    pub pc: u16,
    i: u16,
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

    pub fn execute_opcode(mut self, opcode: u16, kb: Keyboard, mut dis: Display, mut ram: Ram) {
        let nnn = opcode & 0x0FFF;
        let nn = opcode & 0x00FF;
        let n = opcode & 0x000F;
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        let vx = ram.v[x];
        let vy = ram.v[y];

        match opcode & 0xF000 {
            0x0000 => {
                match opcode & 0x000F {
                    0x0000 => {
                        dis.clear();
                        self.pc += 2;
                    },
                    0x000E => self.pc = ram.stack.pop().unwrap(),
                    _ => panic!("Unknown opcode [0x0000]: {:#X}", opcode),
                }
            },
            0x1000 => self.pc = nnn,
            0x2000 => {
                ram.stack.push(self.pc);
                ram.sp += 1;
                self.pc = nnn;
            },
            0x3000 => { // if vx == nn
                if vx == nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x4000 => { // if vx != nn
                if vx != nn {
                    self.pc += 4
                } else {
                    self.pc += 2
                }
            },
            0x5000 => { // if vx == vy
                if vx == vy {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x6000 => { // vx = nn
                ram.v[y] = nn;
                self.pc += 2;
            },
            0x7000 => { // vx += nn
                ram.v[x] = vx.wrapping_add(nn);
                self.pc += 2;
            },
            0x8000 => {
                let f = 0xF as usize;

                match opcode & 0x000F {
                    0x0000 => ram.v[x] = vy,
                    0x0001 => ram.v[x] = vx | vy,
                    0x0002 => ram.v[x] = vx & vy,
                    0x0003 => ram.v[x] = vx ^ vy,
                    0x0004 => {
                        ram.v[x] += vy;
                        if vx + vy > 0xFF {
                            ram.v[f] = 1;
                        } else {
                            ram.v[f] = 0;
                        }
                    },
                    0x0005 => {
                        let res = (vx as i8) - (vy as i8);
                        ram.v[x] = res as u16;
                        if res < 0 {
                            ram.v[f] = 1;
                        } else {
                            ram.v[f] = 0;
                        }
                    },
                    0x0006 => {
                        ram.v[f] = vx & 0x1;
                        ram.v[x] = vx >> 1;
                    },
                    0x0007 => {
                        let res = (vy as i8) - (vx as i8);
                        ram.v[x] = res as u16;
                        if res < 0 {
                            ram.v[f] = 1;
                        } else {
                            ram.v[f] = 0;
                        }
                    },
                    0x000E => {
                        ram.v[f] = (vx & 0x80) >> 7;
                        ram.v[x] = vx << 1;
                    },
                    _ => panic!("Unknown opcode [0x8000]: {:#X}", opcode),
                }
                self.pc += 2
            },
            0x9000 => {
                if vx != vy {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0xA000 => {
                self.i = nnn;
                self.pc += 2;
            },
            0xB000 => self.pc = ram.v[0] + nnn,
            0xC000 => {
                let mut rand = thread_rng();
                ram.v[x] = nn & rand.gen_range(0, 256);
                self.pc += 2;
            },
            0xD000 => {
                let mut flipped_px = false;
                for y_idx in 0..n {
                    let byte = ram.memory[(self.i + y_idx) as usize];
                    if dis.draw(byte, vx as usize, (vy + y_idx) as usize) {
                        let mut flipped_px = true;
                    }
                }
                if flipped_px {
                    ram.v[0xF] = 1;
                } else {
                    ram.v[0xF] = 0;
                }
                self.pc += 2;
            },
            0xE000 => {
                match opcode & 0x000F {
                    0x000E => {
                        if kb.is_key_pressed(vx) {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    },
                    0x0001 => {
                        if !kb.is_key_pressed(vx) {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    },
                    _ => panic!("Unknown opcode [0xE000]: {:#X}", opcode),
                }
            },
            0xF000 => {
                match opcode & 0x00FF {
                    0x0007 => {
                        ram.v[x] = self.delay_timer;
                        self.pc += 2;
                    },
                    0x000A => {
                        let mut key = 0;
                        for i in 0..16 {
                            if kb.keys[i] != 0 {
                                key = i as u16;
                            }
                        }

                        if kb.get_key() {
                            ram.v[x] = key;
                            self.pc += 2;
                        }
                    },
                    0x0015 => {
                        self.delay_timer = vx;
                        self.pc += 2;
                    },
                    0x0018 => {
                        self.sound_timer = vx;
                        self.pc += 2;
                    },
                    0x001E => {
                        self.i += vx;
                        self.pc += 2;
                    },
                    0x0029 => {
                        self.i = vx * 5; // each sprite is 5 lines long
                        self.pc += 2;
                    },
                    0x0033 => {
                        let i = self.i as usize;
                        ram.memory[i] = vx / 100;
                        ram.memory[i + 1] = (vx / 10) % 10;
                        ram.memory[i + 2] = (vx % 100) % 10;
                        self.pc += 2;
                    },
                    0x0055 => {
                        for idx in 0..x + 1 {
                            ram.memory[(self.i + idx as u16) as usize] = ram.v[idx]
                        }
                        self.i += x as u16 + 1;
                        self.pc += 2;
                    },
                    0x0065 => {
                        for idx in 0..x + 1 {
                            ram.v[idx] = ram.memory[(self.i + idx as u16) as usize]
                        }
                        self.i += x as u16 + 1;
                        self.pc += 2;
                    },
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
