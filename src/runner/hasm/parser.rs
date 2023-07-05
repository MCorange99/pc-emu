use std::collections::HashMap;

use color_eyre::Result;
use parse_int;

use super::types::{self, Program};
#[derive(Debug, Clone)]
pub struct Parser {
    code: String,
    program: Program,
    line: usize,
    file: String,
    section: Section,
    labels_to_find: HashMap<String, types::Loc>
}

#[derive(Debug, Clone, PartialEq)]
enum Section {
    Text,
    Data,
    RoData,
    None
}

impl Parser {
    pub fn new(code: String, file: String) -> Self {
        Self {
            code,
            program: Program::default(),
            line: 0,
            file,
            section: Section::None,
            labels_to_find: HashMap::new(),
        }
    }

    // fn trim_left<F>(&mut self, t: String, predicate: F) -> String where F: Fn(char) -> bool{
    //     let mut buf = String::new();

    //     for (i, c) in t.chars().enumerate() {
            
    //     }
    // }

    fn find_char<F>(&mut self, text: &str, mut col: usize, predicate: F) -> usize where F: Fn(char, char) -> bool {
        let mut last = '\0';
        while col < text.len() && !predicate(last, text.chars().nth(col).unwrap()) {
            last = text.chars().nth(col).unwrap();
            col += 1;
        }
    
        col
    }

    pub fn parse(&mut self) -> Result<&mut Self> {
        let mut ip = 0;
        for (_, mut line) in self.code.clone().lines().enumerate() {

            if line.trim().is_empty() {
                continue;
            }

            let start_col = self.find_char(&line, 0, |_, c| c != ' ' && c != '\t');
            line = &line[start_col..];
            let col = self.find_char(&line, 0, |_, c| c == ' ' || c == '\t' || c == ':');
            let instruction_name = &line[..col];

            
            line = &line[col..].trim();
            
            if instruction_name == "section" {
                let col = self.find_char(&line, 0, |_, c| c == ' ' || c == '\t' || c == ':');
                let sec_name = &line[..col];

                match sec_name {
                    ".text" => self.section = Section::Text,
                    ".data" => self.section = Section::Data,
                    ".rodata" => self.section = Section::RoData,
                    _ => {
                        return Err(color_eyre::eyre::eyre!("{}: Unknown section {sec_name:?}", self.loc().human()));
                    }
                }
                continue;
            }


            if &line.trim().chars().nth(0).unwrap() == &':' {
                if self.program.text.labels.contains_key(&instruction_name.to_string()) {
                    return Err(color_eyre::eyre::eyre!("{}: Label {:?} already exists", self.loc().human(), instruction_name));
                }

                if self.labels_to_find.contains_key(&instruction_name.to_string()){
                    self.labels_to_find.remove(&instruction_name.to_string());
                }

                
                if self.section == Section::Text {
                    self.program.text.labels.insert(instruction_name.to_string(), ip as u32);
                    self.program.text.tokens.push(types::Token::new(self.loc(), types::TokenType::Label(instruction_name.to_string(), ip)));
                    ip += 1;
                    continue;
                } else 
                if self.section == Section::RoData {
                    line = &line[1..].trim();
                } else {
                    todo!("other sections");
                }
            }


            if self.section == Section::Text {
                let mut arg1 = None;
                let mut arg2 = None;
    
                if !line.is_empty() {
                    let arg_pos = self.find_char(&line, 0, |_, c|c == ',');
                    arg1 = Some(self.parse_arg(&line[..arg_pos].trim())?);
                    line = &line[arg_pos..].trim();
    
                    line = &line[self.find_char(&line, 0, |_, c|c != ',' && c != ' ' && c != '\t')..];
                }
    
                
                if !line.is_empty() {
                    let arg_pos = self.find_char(&line, 0, |_, c|c == ',');
                    arg2 = Some(self.parse_arg(&line[..arg_pos].trim())?);
                    // line = &line[arg_pos+1..].trim();
                }
                
                
                let instruction= self.get_instruction_type(&instruction_name, arg1, arg2)?;
                self.program.text.tokens.push(types::Token::new(self.loc(), instruction));
                ip += 1;
            } else
            if self.section == Section::RoData {
                let type_end = self.find_char(&line, 0, |_, c|c == ' ' || c == '\t');
                let typ = &line[..type_end];

                line = &line[type_end..].trim();

                let typ = match typ {
                    "db" => types::TokenSize::Byte,
                    "dw" => types::TokenSize::Word,
                    "dd" => types::TokenSize::DoubleWord,
                    _ => return Err(color_eyre::eyre::eyre!("{}: Unknown type {:?}", self.loc().human(), typ))
                };

                let mut data: Vec<types::HasmSize> = Vec::new();

                let mut buf = String::new();
                let mut is_str = false;
                for c in line.chars() {
                    if c == '"' {
                        if is_str {
                            if !buf.ends_with('\\') {
                                let chars = buf.as_bytes();
                                for cc in chars {
                                    data.push(*cc as u32);
                                }
                                buf.clear();
                                is_str = false;
                            } else {
                                buf.pop();
                                buf.push(c);
                            }
                        } else {
                            is_str = true;
                        }
                    }

                    if c == ' ' {
                        if is_str {
                            buf.push(c);
                        }
                    }

                    if c == ',' {
                        if is_str {
                            buf.push(c);
                        } else {
                            let val = parse_int::parse::<types::HasmSize>(&buf)?;
                            data.push(val);
                            buf.clear();
                        }
                    }
                    
                    buf.push(c);
                }
                
                if !buf.is_empty() {
                    let val = parse_int::parse::<types::HasmSize>(&buf);
                    if let Ok(val) = val {
                        data.push(val);
                    }
                    buf.clear();
                }

                let label = types::ProgramDataLabel{
                    readonly: true,
                    data,
                    size: typ,
                };

                self.program.rodata.labels.insert(instruction_name.to_string(), label);

            }
        }

        let mut buff = String::new();

        for (ln, ll) in self.labels_to_find.iter() {
            buff.push_str(format!("{}: Unknown label {:?}\n", ll.clone().human(), ln).as_str())
        }

        if !self.labels_to_find.is_empty(){
            return Err(color_eyre::eyre::eyre!("{}", buff));
        }

        Ok(self)
    }

