use std::collections::HashMap;

use color_eyre::{Result, eyre};

use self::types::{Program, ArgType};

use super::get_prog_mem;


pub mod types;
mod alloc;
pub mod parser;


#[derive(Debug, Clone)]
pub struct HasmRunner {
    //         [r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, rsp, eq_flag, lt_flag, gt_flag]
    pub registers: [types::HasmSize; 14],
    pub allocator: alloc::Allocator,
    pub program: Program,
    pub ro_data_labels: HashMap<String, types::HasmSize>,
    pub data_labels: HashMap<String, types::HasmSize>
}

impl HasmRunner {
    pub fn new(mem_size: usize) -> Self {
        Self {
            registers: [0; 14],
            allocator: alloc::Allocator::new(mem_size),
            program: Program::default(),
            ro_data_labels: HashMap::new(),
            data_labels: HashMap::new(),
        }
    }

    pub fn run_program(&mut self, mut code: Vec<u8>, _: String) -> Result<()>{

        if !(code[0] == b'.' && code[1] == b'H' && code[2] == b'A' && code[3] == b'S' && code[4] == b'M') {
            return Err(eyre::eyre!("Not a HASM executable"));
        }

        code = code[5..].to_vec();

        self.program = bincode::deserialize(&code)?;
        self.load_data_into_mem()?;
        self.run()?;
        Ok(())
    }


    fn run(&mut self) -> Result<()> {        
        let Some(ip) = self.program.text.labels.get("_start".into()) else {
            return Err(eyre::eyre!("Cannot find label '_start'"));
        };
        let mut ip = ip.clone() as usize;

        while ip < self.program.text.tokens.len() {
            let op = &self.program.text.tokens[ip].clone();

            match &op.typ {
                types::TokenType::Mov(size, arg1, arg2) => {
                    println!("Mov");
                    let val = self.read_arg(arg2, size)?;

                    self.write_ref(size.clone(), arg1.clone(), val)?;

                },
                types::TokenType::Add(size, arg1, arg2) => {
                    println!("Add");
                    let val1 = self.read_arg(arg1, size)?;
                    let val2 = self.read_arg(arg2, size)?;
                    

                    let val = val1 + val2;

                    self.write_ref(size.clone(), arg1.clone(), val)?;
                },
                types::TokenType::Sub(size, arg1, arg2) => {
                    println!("Sub");
                    let val1 = self.read_arg(arg1, size)?;
                    let val2 = self.read_arg(arg2, size)?;
                    

                    let val = val1 - val2;

                    self.write_ref(size.clone(), arg1.clone(), val)?;
                    
                },
                types::TokenType::Mul(size, arg1, arg2) => {
                    println!("Mul");
                    let val1 = self.read_arg(arg1, size)?;
                    let val2 = self.read_arg(arg2, size)?;
                    

                    let val = val1 * val2;

                    self.write_ref(size.clone(), arg1.clone(), val)?;
                    
                },
                types::TokenType::Div(size, arg1, arg2) => {
                    println!("Div");
                    let val1 = self.read_arg(arg1, size)?;
                    let val2 = self.read_arg(arg2, size)?;
                    

                    let val = val1 / val2;

                    self.write_ref(size.clone(), arg1.clone(), val)?;
                    
                },
                types::TokenType::Cmp(size, arg1, arg2) => {
                    println!("Cmp");
                    let val1 = self.read_arg(arg1, size)?;
                    let val2 = self.read_arg(arg2, size)?;

                    
                    self.registers[types::Register::GtFlag as usize] = (val1 >  val2) as types::HasmSize;
                    self.registers[types::Register::LtFlag as usize] = (val1 <  val2) as types::HasmSize;
                    self.registers[types::Register::EqFlag as usize] = (val1 == val2) as types::HasmSize;

                }
                types::TokenType::Je(arg1) => {
                    println!("Je");

                    let addr = self.read_arg(arg1, &types::TokenSize::DoubleWord)?;
                    if self.registers[types::Register::EqFlag as usize] == 1 {
                        ip = addr as usize;
                    }

                },
                types::TokenType::Jne(arg1) => {
                    println!("Jne");
                    let addr = self.read_arg(arg1, &types::TokenSize::DoubleWord)?;
                    if self.registers[types::Register::EqFlag as usize] == 0 {
                        ip = addr as usize;
                    }
                },
                types::TokenType::Jgt(arg1) => {
                    println!("Jgt");
                    let addr = self.read_arg(arg1, &types::TokenSize::DoubleWord)?;
                    if self.registers[types::Register::GtFlag as usize] == 1 {
                        ip = addr as usize;
                    }
                },
                types::TokenType::Jlt(arg1) => {
                    println!("Jlt");
                    let addr = self.read_arg(arg1, &types::TokenSize::DoubleWord)?;
                    if self.registers[types::Register::LtFlag as usize] == 1 {
                        ip = addr as usize;
                    }
                },
                types::TokenType::Jge(arg1) => {
                    println!("Jge");
                    let addr = self.read_arg(arg1, &types::TokenSize::DoubleWord)?;
                    if self.registers[types::Register::EqFlag as usize] == 1 ||
                        self.registers[types::Register::GtFlag as usize] == 1 {
                        ip = addr as usize;
                    }
                },
                types::TokenType::Jle(arg1) => {
                    println!("Jle");
                    let addr = self.read_arg(arg1, &types::TokenSize::DoubleWord)?;
                    if self.registers[types::Register::EqFlag as usize] == 1 ||
                        self.registers[types::Register::LtFlag as usize] == 1 {
                        ip = addr as usize;
                    }
                },
                types::TokenType::Jmp(arg1) => {
                    println!("Jmp");
                    let addr = self.read_arg(arg1, &types::TokenSize::DoubleWord)?;
                    ip = addr as usize;
                },
                types::TokenType::Syscall => {
                    println!("Syscall");

                },
                types::TokenType::Label(_, _) => {
                    println!("Label");

                },
            }


            ip += 1;
        }

        
        Ok(())
    }

