use std::collections::HashMap;


pub type HasmSize = u32;


#[derive(Debug, Clone)]
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
    EqFlag = 11
}

impl Register {
    pub fn from_usize(i: usize) -> Self {
        let x: Register = unsafe { std::mem::transmute(i as i8) };
        x
    }
}


#[derive(Debug, Clone)]
pub enum ArgType {
    Register(Register),
    IntLiteral(HasmSize),
    Deref(Box<ArgType>),
    Label(String),
    Section(SectionType),
    None
}

#[derive(Debug, Clone)]
pub enum SectionType {
    Text,
    Data,
    RoData,
    Rss,
    None
}
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub enum TokenType {
    // data moving
    Movb(ArgType, ArgType),
    Movw(ArgType, ArgType),
    Movdw(ArgType, ArgType),

    // Math
    Add(ArgType, ArgType),
    Sub(ArgType, ArgType),
    
    // comparison
    Eq(ArgType, ArgType),
    Neq(ArgType, ArgType),
    Lt(ArgType, ArgType),
    Gt(ArgType, ArgType),
    Le(ArgType, ArgType),
    Ge(ArgType, ArgType),

    // jumps
    Je(ArgType),
    Jne(ArgType),
    Jmp(ArgType),

    // special
    Section(ArgType),
    Syscall,
    Label(String, usize),
}
#[derive(Debug, Clone)]
pub struct Token {
    pub loc: Loc,
    pub typ: TokenType
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Program {
    pub tokens: Vec<Token>,
    pub labels: HashMap<String, HasmSize>
}

impl Default for Program {
    fn default() -> Self {
        Self { tokens: Default::default(), labels: Default::default() }
    }
}