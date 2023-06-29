pub mod builtin;
mod util;

use crate::{keyboard::KeyPress, screen::Screen};
use std::{fmt::Write, sync::Mutex};
use color_eyre::Result;
use lazy_static::lazy_static;
use shlex;



lazy_static!{
    static ref RET_CODE: Mutex<usize> = Mutex::from(0);
}



#[derive(Debug, Clone)]
pub struct Shell {
    // emulator: Emulator,
    pub reading_input: bool,
    pub input_buf: String,
    pub should_exit: bool
}

impl Shell {
    pub fn new() -> Self {
        Self {
            // emulator: e,
            reading_input: false,
            input_buf: String::new(),
            should_exit: false
        }
    }

    pub fn start(&mut self) {

    }

    pub fn step(&mut self, screen: &mut Screen) -> Result<()>{
        if self.reading_input {
            return Ok(());
        }
        if !self.input_buf.trim().is_empty() {
            
            let argv: Vec<String> = shlex::split(self.input_buf.trim()).unwrap();
            let (_is_builtin, _ret) = builtin::run_command(screen, argv)?;
        }
        

        self.input_buf.clear();
        write!(screen, "[shell] >>")?;
        self.reading_input = true;
        Ok(())
    }
    

    pub fn kb_event(&mut self, kp: KeyPress) {
        match kp.code {
            crate::keyboard::KeyCode::Char(chr) => {
                self.input_buf.push(chr);
            }
            crate::keyboard::KeyCode::Backspace => {
                self.input_buf.pop();
            }
            crate::keyboard::KeyCode::Enter => {
                self.reading_input = false;
            },
            _ => ()
        }
    }
}