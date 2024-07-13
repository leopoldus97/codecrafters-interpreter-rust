use std::{
    env, fs,
    io::{self, Write},
    process,
    sync::atomic::Ordering,
};

use lox_rs::{ast::printer::AstPrinter, parser::Parser, scanner::Scanner, HAD_ERROR};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        eprintln!("Usage: {} [script]", args[0]).unwrap();
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

    if HAD_ERROR.load(Ordering::SeqCst) {
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

        HAD_ERROR.store(false, Ordering::SeqCst)
    }
}

fn run(source: String) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    let mut parser = Parser::new(tokens.to_owned());
    let expression = parser.parse();

    if HAD_ERROR.load(Ordering::SeqCst) {
        return;
    }

    println!("{}", AstPrinter::new().print(expression.unwrap()));
}
