use std::collections::HashSet;
use std::iter::FromIterator;

use sourcecode::Position;

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

    pub fn tail(&self) -> impl Iterator<Item = (Operator, &Multiply)> {
        self.binary_operation.tail()
    }
}

impl SyntaxTree for Add {
    fn parse(mut token_reader: &mut TokenReader)
    -> Result<Add, (Option<Position>, String)> {
        let operators = HashSet::from_iter(vec![Operator::Add, Operator::Sub].into_iter());
        BinaryOperation::parse(&mut token_reader, &operators)
        .map(|binary_operation| Add {binary_operation})
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use sourcecode::Findable;
    use token::Token;

    #[test]
    fn test_parse_add() {
        // 3+5-1
        let findable_tokens = vec![
            Findable::new(Token::Number(3), Position(0)),
            Findable::new(Token::add(), Position(1)),
            Findable::new(Token::Number(5), Position(2)),
            Findable::new(Token::sub(), Position(3)),
            Findable::new(Token::Number(1), Position(4)),
        ];
        let mut token_reader = TokenReader::new(&findable_tokens);

        let add = Add::parse(&mut token_reader).unwrap();
        let mut tail = add.tail();

        assert_eq!(tail.next().unwrap().0, Operator::Add);
        assert_eq!(tail.next().unwrap().0, Operator::Sub);
    }
}