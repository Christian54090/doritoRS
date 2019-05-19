pub struct Keyboard {
    key: [u8; 16],
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard { key: [0; 16] }
    }
}