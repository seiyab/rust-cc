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
    block_stack: Vec<i64>,
    block_seq: i64,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            next: 0,
            block_stack: Vec::new(),
            block_seq: 1,
        }
    }

    pub fn lookup(&self, target: &Code<String>) -> Result<Vec<Line>, Span> {
        let id = self.block_stack
            .iter().rev()
            .map(|i| format!("{}#{}", &target.value, i))
            .find(|id| self.variables.get(id) != None)
            .unwrap_or(format!("{}#{}", &target.value, 0));
        self.variables
            .get(&id)
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
        match self.variables.insert(self.variable_id(target), self.next) {
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

    fn variable_id(&self, name: &String) -> String {
        format!("{}#{}", name, self.block_stack.last().unwrap_or(&0))
    }

    pub fn into_block(&mut self) {
        self.block_seq += 1;
        self.block_stack.push(self.block_seq);
    }
    pub fn outof_block(&mut self) {
        self.block_stack.pop();
    }
}