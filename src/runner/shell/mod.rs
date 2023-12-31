pub mod builtin;
mod util;

use crate::{keyboard::KeyPress, screen::Screen};
use std::{fmt::Write, sync::Mutex, path::PathBuf};
use color_eyre::Result;
use lazy_static::lazy_static;
use shlex;
use super::hasm::HasmRunner;
use super::fs::HgPath;

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

    pub fn step(&mut self, screen: &mut Screen, hasm: &mut HasmRunner) -> Result<()>{
        if self.reading_input {
            return Ok(());
        }
        let argv: Vec<String> = shlex::split(self.input_buf.trim()).unwrap();


        let (is_builtin, ret_code) = if !self.input_buf.trim().is_empty() {
            
            builtin::run_command(screen, self, argv.clone())?
        } else {(false, 0)};
        

        if !is_builtin && !argv.is_empty() {
            let bins = std::fs::read_dir(PathBuf::from("/bin").get_host_path())?;

            for f in bins {
                let f = f?;
                if f.file_name().to_string_lossy().to_string() == argv[0] {
                    if let Err(e) = self.run_app(argv[0].clone(), std::fs::read(f.path())?, hasm, screen) { 
                        writeln!(screen, "Error running program:\n{}", e.to_string())?;
                    }
                }
            }
        }

        self.input_buf.clear();
        write!(screen, "[{}] >>", ret_code)?;
        self.reading_input = true;
        Ok(())
    }


    fn run_app(&mut self, file_name: String, data: Vec<u8>, hasm: &mut HasmRunner, s: &mut Screen) -> Result<()> {
        let ret = hasm.run_program(data, file_name);

        if let Err(e) = ret {
            writeln!(s, "{}", e.to_string())?;
        }


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