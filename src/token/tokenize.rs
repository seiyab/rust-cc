use std::i64;

use general::TryReader;

use sourcecode::Position;
use sourcecode::Findable;
use token::token::Token;
use token::token::Operator;
use token::token::ReservedWord;
use token::token::Dictionary;

pub fn tokenize(s: &String) -> Result<Vec<Findable<Token>>, Position> {
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
        if let Ok((consume, _)) = reader.try_(|mut r| character(&mut r, '\n')) {
            pos += consume;
            line += 1;
            continue;
        }
        if let Ok((consume, n)) = reader.try_(number) {
            tokens.push(Findable::new(
                Token::Number(n),
                Position::new(line, pos)
            ));
            pos += consume;
            continue;
        }
        if let Ok((consume, w)) = reader.try_(word) {
            let token = match w.as_str() {
                "return" => Token::return_(),
                "let" => Token::let_(),
                wd => Token::Identifier(wd.to_string())
            };
            tokens.push(Findable::new(
                token,
                Position::new(line, pos)
            ));
            pos += consume;
            continue;
        }
        if let Ok((consume, t)) = reader.try_(|mut r| operator(&mut r, &dictionary)) {
            tokens.push(Findable::new(
                t.clone(),
                Position::new(line, pos)
            ));
            pos += consume;
            continue;
        }
        return Err(Position::new(line, pos))
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

pub struct TokenReader<'a> {
    tokens: &'a Vec<Findable<Token>>,
    needle: usize,
}

impl<'a> Iterator for TokenReader<'a> {
    type Item = &'a Findable<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_next() {
            let ret = Some(&self.tokens[self.needle]);
            self.needle = self.needle + 1;
            ret
        } else {
            None
        }
    }
}

impl TokenReader<'_> {
    pub fn new(tokens: &Vec<Findable<Token>>) -> TokenReader {
        TokenReader { tokens: tokens, needle: 0 }
    }

    pub fn has_next(&self) -> bool {
        return self.needle < self.tokens.len()
    }

    pub fn peek(&self) -> Option<&Findable<Token>> {
        if &self.needle < &self.tokens.len() {
            Some(&self.tokens[self.needle])
        } else {
            None
        }
    }

    pub fn skip(&mut self) {
        self.needle += 1;
    }

    pub fn consume_reserved_word(&mut self, expected_word: ReservedWord)
    -> Result<ReservedWord, Option<Position>> {
        self.peek().ok_or(None)
        .map(|findable| findable.map(|token| token.clone()))
        .and_then(|token| match token.value() {
            Token::ReservedWord(actual_word) =>
                if *actual_word==expected_word { self.skip(); Ok(*actual_word) }
                else { Err(Some(token.position())) }
            _ => Err(None),
        })
    }

    pub fn consume_identifier(&mut self)
    -> Result<String, Option<Position>> {
        self.peek().ok_or(None)
        .map(|findable| findable.map(|token| token.clone()))
        .and_then(|token| match token.value() {
            Token::Identifier(name) => { self.skip(); Ok(name.clone()) },
            _ => Err(Some(token.position())),
        })
    }

    pub fn consume_operator(&mut self, expected_operator: Operator)
    -> Result<Operator, Option<Position>> {
        self.peek().ok_or(None)
        .map(|findable| findable.map(|token| token.clone()))
        .and_then(|token| match token.value() {
            Token::Operator(actual_operator) => 
                if *actual_operator == expected_operator { self.skip(); Ok(*actual_operator) }
                else { Err(Some(token.position())) },
            _ => Err(Some(token.position())),
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let src = String::from("1 + 23 - 2 * 4");

        let findable_tokens = tokenize(&src).unwrap();

        assert_eq!(findable_tokens[0].value(), &Token::Number(1));
        assert_eq!(findable_tokens[0].position(), Position::new(0, 0));

        assert_eq!(findable_tokens[1].value(), &Token::add());
        assert_eq!(findable_tokens[1].position(), Position::new(0, 2));

        assert_eq!(findable_tokens[2].value(), &Token::Number(23));
        assert_eq!(findable_tokens[2].position(), Position::new(0, 4));

        assert_eq!(findable_tokens[3].value(), &Token::sub());
        assert_eq!(findable_tokens[3].position(), Position::new(0, 7));

        assert_eq!(findable_tokens[4].value(), &Token::Number(2));
        assert_eq!(findable_tokens[4].position(), Position::new(0, 9));

        assert_eq!(findable_tokens[5].value(), &Token::mul());
        assert_eq!(findable_tokens[5].position(), Position::new(0, 11));

        assert_eq!(findable_tokens[6].value(), &Token::Number(4));
        assert_eq!(findable_tokens[6].position(), Position::new(0, 13));

        assert_eq!(findable_tokens.len(), 7);
    }

    #[test]
    fn test_tokenize_relational() {
        let src = String::from("1 <= 3");

        let findable_tokens = tokenize(&src).unwrap();

        assert_eq!(findable_tokens[0].value(), &Token::Number(1));
        assert_eq!(findable_tokens[0].position(), Position::new(0, 0));

        assert_eq!(findable_tokens[1].value(), &Token::le());
        assert_eq!(findable_tokens[1].position(), Position::new(0, 2));

        assert_eq!(findable_tokens[2].value(), &Token::Number(3));
        assert_eq!(findable_tokens[2].position(), Position::new(0, 5));
    }

    #[test]
    fn test_tokenize_no_space() {
        let src = String::from("1+2");

        let findable_tokens = tokenize(&src).unwrap();

        assert_eq!(findable_tokens[0].value(), &Token::Number(1));
        assert_eq!(findable_tokens[0].position(), Position::new(0, 0));

        assert_eq!(findable_tokens[1].value(), &Token::add());
        assert_eq!(findable_tokens[1].position(), Position::new(0, 1));

        assert_eq!(findable_tokens[2].value(), &Token::Number(2));
        assert_eq!(findable_tokens[2].position(), Position::new(0, 2));
    }

    #[test]
    fn test_tokenize_number() {
        let src = "2".chars().collect();
        let mut reader = TryReader::new(&src);
        assert_eq!(number(&mut reader), Ok(2));
    }
}