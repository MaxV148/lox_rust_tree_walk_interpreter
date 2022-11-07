use std::{
    env,
    fs::{self},
    io::{self, BufRead, BufReader},
};
use compiler::Scanner;

fn main() {

     let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        run_prompt();
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        println!("Usage: rlox [script.lox]");
    } 
}

fn run_file(path: &str) {
    match fs::read_to_string(path) {
        Ok(source) => run(source),
        Err(_) => panic!(""),
    };
}

fn run_prompt() {
    println!("Welcom to the Lox REPL.");
    let mut reader = BufReader::new(io::stdin());

    loop {
        
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(_) => {
                run(line);
            },
            Err(_) => return,
        }
    }
}

fn run(source: String) {
    let mut scanner = Scanner::new(source.chars());
    let tokens = scanner.scan_tokens();
    for token in tokens {
        println!("{}", token);
    }
}
