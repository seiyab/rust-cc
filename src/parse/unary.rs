use sourcecode::Position;

use token::TokenReader;
use token::Token;
use token::Operator;

use parse::SyntaxTree;
use parse::Primary;

pub enum Unary {
    Positive(Primary),
    Negative(Primary),
}

impl SyntaxTree for Unary {
    fn parse(token_reader: &mut TokenReader)
    -> Result<Unary, (Option<Position>, String)> {
        match token_reader.peek().map(|findable| findable.value()) {
            Some(Token::Operator(Operator::Add)) => {
                token_reader.skip();
                Primary::parse(token_reader).map(Unary::Positive)
            },
            Some(Token::Operator(Operator::Sub)) => {
                token_reader.skip();
                Primary::parse(token_reader).map(Unary::Negative)
            },
            Some(_) => Primary::parse(token_reader).map(Unary::Positive),
            None => return Err((None, String::from("\"+\"、\"-\"、または式を期待していましたが、トークンがありませんでした。"))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sourcecode::Findable;

    #[test]
    fn test_parse_positive() {
        // +3
        let findable_tokens = vec![
            Findable::new(Token::add(), Position(0)),
            Findable::new(Token::Number(3), Position(1)),
        ];
        let mut token_reader = TokenReader::new(&findable_tokens);

        let unary = Unary::parse(&mut token_reader).unwrap();

        if let Unary::Positive(_) = unary {
        } else {
            panic!("正になっていません。")
        }
    }

    #[test]
    fn test_parse_implicit_positive() {
        // 6
        let findable_tokens = vec![
            Findable::new(Token::Number(6), Position(0)),
        ];
        let mut token_reader = TokenReader::new(&findable_tokens);

        let unary = Unary::parse(&mut token_reader).unwrap();

        if let Unary::Positive(_) = unary {
        } else {
            panic!("正になっていません。")
        }
    }

    #[test]
    fn test_parse_negative() {
        // -5
        let findable_tokens = vec![
            Findable::new(Token::sub(), Position(0)),
            Findable::new(Token::Number(5), Position(1)),
        ];
        let mut token_reader = TokenReader::new(&findable_tokens);

        let unary = Unary::parse(&mut token_reader).unwrap();

        if let Unary::Negative(_) = unary {
        } else {
            panic!("正になっています。")
        }
    }
}