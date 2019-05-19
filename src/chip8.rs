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
        let mut ram = Ram::new();

        //for i in 0..80 {
        //    ram.memory[i] = font_set[i];
        //}

        Chip8 {
            ram,
            cpu: Cpu::new(),
            display: Display::new(),
            keyboard: Keyboard::new(),
        }
    }

    pub fn run_cpu(self) {
        let ram = self.ram;
        let pc = self.cpu.pc;

        self.cpu.emulate_cycle(
            ram.memory[pc as usize] | ram.memory[(pc + 1) as usize]
        );
    }
}