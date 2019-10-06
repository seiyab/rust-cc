use std::env;
use std::io::{self, Write};
use std::process;

mod sourcecode;
use sourcecode::Position;

mod token;
use token::Token;
use token::TokenReader;
use token::tokenize;

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


fn log_error(s: &str) {
    let stderr = io::stderr();
    let mut errhandle = stderr.lock();
    let _ = errhandle.write_all(String::from(format!("{}\n", s)).as_bytes());
}

fn point_error(src: &String, position: usize, message: &str) {
    log_error(src);
    log_error(format!("{}^{}", " ".to_string().repeat(position).as_str(), message).as_str());
}

