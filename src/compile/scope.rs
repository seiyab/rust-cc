use std::collections::HashMap;
use compile::assembly::Instruction;
use compile::assembly::Address;
use compile::assembly::Register;

pub trait Scope {
    fn lookup(&self, target: &String) -> Result<Vec<Instruction>, ()>;
    fn declare(&mut self, target: &String) -> Result<Vec<Instruction>, ()>;
    fn prologue(&self) -> Vec<Instruction>;
    fn epilogue(&self) -> Vec<Instruction>;
}

pub type PointerOffset = i64;

pub struct CurrentScope {
    variables: HashMap<String, PointerOffset>,
    next: PointerOffset,
}

impl CurrentScope {
    pub fn new() -> CurrentScope {
        CurrentScope {
            variables: HashMap::new(),
            next: 0,
        }
    }
}

impl Scope for CurrentScope{
    fn lookup(&self, target: &String) -> Result<Vec<Instruction>, ()> {
        self.variables
            .get(target)
            .ok_or(())
            .map(|&offset| {
                vec![
                    Instruction::Mov(Box::new(Register::Rax), Box::new(Register::Rbp)),
                    Instruction::Sub(Register::Rax, Box::new(offset)),
                    Instruction::Mov(Box::new(Register::Rax), Box::new(Address::new(Register::Rax))),
                    Instruction::Push(Box::new(Register::Rax))
                ]
            })
    }

    // RSPの指す値を代入
    fn declare(&mut self, target: &String) -> Result<Vec<Instruction>, ()> {
        self.next += 8;
        match self.variables.insert(target.clone(), self.next) {
            Some(_) => Err(()),
            None => {
                Ok(vec![
                    Instruction::Mov(Box::new(Register::Rax), Box::new(Register::Rbp)),
                    Instruction::Sub(Register::Rax, Box::new(self.next)),
                    Instruction::Pop(Register::Rdi),
                    Instruction::Mov(Box::new(Address::new(Register::Rax)), Box::new(Register::Rdi))
                ])
            },
        }
    }
    fn prologue(&self) -> Vec<Instruction> {
        vec![
            Instruction::Push(Box::new(Register::Rbp)),
            Instruction::Mov(Box::new(Register::Rbp), Box::new(Register::Rsp)),
            Instruction::Sub(Register::Rsp, Box::new(self.next))
        ]
    }
    fn epilogue(&self) -> Vec<Instruction> {
        vec![
            Instruction::Mov(Box::new(Register::Rsp), Box::new(Register::Rbp)),
            Instruction::Pop(Register::Rbp),
            Instruction::Ret,
        ]
    }
}