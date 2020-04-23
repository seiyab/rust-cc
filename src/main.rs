use std::cmp::max;
use std::cmp::min;
use std::env;
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

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err("引数の個数が正しくありません".to_string());
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
            return Err(point_error_position(&src, pos, "トークナイズできません"));
        }
    };

    let mut token_reader = TryReader::new(&tokens);

    let root = match Root::parse(&mut token_reader) {
        Ok(root) => root,
        Err((Some(span), message)) => {
            return Err(point_error_span(&src, span, message.as_str()))
        },
        Err((None, message)) => {
            return Err(message);
        },
    };

    match Compiler::compile(&root) {
        Ok(compiler) => {
            println!("{}", compiler.assembly_string());
        },
        Err((span, message)) => {
            return Err(point_error_span(&src, span, message.as_str()))
        }
    }

    process::exit(0);
}


fn point_error_position(src: &String, position: Position, message: &str) -> String {
    let mut err = src.as_str().split("\n").nth(position.line).unwrap().to_string();
    err.push_str(format!("{}^{}", " ".to_string().repeat(position.pos).as_str(), message).as_str());
    err
}

fn point_error_span(src: &String, span: Span, message: &str) -> String {
    let mut err = String::new();
    let Span{start, end} = span;
    let lines: Vec::<&str> = src.as_str().split("\n").collect();
    for i in start.line..end.line+1 {
        let line = lines[i];
        err.push_str(line);
        let line_start = Position{ line: i, pos: 0 };
        let line_end = Position{ line: i, pos: line.len() };
        let indicator_span = Span{ start: max(start, line_start), end: min(end, line_end) };
        let indicator_length = indicator_span.end.pos - indicator_span.start.pos;
        err.push_str(format!("{}{}",
            " ".to_string().repeat(indicator_span.start.pos).as_str(),
            "^".to_string().repeat(indicator_length).as_str(),
        ).as_str());

    }
    err.push_str(message);
    err
}

