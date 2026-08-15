mod types;
mod compiler;
mod parser;
mod symbol;
mod tokenizer;
mod object;
mod vm;

use types::Checker;
use compiler::Compiler;
use parser::Parser;
use std::io::{self, Write};
use symbol::Interner;
use tokenizer::tokenize;
use vm::VM;

fn get_code_row() -> String {
    print!(">>> ");
    io::stdout().flush().expect("Error on flushing");
    let mut stdin_conetent = String::new();
    io::stdin()
        .read_line(&mut stdin_conetent)
        .expect("Error on readnig line.");
    stdin_conetent
}

fn main() {
    let debug = true;
    println!("=== TyPy (v 0.1.0) ===");
    if debug {
        println!("Debug mode is ON.");
    }

    let mut vm = VM::new();
    let mut interner = Interner::new();
    let mut checker = Checker::new();
    loop {
        let source = get_code_row();

        let tokens = tokenize(source);
        if debug {
            println!("[1] Tokens: {:?}", tokens);
        }

        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Parsing error");
        if debug {
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
        if debug {
            println!("\n[3] Byte-code: {:?}", bytecode);
            println!("\n[4] Running:");
        }

        match vm.run(&bytecode, &interner, debug) {
            Ok(result) => println!("{}", result),
            Err(e) => eprintln!("{}", e),
        }
    }
}