    fn parse_arg(&mut self, s: &str) -> Result<types::ArgType> {
        if &s[0..1] == "[" {
            return Ok(types::ArgType::Deref(Box::new(self.parse_arg(&s[1..s.len()-1])?)));
        }

        match s {
            "r0" => return Ok(types::ArgType::Register(types::Register::R0)),
            "r1" => return Ok(types::ArgType::Register(types::Register::R1)),
            "r2" => return Ok(types::ArgType::Register(types::Register::R2)),
            "r3" => return Ok(types::ArgType::Register(types::Register::R3)),
            "r4" => return Ok(types::ArgType::Register(types::Register::R4)),
            "r5" => return Ok(types::ArgType::Register(types::Register::R5)),
            "r6" => return Ok(types::ArgType::Register(types::Register::R6)),
            "r7" => return Ok(types::ArgType::Register(types::Register::R7)),
            "r8" => return Ok(types::ArgType::Register(types::Register::R8)),
            "r9" => return Ok(types::ArgType::Register(types::Register::R9)),
            "rsp" => return Ok(types::ArgType::Register(types::Register::Rsp)),
            _ => ()
        };

        if self.program.text.labels.contains_key(&s.to_string()) {
            return Ok(types::ArgType::Label(s.to_string()));
        }

        let Ok(num) = parse_int::parse::<types::HasmSize>(s) else {
            self.labels_to_find.insert(s.to_string(), self.loc());
            return Ok(types::ArgType::Label(s.to_string()));
        };
        
        Ok(types::ArgType::IntLiteral(num))

    }

