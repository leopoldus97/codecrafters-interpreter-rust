use std::{
    env, fs,
    io::{self, Write},
    process,
    sync::atomic::Ordering,
};

use lox_rs::{
    ast::stmt::Stmt,
    interpreter::{environment::Environment, Interpreter},
    parser::Parser,
    scanner::Scanner,
    HAD_ERROR, HAD_RUNTIME_ERROR,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: {} [script]", args[0]);
            process::exit(64);
        }
    }
}

fn run_file(file_path: &String) {
    let file_contents = fs::read_to_string(file_path).unwrap_or_else(|_| {
        eprintln!("Error reading file '{}'", file_path);
        process::exit(74);
    });

    let mut interpreter = Interpreter::new();

    run(file_contents, &mut interpreter);

    if HAD_ERROR.load(Ordering::SeqCst) {
        process::exit(65);
    }

    if HAD_RUNTIME_ERROR.load(Ordering::SeqCst) {
        process::exit(70);
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

        let mut interpreter = Interpreter::new();

        run(line, &mut interpreter);

        HAD_ERROR.store(false, Ordering::SeqCst)
    }
}

fn run(source: String, interpreter: &mut Interpreter) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    let mut parser = Parser::new(tokens.to_owned());
    let statements = parser.parse::<Box<dyn Stmt>>().unwrap();

    if HAD_ERROR.load(Ordering::SeqCst) {
        return;
    }

    interpreter.interpret(statements);
}
