use sourcecode::Position;
use sourcecode::Findable;

use token::Token;
use token::TokenReader;
use token::Operator;

use parse::Multiply;

pub struct Expression {
    head: Multiply,
    tail: Vec<(Findable<Operator>, Multiply)>,
}

impl Expression {
    pub fn parse(mut token_reader: &mut TokenReader)
    -> Result<Expression, (Option<Position>, String)> {
        let mut expression = match Multiply::parse(&mut token_reader) {
            Ok(first_multiply) => Expression {
                head: first_multiply,
                tail: Vec::new(),
            },
            Err(err) => return Err(err),
        };
        while let Some(findable_token) = token_reader.peek() {
            let token = findable_token.value();
            let add_or_sub = match token {
                &Token::Operator(Operator::Add) => findable_token.map(|_| Operator::Add),
                &Token::Operator(Operator::Sub) => findable_token.map(|_| Operator::Sub),
                _ => break,
            };
            token_reader.skip();
            let multiply = match Multiply::parse(&mut token_reader) {
                Ok(multiply) => multiply,
                Err(err) => return Err(err),
            };
            expression.tail.push((add_or_sub, multiply));
        }
        Ok(expression)
    }

    pub fn head(&self) -> &Multiply {
        &self.head
    }

    pub fn tail(&self) -> &Vec<(Findable<Operator>, Multiply)> {
        &self.tail
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_expression() {
        // 3+5-1
        let findable_tokens = vec![
            Findable::new(Token::Number(3), Position(0)),
            Findable::new(Token::add(), Position(1)),
            Findable::new(Token::Number(5), Position(2)),
            Findable::new(Token::sub(), Position(3)),
            Findable::new(Token::Number(1), Position(4)),
        ];
        let mut token_reader = TokenReader::new(&findable_tokens);

        let expression = Expression::parse(&mut token_reader).unwrap();

        assert_eq!(expression.tail[0].0.value(), &Operator::Add);
        assert_eq!(expression.tail[1].0.value(), &Operator::Sub);
    }
}