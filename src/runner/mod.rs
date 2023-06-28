use std::sync::Mutex;




pub static mut PROG_MEM: Mutex<&[u8; 1024*1024]> = Mutex::new(&[0; 1024*1024]); // 1mb

struct Runner {

}

impl Runner {
    pub fn new() -> Self{

        

        Self {

        }
    }
}