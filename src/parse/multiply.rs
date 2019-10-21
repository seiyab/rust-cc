use sourcecode::Position;
use sourcecode::Findable;

use token::Operator;
use token::Token;
use token::TokenReader;

use parse::Unary;

pub struct Multiply {
    pub head: Unary,
    pub tail: Vec<(Findable<Operator>, Unary)>,
}

impl Multiply {
    pub fn parse(mut token_reader: &mut TokenReader)
    -> Result<Multiply, (Option<Position>, String)> {
        let mut multiply = match Unary::parse(&mut token_reader) {
            Ok(first_unary) => Multiply {
                head: first_unary,
                tail: Vec::new(),
            },
            Err(err) => return Err(err),
        };
        while let Some(findable_token) = token_reader.peek() {
            let token = findable_token.value();
            let operator = match token {
                &Token::Operator(Operator::Mul) => Operator::Mul,
                &Token::Operator(Operator::Div) => Operator::Div,
                _ => break,
            };
            let mul_or_div = match operator {
                Operator::Mul => Findable::new(operator, findable_token.position()),
                Operator::Div => Findable::new(operator, findable_token.position()),
                _ => break,
            };
            token_reader.skip();
            let unary = match Unary::parse(&mut token_reader) {
                Ok(unary) => unary,
                Err(err) => return Err(err),
            };
            multiply.tail.push((mul_or_div, unary));
        }
        Ok(multiply)
    }

    pub fn head(&self) -> &Unary {
        &self.head
    }

    pub fn tail(&self) -> &Vec<(Findable<Operator>, Unary)> {
        &self.tail
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert_eq!(multiply.tail[0].0.value(), &Operator::Mul);
        assert_eq!(multiply.tail[1].0.value(), &Operator::Div);
    }
}