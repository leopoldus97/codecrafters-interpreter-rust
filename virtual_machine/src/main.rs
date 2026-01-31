use std::{
    env, fs, io::{self, Write}, path::PathBuf, process
};

use virtual_machine::vm::{InterpretResult, VM};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut vm = VM::new();

    match args.len() {
        1 => repl(&mut vm),
        2 => run_file(&mut vm, &args[1]),
        _ => {
            eprintln!("Usage: clox {}", args[0]);
            process::exit(64);
        }
    }
}

fn run_file(vm: &mut VM, file_name: &String) {
    if file_name.contains("..") || file_name.contains("/") || file_name.contains("\\") {
        return;
    }
    let file_path = PathBuf::from(file_name);

    let file_contents = fs::read_to_string(&file_path).unwrap_or_else(|_| {
        eprintln!("Error reading file '{:?}'", file_path);
        process::exit(74);
    });

    let result = vm.interpret(&file_contents);

    if result == InterpretResult::CompileError {
        process::exit(65);
    }

    if result == InterpretResult::RuntimeError {
        process::exit(70);
    }
}

fn repl(vm: &mut VM) {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        if line.trim().is_empty() {
            break;
        }

        vm.interpret(&line);
    }
}
