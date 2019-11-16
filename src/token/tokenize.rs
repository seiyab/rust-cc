use std::str::FromStr;
use sourcecode::Position;
use sourcecode::Findable;
use token::token::Token;

pub fn tokenize(s: &String) -> Result<Vec<Findable<Token>>, usize> {
    let mut src = s.clone();
    let mut tokens = Vec::new();
    let src_len = src.len() as i64;
    while let Some(head) = src.chars().next() {
        let remaining = src.len() as i64;
        let position = (src_len - remaining) as usize;
        if head==' ' {
            src.drain(..1);
        } else if head.is_digit(10) {
            let n = drain_number(&mut src).unwrap();
            tokens.push(Findable::new(
                Token::Number(n),
                Position(position)
            ));
        } else {
            src.drain(..1);
            let token = match head {
                '+' => Token::add(),
                '-' => Token::sub(),
                '*' => Token::mul(),
                '/' => Token::div(),
                '(' => Token::left_round_bracket(),
                ')' => Token::right_round_bracket(),
                '<' => {
                    if src.chars().next() == Some('=') {
                        src.drain(..1);
                        Token::le()
                    } else {
                        Token::lt()
                    }
                },
                '>' => {
                    if src.chars().next() == Some('=') {
                        src.drain(..1);
                        Token::ge()
                    } else {
                        Token::gt()
                    }
                },
                '=' => {
                    if src.chars().next() == Some('=') {
                        src.drain(..1);
                        Token::eq()
                    } else {
                        return Err(position + 1)
                    }
                },
                '!' => {
                    if src.chars().next() == Some('=') {
                        src.drain(..1);
                        Token::neq()
                    } else {
                        return Err(position + 1)
                    }
                },
                _ => return Err(position),
            };
            tokens.push(Findable::new(
                token,
                Position(position)
            ))
        }
    }
    Ok(tokens)
}

fn drain_number(src: &mut String) -> Result<i64, <i64 as FromStr>::Err> {
    let offset = src.find(|c: char| !c.is_digit(10)).unwrap_or(src.len());
    let digit_str: String = src.drain(..offset).collect();
    digit_str.parse::<i64>()
}

pub struct TokenReader<'a> {
    tokens: &'a Vec<Findable<Token>>,
    needle: usize,
}

impl<'a> Iterator for TokenReader<'a> {
    type Item = &'a Findable<Token>;

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
    pub fn new(tokens: &Vec<Findable<Token>>) -> TokenReader {
        TokenReader { tokens: tokens, needle: 0 }
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
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let src = String::from("1 + 23 - 2 * 4");

        let findable_tokens = tokenize(&src).unwrap();

        assert_eq!(findable_tokens[0].value(), &Token::Number(1));
        assert_eq!(findable_tokens[0].position(), Position(0));

        assert_eq!(findable_tokens[1].value(), &Token::add());
        assert_eq!(findable_tokens[1].position(), Position(2));

        assert_eq!(findable_tokens[2].value(), &Token::Number(23));
        assert_eq!(findable_tokens[2].position(), Position(4));

        assert_eq!(findable_tokens[3].value(), &Token::sub());
        assert_eq!(findable_tokens[3].position(), Position(7));

        assert_eq!(findable_tokens[4].value(), &Token::Number(2));
        assert_eq!(findable_tokens[4].position(), Position(9));

        assert_eq!(findable_tokens[5].value(), &Token::mul());
        assert_eq!(findable_tokens[5].position(), Position(11));

        assert_eq!(findable_tokens[6].value(), &Token::Number(4));
        assert_eq!(findable_tokens[6].position(), Position(13));

        assert_eq!(findable_tokens.len(), 7);
    }

    #[test]
    fn test_tokenize_relational() {
        let src = String::from("1 <= 3");

        let findable_tokens = tokenize(&src).unwrap();

        assert_eq!(findable_tokens[0].value(), &Token::Number(1));
        assert_eq!(findable_tokens[0].position(), Position(0));

        assert_eq!(findable_tokens[1].value(), &Token::le());
        assert_eq!(findable_tokens[1].position(), Position(2));

        assert_eq!(findable_tokens[2].value(), &Token::Number(3));
        assert_eq!(findable_tokens[2].position(), Position(5));

    }

    #[test]
    fn test_tokenize_invalid_string() {
        let src = String::from("1 + foo");

        let result = tokenize(&src);

        assert_eq!(result.err(), Some(4));
    }
}