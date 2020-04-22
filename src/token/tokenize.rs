use std::i64;

use general::TryReader;

use sourcecode::Position;
use sourcecode::Code;
use sourcecode::Span;

use token::token::Token;
use token::token::Dictionary;

pub fn tokenize(s: &String) -> Result<Vec<Code<Token>>, Position> {
    let dictionary = Dictionary::default();
    let cs = &s.chars().collect();
    let mut reader = TryReader::new(cs);
    let mut tokens = Vec::new();
    let mut line = 0;
    let mut pos = 0;
    while reader.has_next() {
        if let Ok((consume, _)) = reader.try_(|mut r| character(&mut r, ' ')) {
            pos += consume;
            continue;
        }
        if let Ok(_) = reader.try_(|mut r| character(&mut r, '\n')) {
            pos = 0;
            line += 1;
            continue;
        }
        if let Ok((consume, n)) = reader.try_(number) {
            let span = Span::new(line, pos, consume);
            tokens.push(Code {
                value: Token::Number(n),
                span,
            });
            pos += consume;
            continue;
        }
        if let Ok((consume, w)) = reader.try_(word) {
            let span = Span::new(line, pos, consume);
            let token = match w.as_str() {
                "return" => Token::return_(),
                "let" => Token::let_(),
                wd => Token::Identifier(wd.to_string())
            };
            tokens.push(Code {
                value: token,
                span,
            });
            pos += consume;
            continue;
        }
        if let Ok((consume, t)) = reader.try_(|mut r| operator(&mut r, &dictionary)) {
            let span = Span::new(line, pos, consume);
            tokens.push(Code {
                value: t.clone(),
                span,
            });
            pos += consume;
            continue;
        }
        return Err(Position{ line, pos })
    }
    Ok(tokens)
}

fn operator(reader: &mut TryReader<char>, dict: &Dictionary) -> Result<Token, Option<Token>> {
    reader.try_(|mut r| {
        r.next()
            .map(|c| *c)
            .ok_or(dict.terminal().clone())
            .and_then(|c| match dict.dig(c) {
                Ok(d) => match operator(&mut r, d) {
                    Ok(t) => Ok(t),
                    Err(Some(t)) => Ok(t),
                    Err(None) => Err(None),
                },
                Err(Some(token)) => Err(Some(token.clone())),
                _ => Err(None)
            })
    })
    .map(|(_, t)| t)
}

fn character(reader: &mut TryReader<char>, target: char) -> Result<(), Option<()>> {
    reader.next()
        .ok_or(None)
        .and_then(|&c| if c==target { Ok(()) } else { Err(None) })
}

fn word(reader: &mut TryReader<char>) -> Result<String, Option<String>> {
    match reader.next() {
        Some(&c) if c.is_alphanumeric() => {
            match word(reader) {
                Ok(s) => Ok(String::from(format!("{}{}", c, s))),
                _ => Ok(c.to_string()),
            }
        },
        _ => Err(None),
    }
}

fn number(reader: &mut TryReader<char>) -> Result<i64, Option<i64>> {
    match reader.next() {
        None => Err(None),
        Some(c) => {
            c.to_digit(10).map(|n| n as i64).map(|n| {
                match reader.try_(number) {
                    Ok((b, m)) => n * (10 as i64).pow(b as u32) + m,
                    Err(_) => n,
                }
            }).ok_or(None)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let src = String::from("1 + 23 - 2 * 4");

        let findable_tokens = tokenize(&src).unwrap();

        assert_eq!(findable_tokens[0].value, Token::Number(1));
        assert_eq!(findable_tokens[0].span, Span::new(0, 0, 1));

        assert_eq!(findable_tokens[1].value, Token::add());
        assert_eq!(findable_tokens[1].span, Span::new(0, 2, 1));

        assert_eq!(findable_tokens[2].value, Token::Number(23));
        assert_eq!(findable_tokens[2].span, Span::new(0, 4, 2));

        assert_eq!(findable_tokens[3].value, Token::sub());
        assert_eq!(findable_tokens[3].span, Span::new(0, 7, 1));

        assert_eq!(findable_tokens[4].value, Token::Number(2));
        assert_eq!(findable_tokens[4].span, Span::new(0, 9, 1));

        assert_eq!(findable_tokens[5].value, Token::mul());
        assert_eq!(findable_tokens[5].span, Span::new(0, 11, 1));

        assert_eq!(findable_tokens[6].value, Token::Number(4));
        assert_eq!(findable_tokens[6].span, Span::new(0, 13, 1));

        assert_eq!(findable_tokens.len(), 7);
    }

    #[test]
    fn test_tokenize_relational() {
        let src = String::from("1 <= 3");

        let findable_tokens = tokenize(&src).unwrap();

        assert_eq!(findable_tokens[0].value, Token::Number(1));
        assert_eq!(findable_tokens[0].span, Span::new(0, 0, 1));

        assert_eq!(findable_tokens[1].value, Token::le());
        assert_eq!(findable_tokens[1].span, Span::new(0, 2, 2));

        assert_eq!(findable_tokens[2].value, Token::Number(3));
        assert_eq!(findable_tokens[2].span, Span::new(0, 5, 1));
    }

    #[test]
    fn test_tokenize_no_space() {
        let src = String::from("1+2");

        let findable_tokens = tokenize(&src).unwrap();

        assert_eq!(findable_tokens[0].value, Token::Number(1));
        assert_eq!(findable_tokens[0].span, Span::new(0, 0, 1));

        assert_eq!(findable_tokens[1].value, Token::add());
        assert_eq!(findable_tokens[1].span, Span::new(0, 1, 1));

        assert_eq!(findable_tokens[2].value, Token::Number(2));
        assert_eq!(findable_tokens[2].span, Span::new(0, 2, 1));
    }

    #[test]
    fn test_tokenize_number() {
        let src = "2".chars().collect();
        let mut reader = TryReader::new(&src);
        assert_eq!(number(&mut reader), Ok(2));
    }
}