use std::collections::HashSet;
use std::iter::FromIterator;

use sourcecode::Position;

use token::Operator;
use token::TokenReader;

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

    pub fn tail(&self) -> impl Iterator<Item = (Operator, &Add)> {
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
    fn parse(mut token_reader: &mut TokenReader)
    -> Result<Relational, (Option<Position>, String)> {
        BinaryOperation::parse(&mut token_reader, &Self::operators())
        .map(|binary_operation| Relational {binary_operation})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sourcecode::Findable;
    use token::Token;

    #[test]
    fn test_parse_relational() {
        // 3 < 5 <= 1
        let findable_tokens = vec![
            Findable::new(Token::Number(3), Position(0)),
            Findable::new(Token::lt(), Position(1)),
            Findable::new(Token::Number(5), Position(2)),
            Findable::new(Token::le(), Position(3)),
            Findable::new(Token::Number(1), Position(4)),
        ];
        let mut token_reader = TokenReader::new(&findable_tokens);

        let relational = Relational::parse(&mut token_reader).unwrap();
        let mut tail = relational.tail();

        assert_eq!(tail.next().unwrap().0, Operator::Less);
        assert_eq!(tail.next().unwrap().0, Operator::LessEq);
    }
}