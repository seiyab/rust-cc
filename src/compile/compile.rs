use sourcecode::Span;

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

pub fn compile(syntaxtree: &Root) -> Result<Vec<Instruction>, (Span, String)> {
    let mut scope = CurrentScope::new();
    let mut processes = Vec::new();
    for statement in syntaxtree.statements() {
        match compile_statement(statement, &mut scope) {
            Ok(mut statement_instructions) => processes.append(&mut statement_instructions),
            Err(e) => return Err(e),
        }
    }
    let mut instructions = scope.prologue();
    instructions.append(&mut processes);
    instructions.push(Instruction::Pop(Register::Rax));
    instructions.append(&mut scope.epilogue());
    Ok(instructions)
}

fn compile_statement(statement: &Statement, scope: &mut dyn Scope) -> Result<Vec<Instruction>, (Span, String)> {
    match statement {
        Statement::Assignment(asn) => {
            match compile_expression(asn.content(), scope) {
                Ok(mut assign_instructions) => {
                    assign_instructions.append(&mut scope.declare(&asn.identifier().value).unwrap());
                    Ok(assign_instructions)
                },
                Err(e) => Err(e),
            }
        },
        Statement::Return(ret) => compile_expression(ret.content(), scope)
    }
}

fn compile_expression(expression: &Expression, scope: &dyn Scope) -> Result<Vec<Instruction>, (Span, String)> {
    compile_equality(expression.equality(), scope)
}

fn compile_equality(equality: &Equality, scope: &dyn Scope) -> Result<Vec<Instruction>, (Span, String)> {
    let mut instructions = Vec::new();
    match compile_relational(equality.head(), scope) {
        Ok(mut relational_instructions) => instructions.append(&mut relational_instructions),
        Err(e) => return Err(e),
    }
    for (operator, relational) in equality.tail() {
        match compile_relational(relational, scope) {
            Ok(mut relational_instructions) => instructions.append(&mut relational_instructions),
            Err(e) => return Err(e),
        }
        instructions.push(Instruction::Pop(Register::Rdi));
        instructions.push(Instruction::Pop(Register::Rax));
        instructions.push(Instruction::Cmp(Register::Rax, Box::new(Register::Rdi)));
        let setx = match operator.value {
            Operator::Equal => Instruction::Sete(Register::Al),
            _ => Instruction::Setne(Register::Al),
        };
        instructions.push(setx);
        instructions.push(Instruction::Movzb(Register::Rax, Box::new(Register::Al)));
        instructions.push(Instruction::Push(Box::new(Register::Rax)));
    }
    Ok(instructions)
}

fn compile_relational(relational: &Relational, scope: &dyn Scope) -> Result<Vec<Instruction>, (Span, String)> {
    let mut instructions = Vec::new();
    match compile_add(relational.head(), scope) {
        Ok(mut add_instructions) => instructions.append(&mut add_instructions),
        Err(e) => return Err(e),
    }
    for (operator, add) in relational.tail() {
        match compile_add(add, scope) {
            Ok(mut add_instructions) => instructions.append(&mut add_instructions),
            Err(e) => return Err(e),
        }
        instructions.push(Instruction::Pop(Register::Rdi));
        instructions.push(Instruction::Pop(Register::Rax));
        instructions.push(Instruction::Cmp(Register::Rax, Box::new(Register::Rdi)));
        let setx = match operator.value {
            Operator::Less => Instruction::Setl(Register::Al),
            Operator::LessEq => Instruction::Setle(Register::Al),
            Operator::Greater => Instruction::Setg(Register::Al),
            _ => Instruction::Setge(Register::Al),
        };
        instructions.push(setx);
        instructions.push(Instruction::Movzb(Register::Rax, Box::new(Register::Al)));
        instructions.push(Instruction::Push(Box::new(Register::Rax)));
    }
    Ok(instructions)
}

