use yathon::checker::Checker;
use yathon::compiler::Compiler;
use yathon::parser::Parser;
use yathon::symbol::Interner;
use yathon::tokenizer::tokenize;
use yathon::vm::VM;

fn execute_one_line(source: &str) -> String {
    let tokens = tokenize(source.to_string());
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Parsing error");

    let mut interner = Interner::new();

    let mut checker = Checker::new();
    if let Err(e) = checker.check(&ast, &mut interner) {
        return e;
    }

    let compiler = Compiler::new();
    let bytecode = compiler.compile(&ast, &mut interner);

    let mut vm = VM::new();
    match vm.run(&bytecode, &interner, false) {
        Ok(result) => format!("{}", result),
        Err(e) => format!("{}", e),
    }
}

fn execute_several_lines(lines: Vec<&str>) -> Vec<String> {
    let mut vm = VM::new();
    let mut interner = Interner::new();
    let mut checker = Checker::new();

    let mut results: Vec<String> = Vec::new();

    for line in lines {
        let tokens = tokenize(line.to_string());
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Parsing error");

        if let Err(e) = checker.check(&ast, &mut interner) {
            results.push(e);
            continue;
        }

        let compiler = Compiler::new();
        let bytecode = compiler.compile(&ast, &mut interner);

        let result = match vm.run(&bytecode, &interner, false) {
            Ok(result) => format!("{}", result),
            Err(e) => format!("{}", e),
        };
        results.push(result);
    }
    results
}

#[test]
fn test_int_sum() {
    assert_eq!(execute_one_line("2 + 2"), "4".to_string());
    assert_eq!(execute_one_line("10 + 10"), "20".to_string());
    assert_eq!(execute_one_line("0 + 0"), "0".to_string());
    assert_eq!(execute_one_line("123 + 0"), "123".to_string());
}

#[test]
fn test_int_sub() {
    let result = execute_one_line("5 - 2");
    assert_eq!(result, "3".to_string());
}

#[test]
fn test_int_mul() {
    let result = execute_one_line("2 * 5");
    assert_eq!(result, "10".to_string());
}

#[test]
fn test_int_div() {
    let result = execute_one_line("10 / 5");
    assert_eq!(result, "2".to_string());
}

#[test]
fn test_int_zero_div() {
    let result = execute_one_line("10 / 0");
    assert_eq!(result, "ZeroDivisionError: division by zero".to_string());
}

#[test]
fn test_int_vars() {
    let result = execute_several_lines(vec!["x = 10", "x + 5"]);
    assert_eq!(result, vec!["10".to_string(), "15".to_string()]);
}

#[test]
fn test_static_name_error() {
    let result = execute_one_line("a + 1");
    assert_eq!(result, "NameError: name 'a' is not defined");
}

#[test]
fn test_multiple_vars_persistence() {
    // Этот тест упадет, если Interner создается внутри цикла
    let result = execute_several_lines(vec!["x = 10", "y = 20", "x + y"]);
    assert_eq!(result, vec!["10", "20", "30"]);
}
