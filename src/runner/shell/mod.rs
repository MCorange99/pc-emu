pub mod builtin;
mod util;

use crate::{keyboard::KeyPress, screen::Screen};
use std::{fmt::Write, sync::Mutex, path::PathBuf};
use color_eyre::Result;
use lazy_static::lazy_static;
use shlex;
use super::hasm::HasmRunner;


lazy_static!{
    static ref RET_CODE: Mutex<usize> = Mutex::from(0);
}



#[derive(Debug, Clone)]
pub struct Shell {
    // emulator: Emulator,
    pub reading_input: bool,
    pub input_buf: String,
    pub should_exit: bool,
    pub cwd: PathBuf,
    pub hasm_runner: HasmRunner,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            // emulator: e,
            reading_input: false,
            input_buf: String::new(),
            should_exit: false,
            cwd: PathBuf::from("/"),
            hasm_runner: HasmRunner::new(crate::runner::MEM_SIZE)
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
            let (_is_builtin, _ret) = builtin::run_command(screen, self, argv)?;
        }
        

        self.input_buf.clear();
        write!(screen, "[{}] >>", self.cwd.to_str().unwrap())?;
        self.reading_input = true;
        Ok(())
    }


    fn run_app() -> Result<()> {
        Ok(())
    }
    

    pub fn kb_event(&mut self, screen: &mut Screen, kp: KeyPress) {
        match kp.code {
            crate::keyboard::KeyCode::Char(chr) => {
                if self.input_buf.is_empty() {
                    screen.disable_deleting = false;
                }
                self.input_buf.push(chr);
            }
            crate::keyboard::KeyCode::Backspace => {
                if self.input_buf.is_empty() {
                    screen.disable_deleting = true;
                }
                self.input_buf.pop();
            }
            crate::keyboard::KeyCode::Enter => {
                if self.input_buf.is_empty() {
                    screen.disable_deleting = false;
                }
                self.reading_input = false;
            },
            _ => ()
        }
    }
}