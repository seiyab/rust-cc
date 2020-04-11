use std::collections::HashSet;
use std::iter::FromIterator;

use sourcecode::Position;
use sourcecode::Span;
use sourcecode::Code;

use token::TokenReader;
use token::Operator;

use parse::SyntaxTree;
use parse::BinaryOperation;
use parse::Multiply;

pub struct Add {
    binary_operation: BinaryOperation<Multiply>,
}

impl Add {
    pub fn head(&self) -> &Multiply {
        self.binary_operation.head()
    }

    pub fn tail(&self) -> impl Iterator<Item = (&Code<Operator>, &Multiply)> {
        self.binary_operation.tail()
    }
}

impl SyntaxTree for Add {
    fn parse(mut token_reader: &mut TokenReader)
    -> Result<Add, (Option<Span>, String)> {
        let operators = HashSet::from_iter(vec![Operator::Add, Operator::Sub].into_iter());
        BinaryOperation::parse(&mut token_reader, &operators)
        .map(|binary_operation| Add {binary_operation})
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
    fn test_parse_add() {
        let src = "3 + 5 - 1";

        let tokens = tokenize(&src.to_string()).unwrap();

        let mut token_reader = TokenReader::new(&tokens);

        let add = Add::parse(&mut token_reader).unwrap();
        let mut tail = add.tail();

        assert_eq!(tail.next().unwrap().0.value, Operator::Add);
        assert_eq!(tail.next().unwrap().0.value, Operator::Sub);
    }
}