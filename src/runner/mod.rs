mod fs;
mod shell;
mod hasm;

use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard};
use std::cell::UnsafeCell;

use crate::screen::Screen;
use color_eyre::Result;

pub const MEM_SIZE: usize = 1024 * 1024;

pub mod machine_status_bits {
    pub const MS_SHOULD_EXIT: usize = 0b0000_0001;
}

pub static mut MACHINE_STATUS: Mutex<UnsafeCell<usize>> = Mutex::new(UnsafeCell::new(0));

pub static mut PROG_MEM: Mutex<UnsafeCell<[u8; 1024 * 1024]>> = Mutex::new(UnsafeCell::new([0; MEM_SIZE]));// 1mb

///  # Program memory
///  Make sure the unlock goes out of scope asap
/// 
///  ## Screen
///  120*40 = 48kb. 0x00 - 0x12c0
///
///
pub unsafe fn get_prog_mem<'a>() -> &'a mut [u8]{
    let prog_mem: MutexGuard<UnsafeCell<[u8; 1024 * 1024]>> = PROG_MEM.lock().unwrap();
    &mut *prog_mem.get()
}

#[derive(Debug, Clone)]
pub struct Runner {
    // fs: fs::Fs,
    pub shell: shell::Shell,
    pub hasm: hasm::HasmRunner
}

impl Runner {
    pub fn new() -> Self{
        if !PathBuf::from("./fs").exists() {
            std::fs::create_dir(PathBuf::from("./fs")).unwrap();
        }

        Self {
            // fs: fs::Fs::new(),
            shell: shell::Shell::new(),
            hasm: hasm::HasmRunner::new(MEM_SIZE)
        }
    }

    pub unsafe fn get_prog_mem<'a>(&mut self) -> &'a mut [u8]{
        let prog_mem: MutexGuard<UnsafeCell<[u8; 1024 * 1024]>> = PROG_MEM.lock().unwrap();
        &mut *prog_mem.get()
    }

    pub fn step(&mut self, screen: &mut Screen) -> Result<()>{
        self.shell.step(screen, &mut self.hasm)?;
        Ok(())
    }
}