use general::TryReader;

use sourcecode::Code;
use sourcecode::Span;
use sourcecode::Position;

use token::BracketSide;
use token::Bracket;
use token::Token;

use parse::SyntaxTree;
use parse::Expression;

pub enum Primary {
    Integer(Code<i64>),
    Identifier(Code<String>),
    Expression(Box<Expression>),
    FnCall(FnCall),
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
        match token_reader.try_(FnCall::parse) {
            Ok((_, fn_call)) => return Ok(Self::FnCall(fn_call)),
            _ => ()
        }
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
            Primary::FnCall(f) => f.span(),
        }
    }
}

pub struct FnCall {
    pub func: Code<String>,
    pub args: Vec<Expression>,
    end_pos: Position,
}

impl SyntaxTree for FnCall {
    fn parse(token_reader: &mut TryReader<Code<Token>>)
    -> Result<Self, (Option<Span>, String)> {
        let func = match token_reader.next() {
            Some(token) => {
                match &token.value {
                    Token::Identifier(idfr) => token.map_const(idfr.clone()),
                    _ => return Err((Some(token.span), "識別子を期待していました".to_string())),
                }
            },
            None => return Err((None, "識別子を期待していました".to_string()))
        };
        match token_reader.next().map(|t| t.value.clone()) {
            Some(Token::Bracket(BracketSide::Left(Bracket::Round))) => (),
            _ => return Err((None, "(を期待していました。".to_string())),
        };
        let mut args = Vec::new();
        for _ in 0..6 {
            let arg = token_reader.try_(Expression::parse);
            if let Ok((_, expr)) = arg {
                args.push(expr);

                if let Err(_) = token_reader.try_next(
                    |token| if token.value == Token::Comma { Ok(()) } else { Err(()) }
                ) {
                    break;
                }
            } else {
                break;
            }
        }
        let end_pos = match token_reader.next() {
            Some(token) => {
                match token.value {
                    Token::Bracket(BracketSide::Right(Bracket::Round)) => token.span.start,
                    _ => return Err((Some(token.span), ")を期待していました".to_string()))
                }
            },
            _ => return Err((None, ")を期待していました".to_string()))
        };
        Ok(Self {
            func,
            args,
            end_pos,
        })
    }

    fn span(&self) -> Span {
        Span {
            start: self.func.span.start,
            end: self.end_pos,
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

    #[test]
    fn test_parse_fn_call() {
        let src = "foo(1, 2,)";

        let tokens = tokenize(&src.to_string()).unwrap();

        let mut token_reader = TryReader::new(&tokens);

        let fn_call = FnCall::parse(&mut token_reader).unwrap();

        assert_eq!(fn_call.func.value, "foo".to_string());
        assert_eq!(fn_call.args.len(), 2);

        let src = "bar(1, 2)";

        let tokens = tokenize(&src.to_string()).unwrap();

        let mut token_reader = TryReader::new(&tokens);

        let fn_call = FnCall::parse(&mut token_reader).unwrap();

        assert_eq!(fn_call.func.value, "bar".to_string());
        assert_eq!(fn_call.args.len(), 2);
    }
}