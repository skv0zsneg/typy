mod compiler;
mod object;
mod parser;
mod symbol;
mod tokenizer;
mod types;
mod vm;

use compiler::Compiler;
use parser::Parser;
use std::env;
use std::io::{self, Write};
use symbol::Interner;
use tokenizer::tokenize;
use types::Checker;
use vm::VM;

struct Config {
    debug: bool,
}

fn get_code_row() -> String {
    print!(">>> ");
    io::stdout().flush().expect("Error on flushing");
    let mut stdin_conetent = String::new();
    io::stdin()
        .read_line(&mut stdin_conetent)
        .expect("Error on readnig line.");
    stdin_conetent
}

fn get_config_from_args() -> Config {
    let args: Vec<String> = env::args().collect();

    let mut debug = false;
    for arg in &args[1..] {
        if arg == "--debug" || arg == "-d" {
            debug = true;
        } else {
            panic!("Got unknown argument: {}", arg);
        };
    }

    Config { debug }
}

fn main() {
    let config = get_config_from_args();
    println!("=== TyPy (v {}) ===", env!("CARGO_PKG_VERSION"));
    if config.debug {
        println!("Debug mode is ON.");
    }

    let mut vm = VM::new();
    let mut interner = Interner::new();
    let mut checker = Checker::new();
    loop {
        let source = get_code_row();

        let tokens = tokenize(source);
        if config.debug {
            println!("[1] Tokens: {:?}", tokens);
        }

        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Parsing error");
        if config.debug {
            println!("\n[2] AST: {:#?}", ast);
        }

        match checker.check(&ast, &mut interner) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        }

        let compiler = Compiler::new();
        let bytecode = compiler.compile(&ast, &mut interner);
        if config.debug {
            println!("\n[3] Byte-code: {:?}", bytecode);
            println!("\n[4] Running:");
        }

        match vm.run(&bytecode, &interner, config.debug) {
            Ok(result) => println!("{}", result),
            Err(e) => eprintln!("{}", e),
        }
    }
}