    fn loc(&self) -> types::Loc {
        types::Loc(self.file.clone(), self.line)
    }

    fn get_instruction_type(&mut self, s: &str, arg1: Option<types::ArgType>, arg2: Option<types::ArgType>) -> Result<types::TokenType> {
        match s.to_lowercase().as_str() {
            "syscall" => Ok(types::TokenType::Syscall                                                             ),
            "jmp"     => Ok(types::TokenType::Jmp     (arg1.unwrap()                                             )),
            "je"      => Ok(types::TokenType::Je      (arg1.unwrap()                                             )),
            "jne"     => Ok(types::TokenType::Jne     (arg1.unwrap()                                             )),
            "jgt"     => Ok(types::TokenType::Jgt     (arg1.unwrap()                                             )),
            "jlt"     => Ok(types::TokenType::Jlt     (arg1.unwrap()                                             )),
            "jge"     => Ok(types::TokenType::Jge     (arg1.unwrap()                                             )),
            "jle"     => Ok(types::TokenType::Jle     (arg1.unwrap()                                             )),

            "movb"    => Ok(types::TokenType::Mov     (types::TokenSize::Byte,       arg1.unwrap(), arg2.unwrap())),
            "movw"    => Ok(types::TokenType::Mov     (types::TokenSize::Word,       arg1.unwrap(), arg2.unwrap())),
            "movdw"   => Ok(types::TokenType::Mov     (types::TokenSize::DoubleWord, arg1.unwrap(), arg2.unwrap())),

            "addb"    => Ok(types::TokenType::Add     (types::TokenSize::Byte,       arg1.unwrap(), arg2.unwrap())),
            "addw"    => Ok(types::TokenType::Add     (types::TokenSize::Word,       arg1.unwrap(), arg2.unwrap())),
            "adddw"   => Ok(types::TokenType::Add     (types::TokenSize::DoubleWord, arg1.unwrap(), arg2.unwrap())),

            "subb"    => Ok(types::TokenType::Sub     (types::TokenSize::Byte,       arg1.unwrap(), arg2.unwrap())),
            "subw"    => Ok(types::TokenType::Sub     (types::TokenSize::Word,       arg1.unwrap(), arg2.unwrap())),
            "subdw"   => Ok(types::TokenType::Sub     (types::TokenSize::DoubleWord, arg1.unwrap(), arg2.unwrap())),

            "divb"    => Ok(types::TokenType::Mul     (types::TokenSize::Byte,       arg1.unwrap(), arg2.unwrap())),
            "divw"    => Ok(types::TokenType::Mul     (types::TokenSize::Word,       arg1.unwrap(), arg2.unwrap())),
            "divdw"   => Ok(types::TokenType::Mul     (types::TokenSize::DoubleWord, arg1.unwrap(), arg2.unwrap())),

            "mulb"    => Ok(types::TokenType::Div     (types::TokenSize::Byte,       arg1.unwrap(), arg2.unwrap())),
            "mulw"    => Ok(types::TokenType::Div     (types::TokenSize::Word,       arg1.unwrap(), arg2.unwrap())),
            "muldw"   => Ok(types::TokenType::Div     (types::TokenSize::DoubleWord, arg1.unwrap(), arg2.unwrap())),

            "cmpb"    => Ok(types::TokenType::Cmp     (types::TokenSize::Byte,       arg1.unwrap(), arg2.unwrap())),
            "cmpw"    => Ok(types::TokenType::Cmp     (types::TokenSize::Word,       arg1.unwrap(), arg2.unwrap())),
            "cmpdw"   => Ok(types::TokenType::Cmp     (types::TokenSize::DoubleWord, arg1.unwrap(), arg2.unwrap())),
            
            
            i => Err(color_eyre::eyre::eyre!("{}: Unknown instruction {i}", self.loc().human()))
        }
    }



    pub fn get_program(&mut self) -> types::Program {
        self.program.clone()
    }
}