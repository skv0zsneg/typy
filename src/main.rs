mod compiler;
mod parser;
mod tokenizer;
mod vm;

use compiler::compile;
use parser::Parser;
use tokenizer::tokenize;
use vm::run_vm;

fn main() {
    let source = "x = 10";

    println!("=== yathon VM ===");
    println!("Source code: {}\n", source);

    let tokens = tokenize(source);
    println!("[1] Tokens: {:?}", tokens);

    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Parsing error");
    println!("\n[2] AST: {:#?}", ast);

    let bytecode = compile(&ast);
    println!("\n[3] Byte-code: {:?}", bytecode);

    println!("\n[4] Running:");
    match run_vm(&bytecode, true) {
        Ok(result) => println!("\n✅ Result: {}", result),
        Err(e) => eprintln!("\n❌ Err: {}", e),
    }
}
