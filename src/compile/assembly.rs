pub enum Instruction {
    Push(Box<dyn Readable>),
    Pop(Register),
    Add(Register, Box<dyn Readable>),
    Sub(Register, Box<dyn Readable>),
    Imul(Register, Box<dyn Readable>),
    Cqo,
    Idiv(Register),
    Cmp(Register, Box<dyn Readable>),
    Sete(Register),
    Setne(Register),
    Setl(Register),
    Setle(Register),
    Setg(Register),
    Setge(Register),
    Mov(Box<dyn Writable>, Box<dyn Readable>),
    Movzb(Register, Box<dyn Readable>),
    Ret,
}

impl Instruction {
    pub fn destination_code(&self) -> String {
        match &self {
            &Instruction::Push(readable) => format!("  push {}", readable.read_symbol()),
            &Instruction::Pop(register) => format!("  pop {}", register.symbol()),
            &Instruction::Add(acc, x) => format!("  add {}, {}", acc.symbol(), x.read_symbol()),
            &Instruction::Sub(acc, x) => format!("  sub {}, {}", acc.symbol(), x.read_symbol()),
            &Instruction::Imul(acc, x) => format!("  imul {}, {}", acc.symbol(), x.read_symbol()),
            &Instruction::Cqo => format!("  cqo"),
            &Instruction::Idiv(register) => format!("  idiv {}", register.symbol()),
            &Instruction::Cmp(register, x) => format!("  cmp {}, {}", register.symbol(), x.read_symbol()),
            &Instruction::Sete(register) => format!("  sete {}", register.symbol()),
            &Instruction::Setne(register) => format!("  setne {}", register.symbol()),
            &Instruction::Setl(register) => format!("  setl {}", register.symbol()),
            &Instruction::Setle(register) => format!("  setle {}", register.symbol()),
            &Instruction::Setg(register) => format!("  setg {}", register.symbol()),
            &Instruction::Setge(register) => format!("  setge {}", register.symbol()),
            &Instruction::Mov(register, x) => format!("  mov {}, {}", register.write_symbol(), x.read_symbol()),
            &Instruction::Movzb(register, x) => format!("  movzx {}, {}", register.symbol(), x.read_symbol()),
            &Instruction::Ret => format!("  ret"),
        }
    }
}

pub trait Readable{
    fn read_symbol(&self) -> String;
}

impl Readable for i64 {
    fn read_symbol(&self) -> String {
        self.to_string()
    }
}

pub trait Writable {
    fn write_symbol(&self) -> String;
}

//#[derive(Debug, Clone, Copy)]
//pub enum Readable {
//    Register(Register),
//    Integer(i64)
//}
//
//impl Readable {
//    fn symbol(&self) -> String {
//        match &self {
//            &Readable::Register(register) => register.symbol(),
//            &Readable::Integer(i) => i.to_string(),
//        }
//    }
//}

#[derive(Debug, Clone, Copy)]
pub enum Register {
    Rax,
    Rbp,
    Rdi,
    Rsp,
    Al,
}

impl Register {
    fn symbol(&self) -> String {
        match &self {
            &Register::Rax => String::from("rax"),
            &Register::Rbp => String::from("rbp"),
            &Register::Rdi => String::from("rdi"),
            &Register::Rsp => String::from("rsp"),
            &Register::Al => String::from("al"),
        }
    }
}

impl Readable for Register {
    fn read_symbol(&self) -> String {
        self.symbol()
    }
}

impl Writable for Register {
    fn write_symbol(&self) -> String {
        self.symbol()
    }
}

pub struct Address {
    register: Register
}

impl Address {
    pub fn new(register: Register) -> Address {
        Address{register}
    }
}

impl Readable for Address {
    fn read_symbol(&self) -> String {
        format!("[{}]", self.register.read_symbol())
    }
}

impl Writable for Address {
    fn write_symbol(&self) -> String {
        format!("[{}]", self.register.read_symbol())
    }
}
