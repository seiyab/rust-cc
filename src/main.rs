use std::cmp::max;
use std::cmp::min;
use std::env;
use std::io::{self, Write};
use std::process;

mod general;
use general::TryReader;

mod sourcecode;

use sourcecode::Position;
use sourcecode::Span;

mod token;
use token::tokenize;

mod parse;
use parse::SyntaxTree;
use parse::Root;

mod compile;
use compile::Compiler;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        log_error("引数の個数が正しくありません");
        process::exit(1);
    }

    let main_label = if env::var("OS").map(|var| var == "MAC".to_string()).unwrap_or(false) {
        "_main"
    } else {
        "main"
    };

    println!(".intel_syntax noprefix");
    println!("{}", format!(".global {}", main_label));
    println!("");
    println!("{}", format!("{}:", main_label));

    let src = args[1].clone();
    let tokens = match tokenize(&src) {
        Ok(tokens) => tokens,
        Err(pos) => {
            point_error(&src, pos, "トークナイズできません");
            process::exit(1);
        }
    };

    let mut token_reader = TryReader::new(&tokens);

    let root = match Root::parse(&mut token_reader) {
        Ok(root) => root,
        Err((Some(span), message)) => {
            span_error(&src, span, message.as_str());
            process::exit(1);
        },
        Err((None, message)) => {
            log_error(message.as_str());
            process::exit(1)
        },
    };

    match Compiler::compile(&root) {
        Ok(compiler) => {
            println!("{}", compiler.assembly_string());
        },
        Err((span, message)) => {
            span_error(&src, span, message.as_str());
            process::exit(1);
        }
    }

    process::exit(0);
}


fn log_error(s: &str) {
    let stderr = io::stderr();
    let mut errhandle = stderr.lock();
    let _ = errhandle.write_all(String::from(format!("{}\n", s)).as_bytes());
}

fn point_error(src: &String, position: Position, message: &str) {
    log_error(src.as_str().split("\n").nth(position.line).unwrap());
    log_error(format!("{}^{}", " ".to_string().repeat(position.pos).as_str(), message).as_str());
}

fn span_error(src: &String, span: Span, message: &str) {
    let Span{start, end} = span;
    let lines: Vec::<&str> = src.as_str().split("\n").collect();
    for i in start.line..end.line+1 {
        let line = lines[i];
        log_error(line);
        let line_start = Position{ line: i, pos: 0 };
        let line_end = Position{ line: i, pos: line.len() };
        let indicator_span = Span{ start: max(start, line_start), end: min(end, line_end) };
        let indicator_length = indicator_span.end.pos - indicator_span.start.pos;
        log_error(format!("{}{}",
            " ".to_string().repeat(indicator_span.start.pos).as_str(),
            "^".to_string().repeat(indicator_length).as_str(),
        ).as_str());

    }
    log_error(message);
}

