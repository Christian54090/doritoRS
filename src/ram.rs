pub struct Ram {
    memory: [u8; 4096],
    stack: [u8; 16],
    sp: u8,
}

impl Ram {
    pub fn new() -> Self {
        Ram {
            memory: [0; 4096],
            stack: [0; 16],
            sp: 0,
        }
    }
}