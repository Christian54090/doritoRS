pub struct Ram {
    pub memory: [u16; 4096],
    stack: [u8; 16],
    sp: u8,
    v: [u8; 16],
}

impl Ram {
    pub fn new() -> Self {
        Ram {
            memory: [0; 4096],
            stack: [0; 16],
            sp: 0,
            v: [0; 16],
        }
    }
}
