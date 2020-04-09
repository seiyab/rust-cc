use general::SemiGroup;

use sourcecode::Span;
use sourcecode::Code;

use token::Operator;
use token::TokenReader;
use token::ReservedWord;

use parse::SyntaxTree;
use parse::Expression;

pub enum Statement {
    Assignment(Assignment),
    Return(Return),
}

impl SyntaxTree for Statement {
    fn parse(mut token_reader: &mut TokenReader)
    -> Result<Statement, (Option<Span>, String)> {
        Assignment::parse(&mut token_reader)
        .map(Statement::Assignment)
        .or_else(|_| Return::parse(&mut token_reader).map(Statement::Return))
    }

    fn span(&self) -> Span {
        match self {
            Statement::Assignment(assignment) => assignment.span(),
            Statement::Return(return_) => return_.span(),
        }
    }
}

pub struct Assignment {
    identifier: Code<String>,
    content: Expression,
}

impl Assignment {
    pub fn identifier(&self) -> &Code<String> {
        &self.identifier
    }
    pub fn content(&self) -> &Expression {
        &self.content
    }

    fn parse(token_reader: &mut TokenReader)
    -> Result<Assignment, (Option<Span>, String)> {
        match token_reader.consume_reserved_word(ReservedWord::Let) {
            Ok(_) => Ok(()),
            Err(err) => Err((err, String::from("letを期待していました")))
        }
        .and_then(|()| match token_reader.consume_identifier() {
            Ok(name) => Ok(name),
            Err(err) => Err((err, String::from("識別子を期待していました"))),
        })
        .and_then(|name| match token_reader.consume_operator(Operator::Assign) {
            Ok(_) => Ok(name),
            Err(err) => Err((err, String::from("代入演算子を期待していました"))),
        })
        .and_then(|identifier|
            Expression::parse(token_reader).map(|content| { Assignment{identifier, content} })
        )
    }

    fn span(&self) -> Span {
        self.identifier.span.plus(&self.content.span())
    }
}

pub struct Return {
    return_span: Span,
    content: Expression,
}

impl Return {
    pub fn content(&self) -> &Expression {
        return &self.content
    }

    fn parse(token_reader: &mut TokenReader)
    -> Result<Return, (Option<Span>, String)> {
        match token_reader.consume_reserved_word(ReservedWord::Return) {
            Ok(ret) => Ok(ret),
            Err(err) => Err((err, String::from("returnを期待していました")))
        }
        .and_then(|ret| match Expression::parse(token_reader) {
            Ok(content) => Ok(Return{content, return_span: ret.span}),
            Err(err) => Err(err)
        })
    }

    fn span(&self) -> Span {
        self.return_span.plus(&self.content.span())
    }
}