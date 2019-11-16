use sourcecode::Position;
use sourcecode::Findable;

use token::Operator;
use token::Token;
use token::TokenReader;

use parse::SyntaxTree;
use parse::Add;

pub struct Relational {
    pub head: Add,
    pub tail: Vec<(Findable<Operator>, Add)>,
}

impl Relational {
    pub fn head(&self) -> &Add {
        &self.head
    }

    pub fn tail(&self) -> &Vec<(Findable<Operator>, Add)> {
        &self.tail
    }

    fn operators() -> Vec<Operator> {
        vec![
            Operator::Less,
            Operator::LessEq,
            Operator::Greater,
            Operator::GreaterEq,
        ]
    }
}

impl SyntaxTree for Relational {
    fn parse(mut token_reader: &mut TokenReader)
    -> Result<Relational, (Option<Position>, String)> {
        let mut relational = match Add::parse(&mut token_reader) {
            Ok(first_add) => Relational {
                head: first_add,
                tail: Vec::new(),
            },
            Err(e) => return Err(e),
        };
        while let Some(findable_token) = token_reader.peek() {
            let token = findable_token.value();
            let rel_ops = Relational::operators();
            let operator = match token {
                &Token::Operator(op) if rel_ops.contains(&op) => findable_token.map(|_| op),
                _ => break,
            };
            token_reader.skip();
            let add = match Add::parse(&mut token_reader) {
                Ok(add) => add,
                Err(e) => return Err(e),
            };
            relational.tail.push((operator, add));
        }
        Ok(relational)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert_eq!(relational.tail[0].0.value(), &Operator::Less);
        assert_eq!(relational.tail[1].0.value(), &Operator::LessEq);
    }
}