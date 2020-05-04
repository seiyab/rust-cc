use sourcecode::Span;

use token::Operator;

use parse::Root;
use parse::Statement;
use parse::Expression;
use parse::PureExpression;
use parse::IfExpression;
use parse::BlockExpression;
use parse::Equality;
use parse::Relational;
use parse::Add;
use parse::Multiply;
use parse::Unary;
use parse::Primary;

use super::scope::Scope;
use super::assembly::Instruction;
use super::assembly::Label;
use super::assembly::Line;
use super::assembly::Register;
use super::assembly::Readable;
use super::assembly::Writable;

pub struct Compiler {
    pub lines: Vec<Line>,
    next_label: u64,
}

impl  Compiler {
    pub fn compile(syntaxtree: &Root) -> Result<Self, (Span, String)> {
        let mut scope = Scope::new();
        let mut assembly = Self{
            lines: Vec::new(),
            next_label: 0,
        };
        let mut processes = Vec::new();
        for statement in syntaxtree.statements() {
            match assembly.compile_statement(statement, &mut scope) {
                Ok(mut statement_instructions) => processes.append(&mut statement_instructions),
                Err(e) => return Err(e),
            }
        }
        assembly.lines.append(&mut scope.prologue());
        assembly.lines.append(&mut processes);
        assembly.lines.append(&mut scope.epilogue());
        Ok(assembly)
    }

    pub fn assembly_string(&self) -> String {
        self.lines.iter().map(|line| {
            match line {
                Line::Instruction(instruction) => format!("  {}", instruction.destination_code()),
                Line::Label(label) => format!("{}:", label.name),
            }
        })
        .fold("".to_string(), |mut acc, s| {
            acc.push('\n');
            acc.push_str(&s);
            acc
        })
    }

    fn new_label(&mut self) -> Label {
        let index = self.next_label;
        self.next_label += 1;
        Label {
            name: format!(".L{}", index)
        }
    }

    fn compile_statement(&mut self, statement: &Statement, scope: &mut Scope) -> Result<Vec<Line>, (Span, String)> {
        match statement {
            Statement::Assignment(asn) => {
                match self.compile_expression(asn.content(), scope) {
                    Ok(mut assign_lines) => {
                        assign_lines.append(&mut scope.declare(&asn.identifier().value).unwrap());
                        Ok(assign_lines)
                    },
                    Err(e) => Err(e),
                }
            },
            Statement::Return(ret) => {
                match self.compile_expression(ret.content(), scope) {
                    Ok(mut return_ines) => {
                        return_ines.push(Line::Instruction(Instruction::Pop(Register::Rax)));
                        Ok(return_ines)
                    },
                    Err(e) => Err(e),
                }
            }
        }
    }

    fn compile_expression(&mut self, expression: &Expression, scope: &mut Scope) -> Result<Vec<Line>, (Span, String)> {
        match expression {
            Expression::PureExpression(expr) => self.compile_pure_expression(expr, scope),
            Expression::IfExpression(expr) => self.compile_if_expression(expr, scope),
            Expression::BlockExpression(expr) => self.compile_block_expression(expr, scope),
        }
    }

    fn compile_pure_expression(&mut self, expr: &PureExpression, scope: &mut Scope) -> Result<Vec<Line>, (Span, String)> {
        self.compile_equality(&expr.equality, scope)
    }

    fn compile_if_expression(&mut self, expr: &IfExpression, scope: &mut Scope) -> Result<Vec<Line>, (Span, String)> {
        let else_label = self.new_label();
        let end_label = self.new_label();
        let condition_lines = match self.compile_expression(&*expr.condition, scope) {
            Ok(lines) => lines,
            Err(e) => return Err(e),
        };
        let mut then_lines = match self.compile_expression(&*expr.then, scope) {
            Ok(lines) => lines,
            Err(e) => return Err(e),
        };
        let mut else_lines = match self.compile_expression(&*expr.else_, scope) {
            Ok(lines) => lines,
            Err(e) => return Err(e),
        };

        let mut lines = condition_lines;
        lines.append(&mut vec![
            Line::Instruction(Instruction::Pop(Register::Rax)),
            Line::Instruction(Instruction::Cmp(Register::Rax, Readable::Literal(0))),
            Line::Instruction(Instruction::Je(else_label.clone())),
        ]);
        lines.append(&mut then_lines);
        lines.push(Line::Instruction(Instruction::Jmp(end_label.clone())));
        lines.push(Line::Label(else_label));
        lines.append(&mut else_lines);
        lines.push(Line::Label(end_label));

        Ok(lines)
    }

