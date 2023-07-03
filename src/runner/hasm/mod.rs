use color_eyre::{Result, eyre};

use self::types::Program;

use super::get_prog_mem;


pub mod types;
mod alloc;
mod parser;


#[derive(Debug, Clone)]
pub struct HasmRunner {
    //         [r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, rsp, eq_flag]
    pub registers: [types::HasmSize; 12],
    pub allocator: alloc::Allocator,
    pub program: Program

}

impl HasmRunner {
    pub fn new(mem_size: usize) -> Self {
        Self {
            registers: [0; 12],
            allocator: alloc::Allocator::new(mem_size),
            program: Program::default(),
        }
    }

    pub fn run_program(&mut self, code: String, file: String) -> Result<()>{
        let mut parser = parser::Parser::new(code, file);
        parser.parse()?;
        self.program = parser.get_program();

        self.run()?;
        Ok(())
    }


    fn run(&mut self) -> Result<()> {

        
        let Some(ip) = self.program.labels.get("_start".into()) else {
            return Err(eyre::eyre!("Cannot find label '_start'"));
        };
        let mut ip = ip.clone() as usize;

        while ip < self.program.tokens.len() {
            let op = &self.program.tokens[ip];

            match &op.typ {
                types::TokenType::Movb(arg1, arg2) => {
                    let val: types::HasmSize = match arg2 {
                        types::ArgType::Register(a) => {
                            self.registers[a.clone() as usize]
                        },
                        types::ArgType::IntLiteral(a) => a.clone(),
                        types::ArgType::Deref(a) => {
                            self.clone().read_ref(1, a.as_ref().clone())?
                        },
                        types::ArgType::Label(_) => todo!(),
                        types::ArgType::Section(_) => return Err(eyre::eyre!("Cannot read value of type {arg2:?}")),
                        types::ArgType::None => unreachable!(),
                    };

                    self.write_ref(1, arg1.clone(), val)?;

                },
                types::TokenType::Movw(arg1, arg2) => {
                    let val: types::HasmSize = match arg2 {
                        types::ArgType::Register(a) => {
                            self.registers[a.clone() as usize]
                        },
                        types::ArgType::IntLiteral(a) => a.clone(),
                        types::ArgType::Deref(a) => {
                            self.clone().read_ref(2, a.as_ref().clone())?
                        },
                        types::ArgType::Label(_) => todo!(),
                        types::ArgType::Section(_) => return Err(eyre::eyre!("Cannot read value of type {arg2:?}")),
                        types::ArgType::None => unreachable!(),
                    };

                    self.write_ref(2, arg1.clone(), val)?;

                },
                types::TokenType::Movdw(arg1, arg2) => {
                    let val: types::HasmSize = match arg2 {
                        types::ArgType::Register(a) => {
                            self.registers[a.clone() as usize]
                        },
                        types::ArgType::IntLiteral(a) => a.clone(),
                        types::ArgType::Deref(a) => {
                            self.clone().read_ref(4, a.as_ref().clone())?
                        },
                        types::ArgType::Label(_) => todo!(),
                        types::ArgType::Section(_) => return Err(eyre::eyre!("Cannot read value of type {arg2:?}")),
                        types::ArgType::None => unreachable!(),
                    };

                    self.write_ref(4, arg1.clone(), val)?;

                },
                types::TokenType::Add(_, _) => {

                },
                types::TokenType::Sub(_, _) => {

                },
                types::TokenType::Eq(_, _) => {

                },
                types::TokenType::Neq(_, _) => {

                },
                types::TokenType::Lt(_, _) => {

                },
                types::TokenType::Gt(_, _) => {

                },
                types::TokenType::Le(_, _) => {

                },
                types::TokenType::Ge(_, _) => {

                },
                types::TokenType::Je(_) => {

                },
                types::TokenType::Jne(_) => {

                },
                types::TokenType::Jmp(_) => {

                },
                types::TokenType::Section(_) => {

                },
                types::TokenType::Syscall => {

                },
                types::TokenType::Label(_, _) => ()
            }


            ip += 1;
        }

        
        Ok(())
    }

