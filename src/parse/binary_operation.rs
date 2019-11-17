use std::collections::HashSet;

use sourcecode::Position;

use token::Token;
use token::TokenReader;
use token::Operator;

use parse::SyntaxTree;

pub enum BinaryOperation<Element: SyntaxTree> {
    Single(Element),
    Binary {
        operator: Operator,
        left: Element,
        right: Box<Self>,
    },
}

impl <Element: SyntaxTree> BinaryOperation<Element> {
    pub fn parse(mut token_reader: &mut TokenReader, operators: &HashSet<Operator>)
    -> Result<Self, (Option<Position>, String)> {
        let left = match Element::parse(&mut token_reader) {
            Ok(element) => element,
            Err(err) => return Err(err),
        };
        let maybe_operator = token_reader.peek().and_then(|findable_token| {
            match findable_token.value() {
                &Token::Operator(op) if operators.contains(&op) => Some(op),
                _ => None,
            }
        });
        match maybe_operator {
            None => Ok(Self::Single(left)),
            Some(operator) => {
                token_reader.skip();
                match Self::parse(&mut token_reader, operators) {
                    Ok(right) => Ok(Self::Binary {left, operator, right: Box::new(right)}),
                    Err(err) => Err(err),
                }
            },
        }
    }

    pub fn head(&self) -> &Element {
        match self {
            Self::Single(element) => &element,
            Self::Binary { left, right: _, operator: _ } => &left,
        }
    }

    pub fn tail(&self) -> impl Iterator<Item = (Operator, &Element)> {
        let mut tail = Vec::new();
        let mut node = self;
        while let Self::Binary { left: _, right, operator } = node {
            tail.push((operator.clone(), right.head()));
            node = right;
        }
        tail.into_iter()
    }
}