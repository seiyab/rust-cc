use std::env;
use std::io::{self, Write};
use std::process;
use std::cmp::min;
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

    let mut src = args[1].clone();
    let tokens = tokenize(&src);
    let mut token_reader = TokenReader::new(&tokens);

    if let Some(&Token::Number(num)) = token_reader.next() {
        println!("  mov rax, {}", num);
    } else {
        log_error("最初のトークンが数字ではありません");
        process::exit(1);
    }

    while let Some(&Token::Operator(c)) = token_reader.next() {
        match c {
            '+' => {
                if let Ok(num) = token_reader.read_number() {
                    println!("  add rax, {}", num);
                } else {
                    log_error("想定外のトークンです。");
                    process::exit(1);
                }
            },
            _ => {
                if let Ok(num) = token_reader.read_number() {
                    println!("  sub rax, {}", num);
                } else {
                    log_error("想定外のトークンです。");
                    process::exit(1);
                }
            },
        }
    }

    println!("  ret");

    process::exit(0);
}

fn tokenize(s: &String) -> Vec<Token> {
    let mut src = s.clone();
    let mut tokens = Vec::new();
    while let Some(head) = src.chars().next() {
        if head==' ' {
            src.drain(..1);
        } else if head=='+' || head=='-' {
            src.drain(..1);
            tokens.push(Token::Operator(head));
        } else if head.is_digit(10) {
            let n = drain_number(&mut src).unwrap();
            tokens.push(Token::Number(n));
        } else {
            log_error("想定外の文字です\n");
        }
    }
    tokens
}

#[derive(Debug, PartialEq)]
enum Token {
    Operator(char),
    Number(i64),
}

struct TokenReader<'a> {
    tokens: &'a Vec<Token>,
    needle: usize,
}

impl<'a> Iterator for TokenReader<'a> {
    type Item = &'a Token;

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
    fn new(tokens: &Vec<Token>) -> TokenReader {
        TokenReader { tokens: tokens, needle: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        if &self.needle < &self.tokens.len() {
            Some(&self.tokens[self.needle])
        } else {
            None
        }
    }

    fn read_number(&mut self) -> Result<i64, ()> {
        match self.next() {
            Some(&Token::Number(n)) => Ok(n),
            _ => Err(()),
        }
    }
}


fn drain_head(s: &mut String) -> Option<char> {
    s.drain(..min(1, s.len())).last()
}


fn drain_number(src: &mut String) -> Result<i64, <i64 as FromStr>::Err> {
    let offset = src.find(|c: char| !c.is_digit(10)).unwrap_or(src.len());
    let digit_str: String = src.drain(..offset).collect();
    digit_str.parse::<i64>()
}

fn log_error(s: &str) {
    let stderr = io::stderr();
    let mut errhandle = stderr.lock();
    errhandle.write_all(String::from(format!("{}\n", s)).as_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let src = String::from("1 + 23 - 2");

        let token = tokenize(&src);

        assert_eq!(token[0], Token::Number(1));
        assert_eq!(token[1], Token::Operator('+'));
        assert_eq!(token[2], Token::Number(23));
        assert_eq!(token[3], Token::Operator('-'));
        assert_eq!(token[4], Token::Number(2));
        assert_eq!(token.len(), 5);
    }
}
