use std::collections::HashSet;

use general::SemiGroup;

use sourcecode::Code;
use sourcecode::Span;

use token::Token;
use token::TokenReader;
use token::Operator;

use parse::SyntaxTree;

pub enum BinaryOperation<Element: SyntaxTree> {
    Single(Element),
    Binary {
        operator: Code<Operator>,
        left: Element,
        right: Box<Self>,
    },
}

impl <Element: SyntaxTree> BinaryOperation<Element> {
    pub fn parse(mut token_reader: &mut TokenReader, operators: &HashSet<Operator>)
    -> Result<Self, (Option<Span>, String)> {
        let left = match Element::parse(&mut token_reader) {
            Ok(element) => element,
            Err(err) => return Err(err),
        };
        let maybe_operator = token_reader.peek().and_then(|token| {
            match token.value {
                Token::Operator(op) if operators.contains(&op) => Some(token.map_const(op)),
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

    pub fn tail(&self) -> impl Iterator<Item = (&Code<Operator>, &Element)> {
        let mut tail = Vec::new();
        let mut node = self;
        while let Self::Binary { left: _, right, operator } = node {
            tail.push((operator.clone(), right.head()));
            node = right;
        }
        tail.into_iter()
    }

    pub fn span(&self) -> Span {
        self.tail().fold(self.head().span(), |acc, (op, unary)| acc.plus(&op.span).plus(&unary.span()))
    }
}