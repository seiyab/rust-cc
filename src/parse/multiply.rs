use std::collections::HashSet;
use std::iter::FromIterator;

use general::TryReader;

use sourcecode::Code;
use sourcecode::Span;

use token::Token;
use token::Operator;

use parse::SyntaxTree;
use parse::BinaryOperation;
use parse::Unary;

pub struct Multiply {
    binary_operation: BinaryOperation<Unary>,
}

impl Multiply {
    pub fn head(&self) -> &Unary {
        self.binary_operation.head()
    }

    pub fn tail(&self) -> impl Iterator<Item = (&Code<Operator>, &Unary)> {
        self.binary_operation.tail()
    }
}

impl SyntaxTree for Multiply {
    fn parse(mut token_reader: &mut TryReader<Code<Token>>)
    -> Result<Multiply, (Option<Span>, String)> {
        let operators = HashSet::from_iter(vec![Operator::Mul, Operator::Div].into_iter());
        BinaryOperation::parse(&mut token_reader, &operators)
        .map(|binary_operation| Multiply {binary_operation})
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
    fn test_parse_multiply() {
        let src = "3 * 5 / 1";

        let tokens = tokenize(&src.to_string()).unwrap();

        let mut token_reader = TryReader::new(&tokens);

        let multiply = Multiply::parse(&mut token_reader).unwrap();
        let mut tail = multiply.tail();

        assert_eq!(tail.next().unwrap().0.value, Operator::Mul);
        assert_eq!(tail.next().unwrap().0.value, Operator::Div);
    }
}