#[derive(Clone)]
pub enum Line {
    Instruction(Instruction),
    Label(Label),
}

#[derive(Clone)]
pub enum Instruction {
    // stack
    Push(Readable),
    Pop(Register),

    // operator
    Add(Register, Readable),
    Sub(Register, Readable),
    Imul(Register, Readable),
    Cqo,
    Idiv(Register),
    Cmp(Register, Readable),
    Sete(Register),
    Setne(Register),
    Setl(Register),
    Setle(Register),
    Setg(Register),
    Setge(Register),

    // move
    Mov(Writable, Readable),
    Movzb(Register, Readable),

    // jump
    Je(Label),
    Jmp(Label),
    Call(String),
    Ret,
}

impl Instruction {
    pub fn destination_code(&self) -> String {
        match &self {
            &Instruction::Push(readable) => format!("push {}", readable.symbol()),
            &Instruction::Pop(register) => format!("pop {}", register.symbol()),
            &Instruction::Add(acc, x) => format!("add {}, {}", acc.symbol(), x.symbol()),
            &Instruction::Sub(acc, x) => format!("sub {}, {}", acc.symbol(), x.symbol()),
            &Instruction::Imul(acc, x) => format!("imul {}, {}", acc.symbol(), x.symbol()),
            &Instruction::Cqo => format!("cqo"),
            &Instruction::Idiv(register) => format!("idiv {}", register.symbol()),
            &Instruction::Cmp(register, x) => format!("cmp {}, {}", register.symbol(), x.symbol()),
            &Instruction::Sete(register) => format!("sete {}", register.symbol()),
            &Instruction::Setne(register) => format!("setne {}", register.symbol()),
            &Instruction::Setl(register) => format!("setl {}", register.symbol()),
            &Instruction::Setle(register) => format!("setle {}", register.symbol()),
            &Instruction::Setg(register) => format!("setg {}", register.symbol()),
            &Instruction::Setge(register) => format!("setge {}", register.symbol()),
            &Instruction::Mov(register, x) => format!("mov {}, {}", register.symbol(), x.symbol()),
            &Instruction::Movzb(register, x) => format!("movzx {}, {}", register.symbol(), x.symbol()),
            &Instruction::Je(label) => format!("je {}", label.name),
            &Instruction::Jmp(label) => format!("jmp {}", label.name),
            &Instruction::Call(name) => format!("call {}", name),
            &Instruction::Ret => format!("ret"),
        }
    }
}

#[derive(Clone)]
pub enum Readable {
    Literal(i64),
    Register(Register),
    Address(Address),
}

impl Readable {
    pub fn symbol(&self) -> String {
        match &self {
            Self::Literal(n) => n.to_string(),
            Self::Register(r) => r.symbol(),
            Self::Address(addr) => format!("[{}]", addr.register.symbol())
        }
    }
}

#[derive(Clone)]
pub enum Writable {
    Register(Register),
    Address(Address),
}

impl Writable {
    pub fn symbol(&self) -> String {
        match &self {
            Self::Register(r) => r.symbol(),
            Self::Address(addr) => format!("[{}]", addr.register.symbol()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Register {
    Rax,
    Rcx,
    Rbp,
    Rdi,
    Rdx,
    Rsi,
    Rsp,
    R8,
    R9,
    Al,
}

impl Register {
    fn symbol(&self) -> String {
        match &self {
            &Self::Rax => "rax".to_string(),
            &Self::Rcx => "rcx".to_string(),
            &Self::Rbp => "rbp".to_string(),
            &Self::Rdi => "rdi".to_string(),
            &Self::Rdx => "rdx".to_string(),
            &Self::Rsi => "rsi".to_string(),
            &Self::Rsp => "rsp".to_string(),
            &Self::R8 => "r8".to_string(),
            &Self::R9 => "r9".to_string(),
            &Self::Al => "al".to_string(),
        }
    }

    pub fn fn_args() -> Vec<Self> {
        vec![
            Self::Rdi,
            Self::Rsi,
            Self::Rdx,
            Self::Rcx,
            Self::R8,
            Self::R9,
        ]
    }
}

#[derive(Clone)]
pub struct Address {
    pub register: Register
}

impl Address {
    pub fn new(register: Register) -> Address {
        Address{register}
    }
}

#[derive(Clone)]
pub struct Label {
    pub name: String,
}
