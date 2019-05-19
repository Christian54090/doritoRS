use crate::ram::Ram;
use crate::cpu::Cpu;
use crate::display::Display;
use crate::keyboard::Keyboard;

pub struct Chip8 {
    ram: Ram,
    cpu: Cpu,
    display: Display,
    keyboard: Keyboard,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            ram: Ram::new(),
            cpu: Cpu::new(),
            display: Display::new(),
            keyboard: Keyboard::new(),
        }
    }

    pub fn emulate_cycle() {
        // fetch opcode
        // decode opcode
        // execute opcode
        // update timers
    }
}