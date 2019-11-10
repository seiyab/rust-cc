use token::Operator;

use parse::SyntaxTree;
use parse::Expression;
use parse::Relational;
use parse::Add;
use parse::Multiply;
use parse::Unary;
use parse::Primary;

use compile::assembly::Instruction;
use compile::assembly::Register;
use compile::assembly::Readable;

pub fn compile(syntaxtree: &SyntaxTree) -> Vec<Instruction> {
    let mut instructions = compile_expression(syntaxtree.expression());
    instructions.push(Instruction::Pop(Register::Rax));
    instructions.push(Instruction::Ret);
    instructions
}

fn compile_expression(expression: &Expression) -> Vec<Instruction> {
    compile_relational(expression.relational())
}

fn compile_relational(relational: &Relational) -> Vec<Instruction> {
    let mut instructions = compile_add(relational.head());
    for (findable_operator, add) in relational.tail() {
        instructions.append(&mut compile_add(add));
        instructions.push(Instruction::Pop(Register::Rdi));
        instructions.push(Instruction::Pop(Register::Rax));
        instructions.push(Instruction::Cmp(Register::Rax, Readable::Register(Register::Rdi)));
        let setx = match findable_operator.value() {
            Operator::Less => Instruction::Setl(Register::Al),
            Operator::LessEq => Instruction::Setle(Register::Al),
            Operator::Greater => Instruction::Setg(Register::Al),
            _ => Instruction::Setge(Register::Al),
        };
        instructions.push(setx);
        instructions.push(Instruction::Movzb(Register::Rax, Readable::Register(Register::Al)));
        instructions.push(Instruction::Push(Readable::Register(Register::Rax)));
    }
    instructions
}

fn compile_add(add: &Add) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let head = add.head();
    instructions.append(&mut compile_multiply(head));
    for (findable_operator, multiply) in add.tail() {
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
    instructions.append(&mut compile_unary(head));
    for (findable_operator, unary) in multiply.tail() {
        instructions.append(&mut compile_unary(unary));
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

fn compile_unary(unary: &Unary) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    match &unary {
        &Unary::Positive(primary) => instructions.append(&mut compile_primary(&primary)),
        &Unary::Negative(primary) => {
            instructions.append(&mut compile_primary(&primary));
            instructions.push(Instruction::Pop(Register::Rdi));
            instructions.push(Instruction::Mov(Register::Rax, 0));
            instructions.push(Instruction::Sub(Register::Rax, Readable::Register(Register::Rdi)));
            instructions.push(Instruction::Push(Readable::Register(Register::Rax)));
        }
    }
    instructions
}

fn compile_primary(primary: &Primary) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    match &primary {
        &Primary::Integer(n) => instructions.push(Instruction::Push(Readable::Integer(*n))),
        &Primary::Expression(expression) => instructions.append(&mut compile_expression(&expression)),
    }
    instructions
}
