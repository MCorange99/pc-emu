use std::sync::{Mutex, MutexGuard};
use std::cell::UnsafeCell;


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


pub struct Runner {

}

impl Runner {
    pub fn new() -> Self{
        Self {

        }
    }

    pub unsafe fn get_prog_mem<'a>(&mut self) -> &'a mut [u8]{
        let prog_mem: MutexGuard<UnsafeCell<[u8; 1024 * 1024]>> = PROG_MEM.lock().unwrap();
        &mut *prog_mem.get()
    }


}