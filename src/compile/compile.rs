use token::Operator;

use parse::SyntaxTree;
use parse::Expression;
use parse::Multiply;

use compile::assembly::Instruction;
use compile::assembly::Register;
use compile::assembly::Readable;

pub fn compile(syntaxtree: &SyntaxTree) -> Vec<Instruction> {
    compile_expression(syntaxtree.expression())
}

fn compile_expression(expression: &Expression) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let head = expression.head();
    instructions.append(&mut compile_multiply(head));
    for (findable_operator, multiply) in expression.tail() {
        instructions.append(&mut compile_multiply(multiply));
        instructions.push(Instruction::Pop(Register::Rdi));
        instructions.push(Instruction::Pop(Register::Rax));
        match findable_operator.value() {
            &Operator::Add => instructions.push(Instruction::Add(Register::Rax, Readable::Register(Register::Rdi))),
            _ => instructions.push(Instruction::Sub(Register::Rax, Readable::Register(Register::Rdi))),
        }
        instructions.push(Instruction::Push(Readable::Register(Register::Rax)));
    }
    instructions
}

fn compile_multiply(multiply: &Multiply) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let head = multiply.head();
    instructions.push(Instruction::Push(Readable::Integer(*head.value())));
    for (findable_operator, findable_i64) in multiply.tail() {
        instructions.push(Instruction::Push(Readable::Integer(*findable_i64.value())));
        instructions.push(Instruction::Pop(Register::Rdi));
        instructions.push(Instruction::Pop(Register::Rax));
        match findable_operator.value() {
            &Operator::Mul => instructions.push(Instruction::Imul(Register::Rax, Readable::Register(Register::Rdi))),
            _ => {
                instructions.push(Instruction::Cqo);
                instructions.push(Instruction::Idiv(Register::Rdi));
            }
        }
        instructions.push(Instruction::Push(Readable::Register(Register::Rax)))
    }
    instructions
}
