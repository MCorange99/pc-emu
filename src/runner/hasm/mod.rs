use color_eyre::Result;


pub mod types;
mod alloc;
mod parser;


#[derive(Debug, Clone)]
pub struct HasmRunner {
    //         [r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, rsp, eq_flag]
    pub registers: [u64; 12],
    pub allocator: alloc::Allocator,
    pub program: Vec<types::Token>

}

impl HasmRunner {
    pub fn new(mem_size: usize) -> Self {
        Self {
            registers: [0; 12],
            allocator: alloc::Allocator::new(mem_size),
            program: Vec::new(),
        }
    }

    pub fn run_program(&mut self, code: String, file: String) -> Result<()>{
        let mut parser = parser::Parser::new(code, file);
        parser.parse()?;
        self.program = parser.get_program_tokes();




        Ok(())
    }

}