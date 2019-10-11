use sourcecode::Position;
use sourcecode::Findable;

use token::Operator;
use token::Token;
use token::TokenReader;

pub struct Multiply {
    pub head: Findable<i64>,
    pub tail: Vec<(Findable<Operator>, Findable<i64>)>,
}

impl Multiply {
    pub fn parse(token_reader: &mut TokenReader)
    -> Result<Multiply, (Option<Position>, String)> {
        let first_findable_token = match token_reader.next() {
            Some(findable_token) => findable_token,
            None => return Err((None, String::from("式を期待していましたが、トークンがありませんでした。"))),
        };
        let first_number = match first_findable_token.value() {
            &Token::Number(n) => n,
            _ => return Err((Some(first_findable_token.position()), String::from("数ではありません。"))),
        };
        let mut multiply = Multiply {
            head: first_findable_token.map(|_| first_number),
            tail: Vec::new(),
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
            let number = if let Some(findable_token) = token_reader.peek() {
                match findable_token.value() {
                    &Token::Number(number) => Findable::new(number, findable_token.position()),
                    _ => return Err((Some(findable_token.position()), String::from("数ではありません。"))),
                }
            } else {
                return Err((None, String::from("数を期待していましたが、トークンがありませんでした。")));
            };
            token_reader.skip();
            multiply.tail.push((mul_or_div, number));
        }
        Ok(multiply)
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

        assert_eq!(multiply.head.value(), &3);

        assert_eq!(multiply.tail[0].0.value(), &Operator::Mul);
        assert_eq!(multiply.tail[0].1.value(), &5);

        assert_eq!(multiply.tail[1].0.value(), &Operator::Div);
        assert_eq!(multiply.tail[1].1.value(), &1);
    }
}