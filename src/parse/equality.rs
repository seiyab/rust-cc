use sourcecode::Position;
use sourcecode::Findable;

use token::Operator;
use token::Token;
use token::TokenReader;

use parse::Relational;

pub struct Equality {
    head: Relational,
    tail: Vec<(Findable<Operator>, Relational)>,
}

impl Equality {
    pub fn parse(mut token_reader: &mut TokenReader)
    -> Result<Equality, (Option<Position>, String)> {
        let mut equality = match Relational::parse(&mut token_reader) {
            Ok(first_relational) => Equality {
                head: first_relational,
                tail: Vec::new(),
            },
            Err(e) => return Err(e),
        };
        while let Some(findable_token) = token_reader.peek() {
            let token = findable_token.value();
            let eq_ops = Equality::operators();
            let operator = match token {
                &Token::Operator(op) if eq_ops.contains(&op) => findable_token.map(|_| op),
                _ => break,
            };
            token_reader.skip();
            let relational = match Relational::parse(&mut token_reader) {
                Ok(relational) => relational,
                Err(e) => return Err(e),
            };
            equality.tail.push((operator, relational));
        }
        Ok(equality)
    }

    pub fn head(&self) -> &Relational {
        &self.head
    }

    pub fn tail(&self) -> &Vec<(Findable<Operator>, Relational)> {
        &self.tail
    }

    fn operators() -> Vec<Operator> {
        vec![
            Operator::Equal,
            Operator::NotEqual,
        ]
    }
}