    fn compile_block_expression(&mut self, expr: &BlockExpression, scope: &mut Scope) -> Result<Vec<Line>, (Span, String)> {
        let mut lines = Vec::new();
        for stmt in &expr.statements {
            match self.compile_statement(stmt, scope) {
                Ok(mut stmt_lines) => lines.append(&mut stmt_lines),
                Err(e) => return Err(e),
            }
        }
        match self.compile_expression(&*expr.outcome, scope) {
            Ok(mut expr_lines) => lines.append(&mut expr_lines),
            Err(e) => return Err(e),
        }
        Ok(lines)
    }

    fn compile_equality(&mut self, equality: &Equality, scope: &mut Scope) -> Result<Vec<Line>, (Span, String)> {
        let mut lines = Vec::new();
        match self.compile_relational(equality.head(), scope) {
            Ok(mut relational_lines) => lines.append(&mut relational_lines),
            Err(e) => return Err(e),
        }
        for (operator, relational) in equality.tail() {
            match self.compile_relational(relational, scope) {
                Ok(mut relational_lines) => lines.append(&mut relational_lines),
                Err(e) => return Err(e),
            }
            lines.push(Line::Instruction(Instruction::Pop(Register::Rdi)));
            lines.push(Line::Instruction(Instruction::Pop(Register::Rax)));
            lines.push(Line::Instruction(Instruction::Cmp(Register::Rax, Readable::Register(Register::Rdi))));
            let setx = match operator.value {
                Operator::Equal => Instruction::Sete(Register::Al),
                _ => Instruction::Setne(Register::Al),
            };
            lines.push(Line::Instruction(setx));
            lines.push(Line::Instruction(Instruction::Movzb(Register::Rax, Readable::Register(Register::Al))));
            lines.push(Line::Instruction(Instruction::Push(Readable::Register(Register::Rax))));
        }
        Ok(lines)
    }

    fn compile_relational(&mut self, relational: &Relational, scope: &mut Scope) -> Result<Vec<Line>, (Span, String)> {
        let mut lines = Vec::new();
        match self.compile_add(relational.head(), scope) {
            Ok(mut add_lines) => lines.append(&mut add_lines),
            Err(e) => return Err(e),
        }
        for (operator, add) in relational.tail() {
            match self.compile_add(add, scope) {
                Ok(mut add_lines) => lines.append(&mut add_lines),
                Err(e) => return Err(e),
            }
            lines.push(Line::Instruction(Instruction::Pop(Register::Rdi)));
            lines.push(Line::Instruction(Instruction::Pop(Register::Rax)));
            lines.push(Line::Instruction(Instruction::Cmp(Register::Rax, Readable::Register(Register::Rdi))));
            let setx = match operator.value {
                Operator::Less => Instruction::Setl(Register::Al),
                Operator::LessEq => Instruction::Setle(Register::Al),
                Operator::Greater => Instruction::Setg(Register::Al),
                _ => Instruction::Setge(Register::Al),
            };
            lines.push(Line::Instruction(setx));
            lines.push(Line::Instruction(Instruction::Movzb(Register::Rax, Readable::Register(Register::Al))));
            lines.push(Line::Instruction(Instruction::Push(Readable::Register(Register::Rax))));
        }
        Ok(lines)
    }

