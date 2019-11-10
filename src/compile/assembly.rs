pub enum Instruction {
    Push(Readable),
    Pop(Register),
    Add(Register, Readable),
    Sub(Register, Readable),
    Imul(Register, Readable),
    Cqo,
    Idiv(Register),
    Cmp(Register, Readable),
    Sete(Register),
    Setl(Register),
    Setle(Register),
    Setg(Register),
    Setge(Register),
    Mov(Register, i64),
    Movzb(Register, Readable),
    Ret,
}

impl Instruction {
    pub fn destination_code(&self) -> String {
        match &self {
            &Instruction::Push(readable) => format!("  push {}", readable.symbol()),
            &Instruction::Pop(register) => format!("  pop {}", register.symbol()),
            &Instruction::Add(acc, x) => format!("  add {}, {}", acc.symbol(), x.symbol()),
            &Instruction::Sub(acc, x) => format!("  sub {}, {}", acc.symbol(), x.symbol()),
            &Instruction::Imul(acc, x) => format!("  imul {}, {}", acc.symbol(), x.symbol()),
            &Instruction::Cqo => format!("  cqo"),
            &Instruction::Idiv(register) => format!("  idiv {}", register.symbol()),
            &Instruction::Cmp(register, x) => format!("  cmp {}, {}", register.symbol(), x.symbol()),
            &Instruction::Sete(register) => format!("  sete {}", register.symbol()),
            &Instruction::Setl(register) => format!("  setl {}", register.symbol()),
            &Instruction::Setle(register) => format!("  setle {}", register.symbol()),
            &Instruction::Setg(register) => format!("  setg {}", register.symbol()),
            &Instruction::Setge(register) => format!("  setge {}", register.symbol()),
            &Instruction::Mov(register, x) => format!("  mov {}, {}", register.symbol(), x),
            &Instruction::Movzb(register, x) => format!("  movzb {}, {}", register.symbol(), x.symbol()),
            &Instruction::Ret => format!("  ret"),
        }
    }
}

//#[derive(Debug, Clone, Copy)]
pub enum Readable {
    Register(Register),
    Integer(i64)
}

impl Readable {
    fn symbol(&self) -> String {
        match &self {
            &Readable::Register(register) => register.symbol(),
            &Readable::Integer(i) => i.to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Register {
    Rax,
    Rdi,
    Al,
}

impl Register {
    fn symbol(&self) -> String {
        match &self {
            &Register::Rax => String::from("rax"),
            &Register::Rdi => String::from("rdi"),
            &Register::Al => String::from("al"),
        }
    }
}