
//0x00 - 0x12c0: screen mem

use color_eyre::Result;

const PRIV_MEM: usize = 0x12c0 + 1024;

#[derive(Debug, Clone)]
pub struct Allocator {
    allocations: Vec<(usize, usize)>,
    mem_size: usize,
}

impl Allocator {
    pub fn new(mem_size: usize) -> Self {
        Self {
            allocations: Vec::new(),
            mem_size
        }
    }

    pub fn alloc(&mut self, size: usize) -> usize {
        if self.allocations.is_empty() {

            if size > self.mem_size - PRIV_MEM {
                panic!("Out of memory");
            }


            self.allocations.push((PRIV_MEM, size));
            PRIV_MEM
        } else {
            let (prev_addr, prev_size) = self.allocations.last().unwrap().clone();

            if prev_addr + prev_size + size > self.mem_size - PRIV_MEM {
                panic!("Out of memory");
            }
            self.allocations.push((prev_addr + prev_size, prev_addr + prev_size + size));

            prev_addr + prev_size
        }
    }

    pub fn dealloc(&mut self, addr: usize) -> Result<()> {
        for (i, al) in self.allocations.clone().iter().enumerate() {
            if addr == al.0 {
                self.allocations.remove(i);
                return Ok(());
            }
        }
        Err(color_eyre::eyre::eyre!("Unable to deallocate memory"))
    }
}