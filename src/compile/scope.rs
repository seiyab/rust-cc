use std::collections::HashMap;

use sourcecode::Code;
use sourcecode::Span;

use super::assembly::Line;
use super::assembly::Instruction;
use super::assembly::Address;
use super::assembly::Register;
use super::assembly::Writable;
use super::assembly::Readable;

pub type PointerOffset = i64;

pub struct Scope {
    variables: HashMap<String, PointerOffset>,
    next: PointerOffset,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            next: 0,
        }
    }
}

impl Scope {
    pub fn lookup(&self, target: &Code<String>) -> Result<Vec<Line>, Span> {
        self.variables
            .get(&target.value)
            .ok_or(target.span)
            .map(|&offset| {
                vec![
                    Line::Instruction(Instruction::Mov(Writable::Register(Register::Rax), Readable::Register(Register::Rbp))),
                    Line::Instruction(Instruction::Sub(Register::Rax, Readable::Literal(offset))),
                    Line::Instruction(Instruction::Mov(Writable::Register(Register::Rax), Readable::Address(Address::new(Register::Rax)))),
                    Line::Instruction(Instruction::Push(Readable::Register(Register::Rax)))
                ]
            })
    }

    // RSPの指す値を代入
    pub fn declare(&mut self, target: &String) -> Result<Vec<Line>, ()> {
        self.next += 8;
        match self.variables.insert(target.clone(), self.next) {
            Some(_) => Err(()),
            None => {
                Ok(
                    vec![
                        Line::Instruction(Instruction::Mov(Writable::Register(Register::Rax), Readable::Register(Register::Rbp))),
                        Line::Instruction(Instruction::Sub(Register::Rax, Readable::Literal(self.next))),
                        Line::Instruction(Instruction::Pop(Register::Rdi)),
                        Line::Instruction(Instruction::Mov(Writable::Address(Address::new(Register::Rax)), Readable::Register(Register::Rdi)))
                    ]
                )
            },
        }
    }

    pub fn prologue(&self) -> Vec<Line> {
        vec![
            Line::Instruction(Instruction::Push(Readable::Register(Register::Rbp))),
            Line::Instruction(Instruction::Mov(Writable::Register(Register::Rbp), Readable::Register(Register::Rsp))),
            Line::Instruction(Instruction::Sub(Register::Rsp, Readable::Literal(self.next)))
        ]
    }

    pub fn epilogue(&self) -> Vec<Line> {
        vec![
            Line::Instruction(Instruction::Mov(Writable::Register(Register::Rsp), Readable::Register(Register::Rbp))),
            Line::Instruction(Instruction::Pop(Register::Rbp)),
            Line::Instruction(Instruction::Ret),
        ]
    }
}