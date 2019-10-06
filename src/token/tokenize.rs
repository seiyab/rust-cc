
use std::str::FromStr;
use sourcecode::Position;
use token::token::Token;

pub fn tokenize(s: &String) -> Result<Vec<TokenAndPosition>, usize> {
    let mut src = s.clone();
    let mut tokens = Vec::new();
    let src_len = src.len() as i64;
    while let Some(head) = src.chars().next() {
        let remaining = src.len() as i64;
        let position = (src_len - remaining) as usize;
        if head==' ' {
            src.drain(..1);
        } else if head=='+' || head=='-' {
            src.drain(..1);
            tokens.push(TokenAndPosition {
                token: Token::Operator(head),
                position: Position (position),
            });
        } else if head.is_digit(10) {
            let n = drain_number(&mut src).unwrap();
            tokens.push(TokenAndPosition {
                token: Token::Number(n),
                position: Position (position),
            });
        } else {
            return Err(position)
        }
    }
    Ok(tokens)
}

fn drain_number(src: &mut String) -> Result<i64, <i64 as FromStr>::Err> {
    let offset = src.find(|c: char| !c.is_digit(10)).unwrap_or(src.len());
    let digit_str: String = src.drain(..offset).collect();
    digit_str.parse::<i64>()
}

#[derive(Debug, PartialEq)]
pub struct TokenAndPosition {
    pub token: Token,
    pub position: Position,
}


pub struct TokenReader<'a> {
    tokens: &'a Vec<TokenAndPosition>,
    needle: usize,
}

impl<'a> Iterator for TokenReader<'a> {
    type Item = &'a TokenAndPosition;

    fn next(&mut self) -> Option<Self::Item> {
        if &self.needle < &self.tokens.len() {
            let ret = Some(&self.tokens[self.needle]);
            self.needle = self.needle + 1;
            ret
        } else {
            None
        }
    }
}

impl TokenReader<'_> {
    pub fn new(tokens: &Vec<TokenAndPosition>) -> TokenReader {
        TokenReader { tokens: tokens, needle: 0 }
    }

    pub fn read_number(&mut self) -> Result<i64, Option<Position>> {
        match self.next() {
            Some(token_and_position) =>
            if let Token::Number(n) = token_and_position.token { Ok(n) }
            else { Err(Some(token_and_position.position)) },
            _ => Err(None)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let src = String::from("1 + 23 - 2");

        let token_and_positions = tokenize(&src).unwrap();

        assert_eq!(token_and_positions[0].token, Token::Number(1));
        assert_eq!(token_and_positions[0].position, Position(0));
        assert_eq!(token_and_positions[1].token, Token::Operator('+'));
        assert_eq!(token_and_positions[1].position, Position(2));
        assert_eq!(token_and_positions[2].token, Token::Number(23));
        assert_eq!(token_and_positions[2].position, Position(4));
        assert_eq!(token_and_positions[3].token, Token::Operator('-'));
        assert_eq!(token_and_positions[3].position, Position(7));
        assert_eq!(token_and_positions[4].token, Token::Number(2));
        assert_eq!(token_and_positions[4].position, Position(9));
        assert_eq!(token_and_positions.len(), 5);
    }

    #[test]
    fn test_tokenize_invalid_string() {
        let src = String::from("1 + foo");

        let error = tokenize(&src);

        assert_eq!(error, Err(4));
    }
}