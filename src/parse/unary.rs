use general::SemiGroup;

use sourcecode::Span;

use token::TokenReader;
use token::Token;
use token::Operator;

use parse::SyntaxTree;
use parse::Primary;

pub enum Unary {
    Positive(Primary, Span),
    Negative(Primary, Span),
}

impl SyntaxTree for Unary {
    fn parse(token_reader: &mut TokenReader)
    -> Result<Unary, (Option<Span>, String)> {
        let operator = token_reader.try_(|reader| {
            let maybe_token = reader.next();
            match maybe_token {
                None => Err(()),
                Some(token) => {
                    match token.value {
                        Token::Operator(Operator::Add) => Ok((Operator::Add, token.span)),
                        Token::Operator(Operator::Sub) => Ok((Operator::Sub, token.span)),
                        _ => Err(()),
                    }
                }
            }
        });
        match operator {
            Ok((Operator::Add, span)) => Primary::parse(token_reader).map(|primary| {
                let s = span.plus(&primary.span());
                Unary::Positive(primary, s)
            }),
            Ok((Operator::Sub, span)) => Primary::parse(token_reader).map(|primary| {
                let s = span.plus(&primary.span());
                Unary::Negative(primary, s)
            }),
            _ =>  Primary::parse(token_reader).map(|primary| {
                let span = primary.span();
                Unary::Positive(primary, span)
            }),
        }
    }

    fn span(&self) -> Span {
        match self {
            Unary::Positive(_, span) => span.clone(),
            Unary::Negative(_, span) => span.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use token::tokenize;

    #[test]
    fn test_parse_positive() {
        let src = "+3";
        let tokens = tokenize(&src.to_string()).unwrap();
        let mut token_reader = TokenReader::new(&tokens);

        let unary = Unary::parse(&mut token_reader).unwrap();

        if let Unary::Positive(_, _) = unary {
        } else {
            panic!("正になっていません。")
        }
    }

    #[test]
    fn test_parse_implicit_positive() {
        // 6
        let src = "6";
        let tokens = tokenize(&src.to_string()).unwrap();
        let mut token_reader = TokenReader::new(&tokens);

        let unary = Unary::parse(&mut token_reader).unwrap();

        if let Unary::Positive(_, _) = unary {
        } else {
            panic!("正になっていません。")
        }
    }

    #[test]
    fn test_parse_negative() {
        let src = "-5";
        let tokens = tokenize(&src.to_string()).unwrap();
        let mut token_reader = TokenReader::new(&tokens);

        let unary = Unary::parse(&mut token_reader).unwrap();

        if let Unary::Negative(_, _) = unary {
        } else {
            panic!("正になっています。")
        }
    }
}