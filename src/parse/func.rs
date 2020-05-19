use general::TryReader;

use sourcecode::Span;
use sourcecode::Code;

use token::Token;
use token::Bracket;
use token::BracketSide;
use token::ReservedWord;

use parse::SyntaxTree;
use parse::Expression;

pub struct Func {
    pub name: String,
    pub args: Vec<String>,
    pub body: Expression,
    span: Span,
}

impl SyntaxTree for Func {
    fn parse(token_reader: &mut TryReader<Code<Token>>) -> Result<Self, (Option<Span>, String)> {
        let start = if let Some(token) = token_reader.next() {
            match &token.value {
                Token::ReservedWord(ReservedWord::Func) => token.span.start,
                _ => return Err((Some(token.span), "funcを期待していました".to_string()))
            }
        } else {
            return Err((None, "funcを期待していました".to_string()));
        };

        let name = if let Some(token) = token_reader.next() {
            match &token.value {
                Token::Identifier(name) => name.clone(),
                _ => return Err((Some(token.span), "識別子を期待していました".to_string()))
            }
        } else {
            return Err((None, "識別子を期待していました".to_string()));
        };

        if let Some(token) = token_reader.next() {
            match &token.value {
                Token::Bracket(BracketSide::Left(Bracket::Round)) => name.clone(),
                _ => return Err((Some(token.span), "(を期待していました".to_string()))
            }
        } else {
            return Err((None, "(を期待していました".to_string()));
        };

        let mut args = Vec::new();
        for _ in 0..6 {
            let next = token_reader.try_next(|token| match &token.value {
                Token::Identifier(name) => Ok(name.clone()),
                _ => Err(()),
            });
            let arg = if let Ok(name) = next {
                name
            } else {
                break;
            };
            args.push(arg);
            if let Err(_) = token_reader.try_(
                |reader| match reader.next().map(|c| &c.value) {
                    Some(Token::Comma) => Ok(()),
                    _ => Err(()),
                }
            ) {
                break;
            };
        }

        if let Some(token) = token_reader.next() {
            match &token.value {
                Token::Bracket(BracketSide::Right(Bracket::Round)) => name.clone(),
                _ => return Err((Some(token.span), ")を期待していました".to_string()))
            }
        } else {
            return Err((None, ")を期待していました".to_string()));
        };

        let body = match Expression::parse(token_reader) {
            Ok(expr) => expr,
            Err(e) => return Err(e),
        };

        let span = Span {
            start: start,
            end: body.span().end
        };

        Ok(Self {
            name,
            args,
            body,
            span,
        })
    }

    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use token::tokenize;

    #[test]
    fn test_parse() {
        let src = "func foo(a, b) {
            a + b
        }";
        let tokens = tokenize(&src.to_string()).unwrap();
        let mut token_reader = TryReader::new(&tokens);

        let func = Func::parse(&mut token_reader).unwrap();

        assert_eq!(func.name, "foo".to_string());
        assert_eq!(func.args.len(), 2);
        assert_eq!(func.args[0], "a".to_string());
        assert_eq!(func.args[1], "b".to_string());
    }

    #[test]
    fn test_parse_zero_arg() {
        let src = "func main() 0";
        let tokens = tokenize(&src.to_string()).unwrap();
        let mut token_reader = TryReader::new(&tokens);

        let func = Func::parse(&mut token_reader).unwrap();

        assert_eq!(func.name, "main".to_string());
        assert_eq!(func.args.len(), 0);
    }
}