mod fs;
mod shell;

use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard};
use std::cell::UnsafeCell;


pub mod machine_status_bits {
    pub const MS_SHOULD_EXIT: usize = 0b0000_0001;
}

pub static mut MACHINE_STATUS: Mutex<UnsafeCell<usize>> = Mutex::new(UnsafeCell::new(0));

pub static mut PROG_MEM: Mutex<UnsafeCell<[u8; 1024 * 1024]>> = Mutex::new(UnsafeCell::new([0; 1024 * 1024]));// 1mb

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
    pub shell: shell::Shell
    
}

impl Runner {
    pub fn new() -> Self{
        if !PathBuf::from("./fs").exists() {
            std::fs::create_dir(PathBuf::from("./fs")).unwrap();
        }

        Self {
            // fs: fs::Fs::new(),
            shell: shell::Shell::new(),
        }
    }

    pub unsafe fn get_prog_mem<'a>(&mut self) -> &'a mut [u8]{
        let prog_mem: MutexGuard<UnsafeCell<[u8; 1024 * 1024]>> = PROG_MEM.lock().unwrap();
        &mut *prog_mem.get()
    }


}