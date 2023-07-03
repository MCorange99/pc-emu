use std::collections::HashMap;

use color_eyre::Result;
use parse_int;

use super::types;
#[derive(Debug, Clone)]
pub struct Parser {
    code: String,
    tokens: Vec<types::Token>,
    line: usize,
    file: String,
    labels: HashMap<String, types::HasmSize>
}

impl Parser {
    pub fn new(code: String, file: String) -> Self {
        Self {
            code,
            tokens: vec![],
            line: 0,
            file,
            labels: HashMap::new()
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
            let instruction_name = line[..col].to_string();

            
            line = &line[col..].trim();
            
            
            if &line.trim() == &":" {
                self.labels.insert(instruction_name.clone(), ip as u32);
                self.tokens.push(types::Token::new(self.loc(), types::TokenType::Label(instruction_name, ip)));
                ip += 1;
                continue;
            }


            
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
            self.tokens.push(types::Token::new(self.loc(), instruction));
            ip += 1;
        }
        Ok(self)
    }

    fn parse_arg(&mut self, s: &str) -> Result<types::ArgType> {
        if &s[0..1] == "[" {
            return Ok(types::ArgType::Deref(Box::new(self.parse_arg(&s[1..s.len()-2])?)));
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

        if self.labels.contains_key(&s.to_string()) {
            return Ok(types::ArgType::Label(s.to_string()));
        }

        let Ok(num) = parse_int::parse::<types::HasmSize>(s) else {
            return Err(color_eyre::eyre::eyre!("{}: Unknown label {s:?}", self.loc().human()));
        };
        
        Ok(types::ArgType::IntLiteral(num))

    }

    fn loc(&self) -> types::Loc {
        types::Loc(self.file.clone(), self.line)
    }

    fn get_instruction_type(&mut self, s: &str, arg1: Option<types::ArgType>, arg2: Option<types::ArgType>) -> Result<types::TokenType> {
        match s.to_lowercase().as_str() {
            "syscall" => Ok(types::TokenType::Syscall                               ),
            "movb"     => Ok(types::TokenType::Movb     (arg1.unwrap(), arg2.unwrap())),
            "movw"     => Ok(types::TokenType::Movw     (arg1.unwrap(), arg2.unwrap())),
            "movdw"     => Ok(types::TokenType::Movdw     (arg1.unwrap(), arg2.unwrap())),
            "jmp"     => Ok(types::TokenType::Jmp     (arg1.unwrap()               )),
            "section" => Ok(types::TokenType::Section (arg1.unwrap()               )),
            "add"     => Ok(types::TokenType::Add     (arg1.unwrap(), arg2.unwrap())),
            "sub"     => Ok(types::TokenType::Sub     (arg1.unwrap(), arg2.unwrap())),
            "je"      => Ok(types::TokenType::Je      (arg1.unwrap()               )),
            "jne"     => Ok(types::TokenType::Jne     (arg1.unwrap()               )),
            "eq"      => Ok(types::TokenType::Eq      (arg1.unwrap(), arg2.unwrap())),
            "neq"     => Ok(types::TokenType::Neq     (arg1.unwrap(), arg2.unwrap())),
            "lt"      => Ok(types::TokenType::Lt      (arg1.unwrap(), arg2.unwrap())),
            "gt"      => Ok(types::TokenType::Gt      (arg1.unwrap(), arg2.unwrap())),
            "le"      => Ok(types::TokenType::Le      (arg1.unwrap(), arg2.unwrap())),
            "ge"      => Ok(types::TokenType::Ge      (arg1.unwrap(), arg2.unwrap())),
            
            
            i => Err(color_eyre::eyre::eyre!("{}: Unknown instruction {i}", self.loc().human()))
        }
    }



    pub fn get_program(&mut self) -> types::Program {
        types::Program{
            tokens: self.tokens.clone(),
            labels: self.labels.clone(),
        }
    }
}