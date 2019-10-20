use sourcecode::Position;

use token::BracketSide;
use token::Bracket;
use token::Token;
use token::TokenReader;

use parse::Expression;

pub enum Primary {
    Integer(i64),
    Expression(Box<Expression>),
}

impl Primary {
    pub fn parse(mut token_reader: &mut TokenReader)
    -> Result<Primary, (Option<Position>, String)> {
        let first_findable_token = match token_reader.next() {
            Some(findable_token) => findable_token,
            None => return Err((None, String::from("式を期待していましたが、トークンがありませんでした。"))),
        };
        match first_findable_token.value() {
            &Token::Number(number) => Ok(Primary::Integer(number)),
            &Token::Bracket(BracketSide::Left(Bracket::Round)) => Self::parse_round_bracket(&mut token_reader),
            _ => Err((Some(first_findable_token.position()), String::from("数字または\"(\"を期待しています。"))),
        }
    }

    fn parse_round_bracket(token_reader: &mut TokenReader)
    -> Result<Primary, (Option<Position>, String)> {
        let expression = match Expression::parse(token_reader) {
            Ok(exp) => exp,
            Err(err) => return Err(err),
        };
        let maybe_left_round_bracket = match token_reader.next() {
            Some(findable_token) => findable_token,
            None => return Err((None, String::from("\")\"を期待していましたが、トークンがありませんでした。"))),
        };
        match &maybe_left_round_bracket.value() {
            &Token::Bracket(BracketSide::Right(Bracket::Round)) => Ok(Primary::Expression(Box::new(expression))),
            _ => Err((Some(maybe_left_round_bracket.position()), String::from("\")\"を期待しています。"))),
        }
    }
}

#[cfg(test)]
mod tests {
    use sourcecode::Findable;
    use super::*;

    #[test]
    fn test_parse_integer() {
        // 10
        let findable_tokens = vec![
            Findable::new(Token::Number(10), Position(0)),
        ];
        let mut token_reader = TokenReader::new(&findable_tokens);

        let primary = Primary::parse(&mut token_reader).unwrap();

        if let Primary::Integer(i) = primary {
            assert_eq!(i, 10);
        } else {
            panic!("数字になっていません。")
        }
    }

    #[test]
    fn test_parse_round_bracket() {
        // (4+3)
        let findable_tokens = vec![
            Findable::new(Token::left_round_bracket(), Position(0)),
            Findable::new(Token::Number(4), Position(1)),
            Findable::new(Token::add(), Position(2)),
            Findable::new(Token::Number(3), Position(3)),
            Findable::new(Token::right_round_bracket(), Position(4)),
        ];
        let mut token_reader = TokenReader::new(&findable_tokens);

        let primary = Primary::parse(&mut token_reader).unwrap();

        if let Primary::Expression(expression) = primary {
            assert_eq!(expression.tail().len(), 1);
        } else {
            panic!("Expressionになっていません。")
        }
    }
}