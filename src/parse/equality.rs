use std::collections::HashSet;
use std::iter::FromIterator;

use general::TryReader;

use sourcecode::Code;
use sourcecode::Span;

use token::Operator;
use token::Token;

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
    fn parse(mut token_reader: &mut TryReader<Code<Token>>)
    -> Result<Equality, (Option<Span>, String)> {
        BinaryOperation::parse(&mut token_reader, &Self::operators())
        .map(|binary_operation| Equality{ binary_operation })
    }

    fn span(&self) -> Span {
        self.binary_operation.span()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use token::tokenize;

    #[test]
    fn test_parse_eq() {
        let src = "3 + 5 == 1";

        let tokens = tokenize(&src.to_string()).unwrap();

        let mut token_reader = TryReader::new(&tokens);

        let add = Equality::parse(&mut token_reader).unwrap();
        let mut tail = add.tail();

        assert_eq!(tail.next().unwrap().0.value, Operator::Equal);
    }

    #[test]
    fn test_parse_neq() {
        let src = "3 != 1";

        let tokens = tokenize(&src.to_string()).unwrap();

        let mut token_reader = TryReader::new(&tokens);

        let add = Equality::parse(&mut token_reader).unwrap();
        let mut tail = add.tail();

        assert_eq!(tail.next().unwrap().0.value, Operator::NotEqual);
    }
}