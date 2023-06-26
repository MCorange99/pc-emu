#[derive(Debug)]
pub struct KeyPress {
    key_code: usize,
    action: KeyAction,
    chr: char
}

impl KeyPress {
    pub fn new(key_code: usize, action: KeyAction) -> Self {
        Self {
            key_code,
            action,
            chr: char::from_u32(key_code as u32 + 59).unwrap()
        }
    }
}

#[derive(Debug)]
pub enum KeyAction {
    Press,
    Release,
    Repeat
}



