use sourcecode::Position;

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
    -> Result<Statement, (Option<Position>, String)> {
        Assignment::parse(&mut token_reader)
        .map(Statement::Assignment)
        .or_else(|_| Return::parse(&mut token_reader).map(Statement::Return))
    }
}

pub struct Assignment {
    identifier: String,
    content: Expression,
}

impl Assignment {
    pub fn identifier(&self) -> &String {
        &self.identifier
    }
    pub fn content(&self) -> &Expression {
        &self.content
    }

    fn parse(token_reader: &mut TokenReader)
    -> Result<Assignment, (Option<Position>, String)> {
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
}

pub struct Return {
    content: Expression,
}

impl Return {
    pub fn content(&self) -> &Expression {
        return &self.content
    }

    fn parse(token_reader: &mut TokenReader)
    -> Result<Return, (Option<Position>, String)> {
        match token_reader.consume_reserved_word(ReservedWord::Return) {
            Ok(_) => Ok(()),
            Err(err) => Err((err, String::from("returnを期待していました")))
        }
        .and_then(|()| match Expression::parse(token_reader) {
            Ok(content) => Ok(Return{content}),
            Err(err) => Err(err)
        })
    }
}