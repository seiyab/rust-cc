use general::TryReader;

use sourcecode::Code;
use sourcecode::Span;

use token::BracketSide;
use token::Bracket;
use token::Token;

use parse::SyntaxTree;
use parse::Expression;

pub enum Primary {
    Integer(Code<i64>),
    Identifier(Code<String>),
    Expression(Box<Expression>),
}

impl Primary {
    fn parse_round_bracket(token_reader: &mut TryReader<Code<Token>>)
    -> Result<Primary, (Option<Span>, String)> {
        let expression = match Expression::parse(token_reader) {
            Ok(exp) => exp,
            Err(err) => return Err(err),
        };
        let maybe_left_round_bracket = match token_reader.next() {
            Some(token) => token,
            None => return Err((None, String::from("\")\"を期待していましたが、トークンがありませんでした。"))),
        };
        match &maybe_left_round_bracket.value {
            &Token::Bracket(BracketSide::Right(Bracket::Round)) => Ok(Primary::Expression(Box::new(expression))),
            _ => Err((Some(maybe_left_round_bracket.span), String::from("\")\"を期待しています。"))),
        }
    }
}

impl SyntaxTree for Primary {
    fn parse(mut token_reader: &mut TryReader<Code<Token>>)
    -> Result<Primary, (Option<Span>, String)> {
        let token = match token_reader.next() {
            Some(token) => token,
            None => return Err((None, String::from("式を期待していましたが、トークンがありませんでした。"))),
        };
        match &token.value {
            Token::Number(number) => Ok(Primary::Integer(token.map_const(*number))),
            Token::Identifier(name) => Ok(Primary::Identifier(token.map_const(name.clone()))),
            Token::Bracket(BracketSide::Left(Bracket::Round)) => Self::parse_round_bracket(&mut token_reader),
            _ => Err((Some(token.span), String::from("数字または識別子または\"(\"を期待しています。"))),
        }
    }

    fn span(&self) -> Span {
        match self {
            Primary::Integer(c) => c.span,
            Primary::Identifier(c) => c.span,
            Primary::Expression(e) => e.span(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use token::tokenize;

    #[test]
    fn test_parse_integer() {
        // 10
        let tokens = vec![
            Code {
                value: Token::Number(10),
                span: Span::new(0, 0, 1),
            },
        ];
        let mut token_reader = TryReader::new(&tokens);

        let primary = Primary::parse(&mut token_reader).unwrap();

        if let Primary::Integer(Code { value: i, span: _ }) = primary {
            assert_eq!(i, 10);
        } else {
            panic!("数字になっていません。")
        }
    }

    #[test]
    fn test_parse_round_bracket() {
        let src = "(4+3)";

        let tokens = tokenize(&src.to_string()).unwrap();

        let mut token_reader = TryReader::new(&tokens);

        let primary = Primary::parse(&mut token_reader).unwrap();

        if let Primary::Expression(_expression) = primary {
        } else {
            panic!("Expressionになっていません。")
        }
    }
}