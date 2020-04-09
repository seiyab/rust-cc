use std::collections::HashSet;
use std::iter::FromIterator;

use sourcecode::Code;
use sourcecode::Span;

use token::Operator;
use token::TokenReader;

use parse::SyntaxTree;
use parse::BinaryOperation;
use parse::Relational;

pub struct Equality {
    binary_operation: BinaryOperation<Relational>,
}

impl Equality {
    pub fn head(&self) -> &Relational {
        self.binary_operation.head()
    }

    pub fn tail(&self) -> impl Iterator<Item = (&Code<Operator>, &Relational)> {
        self.binary_operation.tail()
    }

    fn operators() -> HashSet<Operator> {
        HashSet::from_iter(vec![
            Operator::Equal,
            Operator::NotEqual,
        ].into_iter())
    }
}

impl SyntaxTree for Equality {
    fn parse(mut token_reader: &mut TokenReader)
    -> Result<Equality, (Option<Span>, String)> {
        BinaryOperation::parse(&mut token_reader, &Self::operators())
        .map(|binary_operation| Equality{ binary_operation })
    }

    fn span(&self) -> Span {
        self.binary_operation.span()
    }
}