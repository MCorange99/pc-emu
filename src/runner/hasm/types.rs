
use std::{collections::HashMap};

pub type HasmSize = u32;


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Register {
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7,
    R8 = 8,
    R9 = 9,
    Rsp = 10,
    EqFlag = 11,
    LtFlag = 12,
    GtFlag = 13,
}


impl Register {
    pub fn from_usize(i: usize) -> Self {
        let x: Register = unsafe { std::mem::transmute(i as i8) };
        x
    }
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ArgType {
    Register(Register),
    IntLiteral(HasmSize),
    Deref(Box<ArgType>),
    Label(String),
    Section(SectionType),
    None
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SectionType {
    Text,
    Data,
    RoData,
    Rss,
    None
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(dead_code)]
//* Currently only implementing write, to see if it works
pub enum Syscalls {
    Read = 0,
    Write = 1,
    Open = 2,
    Close = 3,
    Stat = 4,
}

#[allow(dead_code)]

impl Syscalls {
    pub fn from_usize(i: usize) -> Self {
        let x: Syscalls = unsafe { std::mem::transmute(i as i8) };
        x
    }
}



// byte u8
// word: u16
// dword: u32
//
//

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TokenSize {
    Byte = 1,
    Word = 2,
    DoubleWord = 4
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TokenType {
    // data moving
    Mov(TokenSize, ArgType, ArgType),

    // Math
    Add(TokenSize, ArgType, ArgType),
    Sub(TokenSize, ArgType, ArgType),
    Mul(TokenSize, ArgType, ArgType),
    Div(TokenSize, ArgType, ArgType),
    
    // comparison
    Cmp(TokenSize, ArgType, ArgType),

    // jumps
    Je(ArgType),
    Jne(ArgType),
    Jlt(ArgType),
    Jgt(ArgType),
    Jle(ArgType),
    Jge(ArgType),
    Jmp(ArgType),

    // special
    Syscall,
    Label(String, usize),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Token {
    pub loc: Loc,
    pub typ: TokenType
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Loc(pub String, pub usize);

impl Loc {
    pub fn human(self) -> String {
        format!("{}:{}", self.0, self.1)
    }
}

impl Token {
    pub fn new(loc: Loc, typ: TokenType) -> Self {
        Self {
            loc,
            typ
        }
    }

    pub fn loc(&self) -> &Loc {
        &self.loc
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Program {
    pub text: ProgramText,
    pub data: ProgramData,
    pub rodata: ProgramData
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProgramData {
    pub labels: HashMap<String, ProgramDataLabel>
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProgramDataLabel {
    pub readonly: bool,
    pub data: Vec<HasmSize>,
    pub size: TokenSize
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProgramText {
    pub tokens: Vec<Token>,
    pub labels: HashMap<String, HasmSize>,
}


impl Default for Program {
    fn default() -> Self {
        Self {
            text: ProgramText { tokens: Default::default(), labels: Default::default() },
            data: ProgramData { labels: HashMap::new()  },
            rodata: ProgramData { labels: HashMap::new()  },
        }
    }
}