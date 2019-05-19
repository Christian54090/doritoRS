pub struct Display {
    gfx: [[u8; 64]; 32],
}

impl Display {
    pub fn new() -> Self {
        Display { gfx: [[0; 64]; 32] }
    }

    pub fn clear(mut self) {
        self.gfx = [[0; 64]; 32]
    }
}