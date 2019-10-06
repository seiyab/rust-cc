use std::env;
use std::io::{self, Write};
use std::process;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        log_error("引数の個数が正しくありません");
        process::exit(1);
    }

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    let src = args[1].clone();
    let tokens = match tokenize(&src) {
        Ok(t) => t,
        Err(pos) => {
            point_error(&src, pos, "トークナイズできません");
            process::exit(1);
        }
    };
    let mut token_reader = TokenReader::new(&tokens);

    let first_token = token_reader.next();
    match first_token {
        Some(token_and_position) => 
        if let Token::Number(num) = token_and_position.token { println!("  mov rax, {}", num) }
        else {
            point_error(&src, 0, "最初のトークンが数字ではありません");
            process::exit(1);
        },
        _ => {
            point_error(&src, 0, "最初のトークンが数字ではありません");
            process::exit(1);
        },
    }

    while let Some(token_and_position) = token_reader.next() {
        if let Token::Operator(c) = token_and_position.token {
            let number_token = token_reader.read_number();
            match number_token {
                Ok(num) => match c {
                    '+' => println!("  add rax, {}", num),
                    _ => println!("  sub rax, {}", num),
                }
                Err(Some(Position(pos))) => {
                    point_error(&src, pos, "数字を期待していました");
                    process::exit(1);
                },
                _ => {
                    point_error(&src, src.len(), "数字を期待していましたが、トークンがありません");
                    process::exit(1);
                }
            }
        } else {
            point_error(&src, token_and_position.position.0, "演算子を期待していました");
            process::exit(1);

        }
    }

    println!("  ret");

    process::exit(0);
}

fn tokenize(s: &String) -> Result<Vec<TokenAndPosition>, usize> {
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

struct TokenAndPosition {
    token: Token,
    position: Position,
}

#[derive(Debug, PartialEq)]
enum Token {
    Operator(char),
    Number(i64),
}

struct TokenReader<'a> {
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
    fn new(tokens: &Vec<TokenAndPosition>) -> TokenReader {
        TokenReader { tokens: tokens, needle: 0 }
    }

    fn read_number(&mut self) -> Result<i64, Option<Position>> {
        match self.next() {
            Some(token_and_position) =>
            if let Token::Number(n) = token_and_position.token { Ok(n) }
            else { Err(Some(token_and_position.position)) },
            _ => Err(None)
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Position(usize);

fn drain_number(src: &mut String) -> Result<i64, <i64 as FromStr>::Err> {
    let offset = src.find(|c: char| !c.is_digit(10)).unwrap_or(src.len());
    let digit_str: String = src.drain(..offset).collect();
    digit_str.parse::<i64>()
}

fn log_error(s: &str) {
    let stderr = io::stderr();
    let mut errhandle = stderr.lock();
    let _ = errhandle.write_all(String::from(format!("{}\n", s)).as_bytes());
}

fn point_error(src: &String, position: usize, message: &str) {
    log_error(src);
    log_error(format!("{}^{}", " ".to_string().repeat(position).as_str(), message).as_str());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let src = String::from("1 + 23 - 2");

        let token_and_position = tokenize(&src).unwrap();

        assert_eq!(token[0].token, Token::Number(1));
        assert_eq!(token[0].position, Position(0));
        assert_eq!(token[1].token, Token::Operator('+'));
        assert_eq!(token[1]position, Token::Operator('+'));
        assert_eq!(token[2].token, Token::Number(23));
        assert_eq!(token[2].position, Position(4));
        assert_eq!(token[3]token, Token::Operator('-'));
        assert_eq!(token[3]position, Position(7));
        assert_eq!(token[4].token, Token::Number(2));
        assert_eq!(token[4].position, Position(9));
        assert_eq!(token.len(), 5);
    }

    #[test]
    fn test_tokenize_invalid_string() {
        let src = String::from("1 + foo");

        let error = tokenize(&src);

        assert_eq!(error, Err(4));
    }
}
