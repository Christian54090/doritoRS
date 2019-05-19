pub struct Ram {
    pub memory: [u16; 4096],
    pub stack: Vec<u16>,
    pub sp: u8,
    pub v: [u16; 16],
}

impl Ram {
    pub fn new() -> Self {
        Ram {
            memory: [0; 4096],
            stack: Vec::<u16>::new(),
            sp: 0,
            v: [0; 16],
        }
    }
}
