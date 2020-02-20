use token::Operator;

use parse::Root;
use parse::Statement;
use parse::Expression;
use parse::Equality;
use parse::Relational;
use parse::Add;
use parse::Multiply;
use parse::Unary;
use parse::Primary;

use compile::assembly::Instruction;
use compile::assembly::Register;
use compile::scope::CurrentScope;
use compile::scope::Scope;

pub fn compile(syntaxtree: &Root) -> Vec<Instruction> {
    let mut scope = CurrentScope::new();
    let mut processes = Vec::new();
    for statement in syntaxtree.statements() {
        processes.append(&mut compile_statement(statement, &mut scope))
    }
    let mut instructions = scope.prologue();
    instructions.append(&mut processes);
    instructions.push(Instruction::Pop(Register::Rax));
    instructions.append(&mut scope.epilogue());
    instructions
}

fn compile_statement(statement: &Statement, scope: &mut dyn Scope) -> Vec<Instruction> {
    match statement {
        Statement::Assignment(asn) => {
            let mut instructions = compile_expression(asn.content(), scope);
            instructions.append(&mut scope.declare(asn.identifier()).unwrap());
            instructions
        }
        Statement::Return(ret) => compile_expression(ret.content(), scope)
    }
}

fn compile_expression(expression: &Expression, scope: &dyn Scope) -> Vec<Instruction> {
    compile_equality(expression.equality(), scope)
}

fn compile_equality(equality: &Equality, scope: &dyn Scope) -> Vec<Instruction> {
    let mut instructions = compile_relational(equality.head(), scope);
    for (operator, relational) in equality.tail() {
        instructions.append(&mut compile_relational(relational, scope));
        instructions.push(Instruction::Pop(Register::Rdi));
        instructions.push(Instruction::Pop(Register::Rax));
        instructions.push(Instruction::Cmp(Register::Rax, Box::new(Register::Rdi)));
        let setx = match operator {
            Operator::Equal => Instruction::Sete(Register::Al),
            _ => Instruction::Setne(Register::Al),
        };
        instructions.push(setx);
        instructions.push(Instruction::Movzb(Register::Rax, Box::new(Register::Al)));
        instructions.push(Instruction::Push(Box::new(Register::Rax)));
    }
    instructions
}

fn compile_relational(relational: &Relational, scope: &dyn Scope) -> Vec<Instruction> {
    let mut instructions = compile_add(relational.head(), scope);
    for (operator, add) in relational.tail() {
        instructions.append(&mut compile_add(add, scope));
        instructions.push(Instruction::Pop(Register::Rdi));
        instructions.push(Instruction::Pop(Register::Rax));
        instructions.push(Instruction::Cmp(Register::Rax, Box::new(Register::Rdi)));
        let setx = match operator {
            Operator::Less => Instruction::Setl(Register::Al),
            Operator::LessEq => Instruction::Setle(Register::Al),
            Operator::Greater => Instruction::Setg(Register::Al),
            _ => Instruction::Setge(Register::Al),
        };
        instructions.push(setx);
        instructions.push(Instruction::Movzb(Register::Rax, Box::new(Register::Al)));
        instructions.push(Instruction::Push(Box::new(Register::Rax)));
    }
    instructions
}

fn compile_add(add: &Add, scope: &dyn Scope) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let head = add.head();
    instructions.append(&mut compile_multiply(head, scope));
    for (operator, multiply) in add.tail() {
        instructions.append(&mut compile_multiply(multiply, scope));
        instructions.push(Instruction::Pop(Register::Rdi));
        instructions.push(Instruction::Pop(Register::Rax));
        match operator {
            Operator::Add => instructions.push(Instruction::Add(Register::Rax, Box::new(Register::Rdi))),
            _ => instructions.push(Instruction::Sub(Register::Rax, Box::new(Register::Rdi))),
        }
        instructions.push(Instruction::Push(Box::new(Register::Rax)));
    }
    instructions
}

fn compile_multiply(multiply: &Multiply, scope: &dyn Scope) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let head = multiply.head();
    instructions.append(&mut compile_unary(head, scope));
    for (operator, unary) in multiply.tail() {
        instructions.append(&mut compile_unary(unary, scope));
        instructions.push(Instruction::Pop(Register::Rdi));
        instructions.push(Instruction::Pop(Register::Rax));
        match operator {
            Operator::Mul => instructions.push(Instruction::Imul(Register::Rax, Box::new(Register::Rdi))),
            _ => {
                instructions.push(Instruction::Cqo);
                instructions.push(Instruction::Idiv(Register::Rdi));
            }
        }
        instructions.push(Instruction::Push(Box::new(Register::Rax)))
    }
    instructions
}

fn compile_unary(unary: &Unary, scope: &dyn Scope) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    match &unary {
        &Unary::Positive(primary) => instructions.append(&mut compile_primary(&primary, scope)),
        &Unary::Negative(primary) => {
            instructions.append(&mut compile_primary(&primary, scope));
            instructions.push(Instruction::Pop(Register::Rdi));
            instructions.push(Instruction::Mov(Box::new(Register::Rax), Box::new(0)));
            instructions.push(Instruction::Sub(Register::Rax, Box::new(Register::Rdi)));
            instructions.push(Instruction::Push(Box::new(Register::Rax)));
        }
    }
    instructions
}

fn compile_primary(primary: &Primary, scope: &dyn Scope) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    match &primary {
        &Primary::Integer(n) => instructions.push(Instruction::Push(Box::new(*n))),
        &Primary::Identifier(name) => instructions.append(&mut scope.lookup(&name).unwrap()),
        &Primary::Expression(expression) => instructions.append(&mut compile_expression(&expression, scope)),
    }
    instructions
}
