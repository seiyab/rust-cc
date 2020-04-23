use general::TryReader;
use::general::SemiGroup;

use sourcecode::Span;
use sourcecode::Code;

use token::Token;
use token::ReservedWord;

use parse::Equality;
use parse::SyntaxTree;

pub enum Expression {
    PureExpression(PureExpression),
    IfExpression(IfExpression),
}

impl SyntaxTree for Expression {
    fn parse(mut token_reader: &mut TryReader<Code<Token>>)
    -> Result<Expression, (Option<Span>, String)> {
        match token_reader.try_(|reader| IfExpression::parse(reader)) {
            Ok((_, expr)) => return Ok(Self::IfExpression(expr)),
            _ => (),
        }

        PureExpression::parse(&mut token_reader)
        .map(Self::PureExpression)
    }

    fn span(&self) -> Span {
        match &self {
            Self::PureExpression(expr) => expr.span(),
            Self::IfExpression(expr) => expr.span(),
        }
    }
}

pub struct PureExpression {
    pub equality: Equality,
}

impl SyntaxTree for PureExpression {
    fn parse(mut token_reader: &mut TryReader<Code<Token>>)
    -> Result<PureExpression, (Option<Span>, String)> {
        Equality::parse(&mut token_reader)
        .map(|equality| Self {equality})
    }

    fn span(&self) -> Span {
        self.equality.span()
    }
}

pub struct IfExpression {
    pub condition: Box<Expression>,
    pub then: Box<Expression>,
    pub else_: Box<Expression>,
}

impl SyntaxTree for IfExpression {
    fn parse(mut token_reader: &mut TryReader<Code<Token>>)
    -> Result<IfExpression, (Option<Span>, String)> {
        match token_reader.next() {
            Some(token) => match token.value {
                Token::ReservedWord(ReservedWord::If) => (),
                _ => return Err((Some(token.span), "ifを期待していました".to_string())),
            },
            _ => return Err((None, "ifを期待していました".to_string())),
        };
        let condition = match Expression::parse(&mut token_reader) {
            Ok(expression) => expression,
            Err(e) => return Err(e),
        };
        match token_reader.next() {
            Some(token) => match token.value {
                Token::ReservedWord(ReservedWord::Then) => (),
                _ => return Err((Some(token.span), "thenを期待していました".to_string())),
            },
            _ => return Err((None, "thenを期待していました".to_string())),
        };
        let then = match Expression::parse(&mut token_reader) {
            Ok(expression) => expression,
            Err(e) => return Err(e),
        };
        match token_reader.next() {
            Some(token) => match token.value {
                Token::ReservedWord(ReservedWord::Else) => (),
                _ => return Err((Some(token.span), "elseを期待していました".to_string())),
            },
            _ => return Err((None, "elseを期待していました".to_string())),
        };
        let else_ = match Expression::parse(&mut token_reader) {
            Ok(expression) => expression,
            Err(e) => return Err(e),
        };
        Ok(Self{
            condition: Box::new(condition),
            then: Box::new(then), 
            else_: Box::new(else_),
        })
    }

    // TODO: if が含まれていない
    fn span(&self) -> Span {
        self.condition.span()
            .plus(&self.then.span())
            .plus(&self.else_.span())
    }
}