use general::TryReader;
use::general::SemiGroup;

use sourcecode::Span;
use sourcecode::Code;

use token::Token;
use token::BracketSide;
use token::Bracket;
use token::ReservedWord;

use parse::SyntaxTree;
use parse::Statement;
use parse::Equality;

pub enum Expression {
    PureExpression(PureExpression),
    IfExpression(IfExpression),
    BlockExpression(BlockExpression),
}

impl SyntaxTree for Expression {
    fn parse(mut token_reader: &mut TryReader<Code<Token>>)
    -> Result<Expression, (Option<Span>, String)> {
        match token_reader.try_(|reader| IfExpression::parse(reader)) {
            Ok((_, expr)) => return Ok(Self::IfExpression(expr)),
            _ => (),
        }

        match token_reader.try_(|reader| BlockExpression::parse(reader)) {
            Ok((_, expr)) => return Ok(Self::BlockExpression(expr)),
            _ => (),
        }

        PureExpression::parse(&mut token_reader)
        .map(Self::PureExpression)
    }

    fn span(&self) -> Span {
        match &self {
            Self::PureExpression(expr) => expr.span(),
            Self::IfExpression(expr) => expr.span(),
            Self::BlockExpression(expr) => expr.span(),
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
        token_reader.drop_while(|token| token.value == Token::LineBreak);
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
        token_reader.drop_while(|token| token.value == Token::LineBreak);
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

pub struct BlockExpression {
    pub statements: Vec::<Statement>,
    pub outcome: Box<Expression>,
    open: Span,
    close: Span,
}

impl SyntaxTree for BlockExpression {
    fn parse(token_reader: &mut TryReader<Code<Token>>)
    -> Result<Self, (Option<Span>, String)> {
        let open = match token_reader.try_next(|token| {
            match token.value {
                Token::Bracket(BracketSide::Left(Bracket::Curly)) => Ok(token.span),
                _ => Err(token.span)
            }
        }) {
            Ok(span) => span,
            Err(e) => return Err((e, "{ を期待していました".to_string())),
        };

        token_reader.drop_while(|token| token.value == Token::LineBreak);

        let mut statements = Vec::new();
        while let Ok((_, statement)) = token_reader.try_(|reader| Statement::parse(reader)) {
            statements.push(statement)
        }

        token_reader.drop_while(|token| token.value == Token::LineBreak);

        let outcome = Box::new(match token_reader.try_(|reader| Expression::parse(reader)) {
            Ok((_, expr)) => expr,
            Err(e) => return Err(e),
        });

        token_reader.drop_while(|token| token.value == Token::LineBreak);

        let close = match token_reader.try_next(|token| {
            match token.value {
                Token::Bracket(BracketSide::Right(Bracket::Curly)) => Ok(token.span),
                _ => Err((Some(token.span), "} を期待していました".to_string())),
            }
        }) {
            Ok(span) => span,
            Err(Some(e)) => return Err(e),
            _ => return Err((None, "ブロックを期待しいていました".to_string())),
        };

        Ok(Self {
            open,
            close,
            statements,
            outcome,
        })
    }

    fn span(&self) -> Span {
        self.statements.iter().fold(self.open, |acc, s| acc.plus(&s.span()))
            .plus(&self.outcome.span())
            .plus(&self.close)
    }
}