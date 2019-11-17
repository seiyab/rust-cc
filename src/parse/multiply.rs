use std::collections::HashSet;
use std::iter::FromIterator;

use sourcecode::Position;

use token::Operator;
use token::TokenReader;

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

    pub fn tail(&self) -> impl Iterator<Item = (Operator, &Unary)> {
        self.binary_operation.tail()
    }
}

impl SyntaxTree for Multiply {
    fn parse(mut token_reader: &mut TokenReader)
    -> Result<Multiply, (Option<Position>, String)> {
        let operators = HashSet::from_iter(vec![Operator::Mul, Operator::Div].into_iter());
        BinaryOperation::parse(&mut token_reader, &operators)
        .map(|binary_operation| Multiply {binary_operation})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sourcecode::Findable;
    use token::Token;

    #[test]
    fn test_parse_multiply() {
        // 3*5/1
        let findable_tokens = vec![
            Findable::new(Token::Number(3), Position(0)),
            Findable::new(Token::mul(), Position(1)),
            Findable::new(Token::Number(5), Position(2)),
            Findable::new(Token::div(), Position(3)),
            Findable::new(Token::Number(1), Position(4)),
        ];
        let mut token_reader = TokenReader::new(&findable_tokens);

        let multiply = Multiply::parse(&mut token_reader).unwrap();
        let mut tail = multiply.tail();

        assert_eq!(tail.next().unwrap().0, Operator::Mul);
        assert_eq!(tail.next().unwrap().0, Operator::Div);
    }
}