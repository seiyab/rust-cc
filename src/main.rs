use std::env;
use std::io::{self, Write};
use std::process;

mod sourcecode;
use sourcecode::Position;

mod token;
use token::TokenReader;
use token::tokenize;

mod parse;
use parse::SyntaxTree;

mod compile;
use compile::compile;

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
        Ok(tokens) => tokens,
        Err(pos) => {
            point_error(&src, pos, "トークナイズできません");
            process::exit(1);
        }
    };
    let mut token_reader = TokenReader::new(&tokens);

    let syntax_tree = match SyntaxTree::parse(&mut token_reader) {
        Ok(st) => st,
        Err((Some(position), message)) => {
            point_error(&src, position.0, message.as_str());
            process::exit(1);
        },
        Err((None, message)) => {
            log_error(message.as_str());
            process::exit(1)
        },
    };

    for instruction in compile(&syntax_tree) {
        println!("{}", instruction.destination_code());
    }

    println!("  pop rax");

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

