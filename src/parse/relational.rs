use std::collections::HashSet;
use std::iter::FromIterator;

use general::TryReader;

use sourcecode::Code;
use sourcecode::Span;

use token::Operator;
use token::Token;

use parse::SyntaxTree;
use parse::BinaryOperation;
use parse::Add;

pub struct Relational {
    binary_operation: BinaryOperation<Add>,
}

impl Relational {
    pub fn head(&self) -> &Add {
        self.binary_operation.head()
    }

    pub fn tail(&self) -> impl Iterator<Item = (&Code<Operator>, &Add)> {
        self.binary_operation.tail()
    }

    fn operators() -> HashSet<Operator> {
        HashSet::from_iter(vec![
            Operator::Less,
            Operator::LessEq,
            Operator::Greater,
            Operator::GreaterEq,
        ].into_iter())
    }
}

impl SyntaxTree for Relational {
    fn parse(mut token_reader: &mut TryReader<Code<Token>>)
    -> Result<Relational, (Option<Span>, String)> {
        BinaryOperation::parse(&mut token_reader, &Self::operators())
        .map(|binary_operation| Relational {binary_operation})
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
    fn test_parse_relational() {
        let src = "3 < 5 <= 1";
        let tokens = tokenize(&src.to_string()).unwrap();
        let mut token_reader = TryReader::new(&tokens);

        let relational = Relational::parse(&mut token_reader).unwrap();
        let mut tail = relational.tail();

        assert_eq!(tail.next().unwrap().0.value, Operator::Less);
        assert_eq!(tail.next().unwrap().0.value, Operator::LessEq);
    }
}