fn compile_add(add: &Add, scope: &dyn Scope) -> Result<Vec<Instruction>, (Span, String)> {
    let mut instructions = Vec::new();
    let head = add.head();
    match compile_multiply(head, scope) {
        Ok(mut multiply_instructions) => instructions.append(&mut multiply_instructions),
        Err(e) => return Err(e),
    }
    for (operator, multiply) in add.tail() {
        match compile_multiply(multiply, scope) {
            Ok(mut multiply_instructions) => instructions.append(&mut multiply_instructions),
            Err(e) => return Err(e),
        }
        instructions.push(Instruction::Pop(Register::Rdi));
        instructions.push(Instruction::Pop(Register::Rax));
        match operator.value {
            Operator::Add => instructions.push(Instruction::Add(Register::Rax, Box::new(Register::Rdi))),
            _ => instructions.push(Instruction::Sub(Register::Rax, Box::new(Register::Rdi))),
        }
        instructions.push(Instruction::Push(Box::new(Register::Rax)));
    }
    Ok(instructions)
}

fn compile_multiply(multiply: &Multiply, scope: &dyn Scope) -> Result<Vec<Instruction>, (Span, String)> {
    let mut instructions = Vec::new();
    let head = multiply.head();
    match compile_unary(head, scope) {
        Ok(mut unary_instructions) => instructions.append(&mut unary_instructions),
        Err(e) => return Err(e),
    }
    for (operator, unary) in multiply.tail() {
        match compile_unary(unary, scope) {
            Ok(mut unary_instructions) => instructions.append(&mut unary_instructions),
            Err(e) => return Err(e),
        }
        instructions.push(Instruction::Pop(Register::Rdi));
        instructions.push(Instruction::Pop(Register::Rax));
        match operator.value {
            Operator::Mul => instructions.push(Instruction::Imul(Register::Rax, Box::new(Register::Rdi))),
            _ => {
                instructions.push(Instruction::Cqo);
                instructions.push(Instruction::Idiv(Register::Rdi));
            }
        }
        instructions.push(Instruction::Push(Box::new(Register::Rax)))
    }
    Ok(instructions)
}

fn compile_unary(unary: &Unary, scope: &dyn Scope) -> Result<Vec<Instruction>, (Span, String)> {
    let mut instructions = Vec::new();
    match &unary {
        &Unary::Positive(primary, _) => {
            match compile_primary(&primary, scope) {
                Ok(mut primary_instructions) => instructions.append(&mut primary_instructions),
                Err(e) => return Err(e),
            }
        }
        &Unary::Negative(primary, _) => {
            match compile_primary(&primary, scope) {
                Ok(mut primary_instructions) => instructions.append(&mut primary_instructions),
                Err(e) => return Err(e),
            };
            instructions.push(Instruction::Pop(Register::Rdi));
            instructions.push(Instruction::Mov(Box::new(Register::Rax), Box::new(0)));
            instructions.push(Instruction::Sub(Register::Rax, Box::new(Register::Rdi)));
            instructions.push(Instruction::Push(Box::new(Register::Rax)));
        }
    }
    Ok(instructions)
}

fn compile_primary(primary: &Primary, scope: &dyn Scope) -> Result<Vec<Instruction>, (Span, String)> {
    let mut instructions = Vec::new();
    match &primary {
        &Primary::Integer(n) => instructions.push(Instruction::Push(Box::new(n.value))),
        &Primary::Identifier(name) => {
            match scope.lookup(&name) {
                Ok(mut lookup_instructions) => instructions.append(&mut lookup_instructions),
                Err(span) => return Err((span, String::from("未定義のシンボルです。"))),
            }
        },
        &Primary::Expression(expression) => {
            match compile_expression(&expression, scope) {
                Ok(mut expression_instructions) => instructions.append(&mut expression_instructions),
                Err(e) => return Err(e),
            }
        },
    }
    Ok(instructions)
}
