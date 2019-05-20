use crate::ram::Ram;

pub struct Display {
    pub flipped_pixel: bool,
    draw_flag: bool,
    pub gfx: [u16; 64 * 32],
}

impl Display {
    pub fn new() -> Self {
        Display {
            flipped_pixel: false,
            draw_flag: false,
            gfx: [0; 64 * 32],
        }
    }

    pub fn clear(&mut self) {
        self.gfx = [0; 64 * 32];
    }

    pub fn draw(&mut self, byte: u16, x: usize, y: usize) -> bool {
        let mut flipped_px = false;
        let mut x_idx = x;
        let mut y_idx = y;
        let mut b = byte;

        for _ in 0..8 {
            x_idx %= 32;
            y_idx %= 64;
            let idx = y_idx * 32 + x_idx;
            let bit = (b & 0b1000_0000) >> 7;
            let prev_value = self.gfx[idx];
            self.gfx[idx] ^= bit;

            if prev_value == 1 && self.gfx[idx] == 0 {
                flipped_px = true;
            }

            x_idx += 1;
            b <<= 1;
        }
        flipped_px
    }
}
