use std::env;
use std::io::{self, Write};
use std::process;
use std::cmp::min;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        let stderr = io::stderr();
        let mut errhandle = stderr.lock();
        errhandle.write_all(String::from("引数の個数が正しくありません\n").as_bytes());
        process::exit(1);
    }

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    let mut src = args[1].clone();

    let operator_offset = src.find(|c: char| c=='+' || c=='-').unwrap_or(src.len());
    let before_operator: String = src.drain(..operator_offset).collect();
    let num = before_operator.parse::<i64>().unwrap();
    println!("  mov rax, {}", num);

    while let Some(operator) = src.drain(..min(1, src.len())).last() {
        let operator_offset = src.find(|c: char| c=='+' || c=='-').unwrap_or(src.len());
        let before_operator: String = src.drain(..operator_offset).collect();
        let num = before_operator.parse::<i64>().unwrap();

        if operator == '+' {
            println!("  add rax, {}", num);
        } else if operator == '-' {
            println!("  sub rax, {}", num);
        } else {
            let stderr = io::stderr();
            let mut errhandle = stderr.lock();
            errhandle.write_all(String::from("想定外の演算子です。\n").as_bytes());
            process::exit(1);
        }
    }

    println!("  ret");

    process::exit(0);
}

