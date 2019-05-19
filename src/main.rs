use chip8::Chip8;

mod chip8;
mod keyboard;
mod display;
mod cpu;
mod ram;

fn main() {
    // setup graphics & input

    let chip8 = Chip8::new();
    // load rom

    // loop
    // emulate one cycle
    // if the draw flag is set, update screen
    // store key press state (pressed and released)
}
