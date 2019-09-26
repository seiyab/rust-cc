use std::env;
use std::io::{self, Write};
use std::process;

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
    println!("  mov rax, {}", args[1].parse::<i64>().unwrap());
    println!("  ret");

    process::exit(0);
}