    fn read_ref(&mut self, bytes: u8, at: types::ArgType) -> Result<types::HasmSize>  {
        match at {
            types::ArgType::Register(r) => {
                unsafe {
                    let mut ret: types::HasmSize = 0;
                    let addr = self.registers[r.clone() as usize].clone() as usize;
                    // 
                    ret = ret | get_prog_mem()[addr] as types::HasmSize;
                    ret = ret << 8;
                    if bytes >= 2 {
                        ret = ret | get_prog_mem()[addr + 1] as types::HasmSize;
                        ret = ret << 8;
                    }
                    if bytes >= 3 {
                        ret = ret | get_prog_mem()[addr + 2] as types::HasmSize;
                        ret = ret << 8;
                    }

                    if bytes >= 4 {
                        ret = ret | get_prog_mem()[addr + 3] as types::HasmSize;
                    }

                    Ok(ret)
                }
            }
            types::ArgType::IntLiteral(r) => {
                unsafe {
                    let mut ret: types::HasmSize = 0;

                    ret = ret | get_prog_mem()[r.clone() as usize ] as types::HasmSize;
                    ret <<= 8;
                    if bytes >= 2 {
                        ret = ret | get_prog_mem()[r.clone() as usize + 1] as types::HasmSize;
                        ret <<= 8;
                    }
                    if bytes >= 3 {
                        ret = ret | get_prog_mem()[r.clone() as usize + 2] as types::HasmSize;
                        ret <<= 8;
                    }

                    if bytes >= 4 {
                        ret = ret | get_prog_mem()[r.clone() as usize + 3] as types::HasmSize;
                    }

                    Ok(ret)
                }
            },
            types::ArgType::Deref(a) => self.read_ref(bytes, a.as_ref().clone()),
            types::ArgType::Label(_) => todo!(),
            types::ArgType::Section(_) => return Err(eyre::eyre!("Cannot read value of type {at:?}")),
            types::ArgType::None => unreachable!(),
        }
    }

    fn write_ref(&mut self, bytes: u8, target: types::ArgType, mut val: types::HasmSize) -> Result<()>  {
        match target {
            types::ArgType::Register(r) => {
                unsafe {
                    let addr = self.registers[r.clone() as usize ].clone() as usize;

                    if bytes == 1 {
                        get_prog_mem()[addr] = val as u8;
                    } else
                    if bytes == 2 {
                        get_prog_mem()[addr] = val as u8 & 0xff;
                        val >>= 8;
                        get_prog_mem()[addr + 1] = val as u8 & 0xff;
                    } else
                    if bytes == 3 {
                        get_prog_mem()[addr] = val as u8 & 0xff;
                        val >>= 8;
                        get_prog_mem()[addr + 1] = val as u8 & 0xff;
                        val >>= 8;
                        get_prog_mem()[addr + 2] = val as u8 & 0xff;
                    } else
                    if bytes == 4 {
                        get_prog_mem()[addr] = val as u8 & 0xff;
                        val >>= 8;
                        get_prog_mem()[addr + 1] = val as u8 & 0xff;
                        val >>= 8;
                        get_prog_mem()[addr + 2] = val as u8 & 0xff;
                        val >>= 8;
                        get_prog_mem()[addr + 3] = val as u8 & 0xff;
                    }

                    Ok(())
                }
            }
            types::ArgType::IntLiteral(r) => {
                unsafe {
                    if bytes == 1 {
                        get_prog_mem()[r.clone() as usize] = val as u8;
                    } else
                    if bytes == 2 {
                        get_prog_mem()[r.clone() as usize] = val as u8 & 0xff;
                        val >>= 8;
                        get_prog_mem()[r.clone() as usize + 1] = val as u8 & 0xff;
                    } else
                    if bytes == 3 {
                        get_prog_mem()[r.clone() as usize] = val as u8 & 0xff;
                        val >>= 8;
                        get_prog_mem()[r.clone() as usize + 1] = val as u8 & 0xff;
                        val >>= 8;
                        get_prog_mem()[r.clone() as usize + 2] = val as u8 & 0xff;
                    } else
                    if bytes == 4 {
                        get_prog_mem()[r.clone() as usize] = val as u8 & 0xff;
                        val >>= 8;
                        get_prog_mem()[r.clone() as usize + 1] = val as u8 & 0xff;
                        val >>= 8;
                        get_prog_mem()[r.clone() as usize + 2] = val as u8 & 0xff;
                        val >>= 8;
                        get_prog_mem()[r.clone() as usize + 3] = val as u8 & 0xff;
                    }
                    Ok(())
                }
            },
            types::ArgType::Deref(a) => self.write_ref(bytes, a.as_ref().clone(), val),
            types::ArgType::Label(_) => todo!(),
            types::ArgType::Section(_) => return Err(eyre::eyre!("Cannot read value of type {target:?}")),
            types::ArgType::None => unreachable!(),
        }
    }

}