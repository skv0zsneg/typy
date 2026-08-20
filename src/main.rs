mod compiler;
mod object;
mod parser;
mod symbol;
mod tokenizer;
mod types;
mod vm;

use compiler::Compiler;
use object::Object;
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

fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Error on flushing");
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Error on reading line.");
    line
}

fn needs_block(line: &str) -> bool {
    let trimmed = line.trim_end();
    trimmed.ends_with(':')
}

fn is_block_end(line: &str) -> bool {
    let trimmed = line.trim();

    if trimmed.is_empty() {
        return true;
    }
    if line.starts_with(' ') || line.starts_with('\t') {
        return false;
    }
    if trimmed == "else:" || trimmed.starts_with("elif ") {
        return false;
    }
    true
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
        let mut source_buffer = Vec::new();
        let mut in_block = false;

        loop {
            let prompt = if in_block { "... " } else { ">>> " };
            let line = read_line(prompt);
            if line.is_empty() {
                println!();
                return;
            }

            source_buffer.push(line.clone());

            if !in_block {
                if needs_block(&line) {
                    in_block = true;
                } else {
                    break;
                }
            } else {
                if is_block_end(&line) {
                    break;
                }
            }
        }
        let source = source_buffer.join("");

        let tokens = tokenize(source);
        if config.debug {
            println!("[1] Tokens: {:?}", tokens);
        }

        let mut parser = Parser::new(tokens);
        let ast = match parser.parse() {
            Ok(ast) => ast,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };

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
            Ok(Object::None) => {}
            Ok(result) => println!("{}", result),
            Err(e) => eprintln!("{}", e),
        }
    }
}
