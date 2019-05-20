pub struct Keyboard {
    pub keys: [u16; 16],
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard { keys: [0; 16] }
    }

    pub fn is_key_pressed(&self, code: u16) -> bool {
        self.keys[code as usize] != 0
    }

    pub fn get_key(mut self) -> bool {
        let mut key_pressed = false;

        for i in 0..16 {
            if self.keys[i] != 0 {
                key_pressed = true
            }
        }
        key_pressed
    }
}