    fn read_arg(&mut self, arg: &ArgType, size: &types::TokenSize) -> Result<types::HasmSize> {
        match arg {
            types::ArgType::Register(a) => {
                Ok(self.registers[a.clone() as usize])
            },
            types::ArgType::IntLiteral(a) => Ok(a.clone()),
            types::ArgType::Deref(a) => {
                self.clone().read_ref(size.clone(), a.as_ref().clone())
            },
            types::ArgType::Label(l) => {
                if let Some(lb) = self.program.text.labels.get(l) {
                    Ok(*lb)
                } else if let Some(_) = self.program.rodata.labels.get(l) {
                    Ok(*self.ro_data_labels.get(l).unwrap())
                } else if let Some(_) = self.program.data.labels.get(l) {
                    Ok(*self.data_labels.get(l).unwrap())
                } else {
                    unreachable!()
                }
            }
            types::ArgType::Section(_) => return Err(eyre::eyre!("Cannot read value of type {arg:?}")),
            types::ArgType::None => unreachable!(),
        }
    }

    fn read_ref(&mut self, bytes: types::TokenSize, at: types::ArgType) -> Result<types::HasmSize>  {
        match at {
            types::ArgType::Register(r) => {
                let addr = self.registers[r.clone() as usize].clone();
                Ok(self.mem_read(addr, bytes))
            }
            types::ArgType::IntLiteral(r) => {
                Ok(self.mem_read(r, bytes))
            },
            types::ArgType::Deref(a) => self.read_ref(bytes, a.as_ref().clone()),
            types::ArgType::Label(_) => todo!(),
            types::ArgType::Section(_) => return Err(eyre::eyre!("Cannot read value of type {at:?}")),
            types::ArgType::None => unreachable!(),
        }
    }

    fn write_ref(&mut self, bytes: types::TokenSize, target: types::ArgType, val: types::HasmSize) -> Result<()>  {
        match target {
            types::ArgType::Register(r) => {
                let addr = self.registers[r.clone() as usize ].clone();
                self.mem_write(addr, val, bytes);
                Ok(())
            }
            types::ArgType::IntLiteral(r) => {
                self.mem_write(r, val, bytes);
                Ok(())
            },
            types::ArgType::Deref(a) => self.write_ref(bytes, a.as_ref().clone(), val),
            types::ArgType::Label(_) => todo!(),
            types::ArgType::Section(_) => return Err(eyre::eyre!("Cannot read value of type {target:?}")),
            types::ArgType::None => unreachable!(),
        }
    }

    fn mem_write(&mut self, addr: types::HasmSize, mut data: types::HasmSize, size: types::TokenSize) {
        unsafe {
            match size {
                types::TokenSize::Byte => {
                    get_prog_mem()[addr as usize] = data as u8;
                },
                types::TokenSize::Word => {
                    get_prog_mem()[addr as usize] = data as u8;
                    data >>= 8;
                    get_prog_mem()[addr as usize + 1] = data as u8;
                },
                types::TokenSize::DoubleWord => {
                    get_prog_mem()[addr as usize] = data as u8;
                    data >>= 8;
                    get_prog_mem()[addr as usize + 1] = data as u8;
                    data >>= 8;
                    get_prog_mem()[addr as usize + 2] = data as u8;
                    data >>= 8;
                    get_prog_mem()[addr as usize + 3] = data as u8;
                },
            }
        }
    }

    fn mem_read(&mut self, addr: types::HasmSize, size: types::TokenSize) -> types::HasmSize {
        unsafe {
            let mut val: types::HasmSize = 0;
            match size {
                types::TokenSize::Byte => {
                    val = get_prog_mem()[addr as usize] as types::HasmSize;
                },
                types::TokenSize::Word => {
                    val |= get_prog_mem()[addr as usize + 1] as types::HasmSize;
                    val <<= 8;
                    val |= get_prog_mem()[addr as usize] as types::HasmSize;
                },
                types::TokenSize::DoubleWord => {
                    val |= get_prog_mem()[addr as usize + 3] as types::HasmSize;
                    val <<= 8;
                    val |= get_prog_mem()[addr as usize + 2] as types::HasmSize;
                    val <<= 8;
                    val |= get_prog_mem()[addr as usize + 1] as types::HasmSize;
                    val <<= 8;
                    val |= get_prog_mem()[addr as usize] as types::HasmSize;
                },
            }
            val
        }
    }

    fn load_data_into_mem(&mut self) -> Result<()> {
        for (name, label) in self.program.rodata.labels.clone() {
            let addr = self.allocator.alloc(label.data.len()*(label.size.clone() as usize));

            for byte in label.data {
                self.mem_write(addr as types::HasmSize, byte, label.size.clone());
            }
            self.ro_data_labels.insert(name, addr as types::HasmSize);
        }
        for (name, label) in self.program.data.labels.clone() {
            let addr = self.allocator.alloc(label.data.len()*(label.size.clone() as usize));

            for byte in label.data {
                self.mem_write(addr as types::HasmSize, byte, label.size.clone());
            }
            self.data_labels.insert(name, addr as types::HasmSize);
        }

        Ok(())
    }

}