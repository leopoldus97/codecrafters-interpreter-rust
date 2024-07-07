use std::fs;
use std::io::{self, Write};
use std::{env, process};

use interpreter_starter_rust::scanner::Scanner;
use interpreter_starter_rust::HAD_ERROR;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        writeln!(io::stderr(), "Usage: {} [script]", args[0]).unwrap();
        process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    };
}

fn run_file(file_path: &String) {
    let file_contents = fs::read_to_string(file_path).unwrap_or_else(|_| {
        eprintln!("Error reading file '{}'", file_path);
        process::exit(74);
    });

    run(file_contents);

    if HAD_ERROR.load(std::sync::atomic::Ordering::SeqCst) {
        process::exit(65);
    }
}

fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        if line.trim().is_empty() {
            break;
        }

        run(line);

        HAD_ERROR.store(false, std::sync::atomic::Ordering::SeqCst)
    }
}

fn run(source: String) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token);
    }
}
