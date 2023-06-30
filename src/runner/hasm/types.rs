


#[derive(Debug, Clone)]
pub enum Register {
    r0 = 0,
    r1 = 1,
    r2 = 2,
    r3 = 3,
    r4 = 4,
    r5 = 5,
    r6 = 6,
    r7 = 7,
    r8 = 8,
    r9 = 9,
    rsp = 10,
    eq_flag = 11
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
    IntLiteral(usize),
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

//* Currently only implementing write, to see if it works
pub enum Syscalls {
    Read = 0,
    Write = 1,
    Open = 2,
    Close = 3,
    Stat = 4,
}

impl Syscalls {
    pub fn from_usize(i: usize) -> Self {
        let x: Syscalls = unsafe { std::mem::transmute(i as i8) };
        x
    }
}
#[derive(Debug, Clone)]
pub enum TokenType {
    // data moving
    Mov(ArgType, ArgType),

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

pub type Loc = (String, usize);

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