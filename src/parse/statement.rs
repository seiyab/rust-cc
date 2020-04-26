use general::SemiGroup;
use general::TryReader;

use sourcecode::Span;
use sourcecode::Code;

use token::Operator;
use token::Token;
use token::ReservedWord;

use parse::SyntaxTree;
use parse::Expression;

pub enum Statement {
    Assignment(Assignment),
    Return(Return),
}

impl SyntaxTree for Statement {
    fn parse(mut token_reader: &mut TryReader<Code<Token>>)
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

    fn parse(token_reader: &mut TryReader<Code<Token>>)
    -> Result<Assignment, (Option<Span>, String)> {
        token_reader.drop_while(|token| token.value == Token::LineBreak);
        match token_reader.try_next(|token| {
            match token.value {
                Token::ReservedWord(ReservedWord::Let) => Ok(()),
                _ => Err(Some(token.span)),
            }
        }) {
            Ok(_) => (),
            Err(err) => return Err((err, String::from("letを期待していました")))
        };
        let identifier = match token_reader.try_next(|token| {
            match &token.value {
                Token::Identifier(name) => Ok(token.map_const(name.clone())),
                _ => Err(token.span)
            }
        }) {
            Ok(name) => name,
            Err(span) => return Err((Some(span), "識別子を期待していました。".to_string()))
        };
        match token_reader.try_next(|token| {
            match token.value {
                Token::Operator(Operator::Assign) => Ok(()),
                _ => Err(token.span)
            }
        }) {
            Ok(_) => (),
            Err(err) => return Err((Some(err), String::from("代入演算子を期待していました"))),
        };
        let content = match Expression::parse(token_reader) {
            Ok(expr) => expr,
            Err(e) => return Err(e),
        };

        Ok(Self {identifier, content})
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

    fn parse(token_reader: &mut TryReader<Code<Token>>)
    -> Result<Return, (Option<Span>, String)> {
        token_reader.drop_while(|token| token.value == Token::LineBreak);
        match token_reader.try_next(|token| {
            match token.value {
                Token::ReservedWord(ReservedWord::Return) => Ok(token.span),
                _ => Err(token.span)
            }
        }) {
            Ok(ret_span) => Ok(ret_span),
            Err(err) => Err((Some(err), String::from("returnを期待していました")))
        }
        .and_then(|ret_span| match Expression::parse(token_reader) {
            Ok(content) => Ok(Return{content, return_span: ret_span}),
            Err(err) => Err(err)
        })
    }

    fn span(&self) -> Span {
        self.return_span.plus(&self.content.span())
    }
}