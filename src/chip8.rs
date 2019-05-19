use crate::keyboard::Keyboard;
use crate::display::Display;
use crate::cpu::Cpu;
use crate::ram::Ram;

pub struct Chip8 {
    keyboard: Keyboard,
    display: Display,
    cpu: Cpu,
    ram: Ram,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut ram = Ram::new();

        //for i in 0..80 {
        //    ram.memory[i] = font_set[i];
        //}

        Chip8 {
            keyboard: Keyboard::new(),
            display: Display::new(),
            cpu: Cpu::new(),
            ram,
        }
    }

    pub fn emulate_cycle(mut self) {
        let ram = self.ram;
        let pc = self.cpu.pc;

        self.cpu.execute_opcode(
            ram.memory[pc as usize] | ram.memory[(pc + 1) as usize],
            self.display,
            ram
        );
    }
}