    fn compile_add(&mut self, add: &Add, scope: &mut Scope) -> Result<Vec<Line>, (Span, String)> {
        let mut lines = Vec::new();
        let head = add.head();
        match self.compile_multiply(head, scope) {
            Ok(mut multiply_lines) => lines.append(&mut multiply_lines),
            Err(e) => return Err(e),
        }
        for (operator, multiply) in add.tail() {
            match self.compile_multiply(multiply, scope) {
                Ok(mut multiply_lines) => lines.append(&mut multiply_lines),
                Err(e) => return Err(e),
            }
            lines.push(Line::Instruction(Instruction::Pop(Register::Rdi)));
            lines.push(Line::Instruction(Instruction::Pop(Register::Rax)));
            match operator.value {
                Operator::Add => lines.push(Line::Instruction(Instruction::Add(Register::Rax, Readable::Register(Register::Rdi)))),
                _ => lines.push(Line::Instruction(Instruction::Sub(Register::Rax, Readable::Register(Register::Rdi)))),
            }
            lines.push(Line::Instruction(Instruction::Push(Readable::Register(Register::Rax))));
        }
        Ok(lines)
    }

    fn compile_multiply(&mut self, multiply: &Multiply, scope: &mut Scope) -> Result<Vec<Line>, (Span, String)> {
        let mut lines = Vec::new();
        let head = multiply.head();
        match self.compile_unary(head, scope) {
            Ok(mut unary_lines) => lines.append(&mut unary_lines),
            Err(e) => return Err(e),
        }
        for (operator, unary) in multiply.tail() {
            match self.compile_unary(unary, scope) {
                Ok(mut unary_lines) => lines.append(&mut unary_lines),
                Err(e) => return Err(e),
            }
            lines.push(Line::Instruction(Instruction::Pop(Register::Rdi)));
            lines.push(Line::Instruction(Instruction::Pop(Register::Rax)));
            match operator.value {
                Operator::Mul => lines.push(Line::Instruction(Instruction::Imul(Register::Rax, Readable::Register(Register::Rdi)))),
                _ => {
                    lines.push(Line::Instruction(Instruction::Cqo));
                    lines.push(Line::Instruction(Instruction::Idiv(Register::Rdi)));
                }
            }
            lines.push(Line::Instruction(Instruction::Push(Readable::Register(Register::Rax))))
        }
        Ok(lines)
    }

    fn compile_unary(&mut self, unary: &Unary, scope: &mut Scope) -> Result<Vec<Line>, (Span, String)> {
        let mut lines = Vec::new();
        match &unary {
            &Unary::Positive(primary, _) => {
                match self.compile_primary(&primary, scope) {
                    Ok(mut primary_lines) => lines.append(&mut primary_lines),
                    Err(e) => return Err(e),
                }
            }
            &Unary::Negative(primary, _) => {
                match self.compile_primary(&primary, scope) {
                    Ok(mut primary_lines) => lines.append(&mut primary_lines),
                    Err(e) => return Err(e),
                };
                lines.push(Line::Instruction(Instruction::Pop(Register::Rdi)));
                lines.push(Line::Instruction(Instruction::Mov(Writable::Register(Register::Rax), Readable::Literal(0))));
                lines.push(Line::Instruction(Instruction::Sub(Register::Rax, Readable::Register(Register::Rdi))));
                lines.push(Line::Instruction(Instruction::Push(Readable::Register(Register::Rax))));
            }
        }
        Ok(lines)
    }

    fn compile_primary(&mut self, primary: &Primary, scope: &mut Scope) -> Result<Vec<Line>, (Span, String)> {
        let mut lines = Vec::new();
        match &primary {
            &Primary::Integer(n) => lines.push(Line::Instruction(Instruction::Push(Readable::Literal(n.value)))),
            &Primary::Identifier(name) => {
                match scope.lookup(&name) {
                    Ok(mut lookup_lines) => lines.append(&mut lookup_lines),
                    Err(span) => return Err((span, String::from("未定義のシンボルです。"))),
                }
            },
            &Primary::Expression(expression) => {
                match self.compile_expression(&expression, scope) {
                    Ok(mut expression_lines) => lines.append(&mut expression_lines),
                    Err(e) => return Err(e),
                }
            },
        }
        Ok(lines)
    }

}