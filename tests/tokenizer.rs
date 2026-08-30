use typy::tokenizer::{Token, tokenize, tokenize_str};

fn name(text: &str) -> Token {
    Token::Name(text.to_string())
}

#[test]
fn empty_input_produces_eof() {
    assert_eq!(tokenize_str(""), vec![Token::Eof]);
}

#[test]
fn owned_string_entry_point_still_works() {
    assert_eq!(
        tokenize(String::from("1")),
        vec![Token::Number(1), Token::NewLine, Token::Eof]
    );
}

#[test]
fn tokenizes_number() {
    assert_eq!(
        tokenize_str("42"),
        vec![Token::Number(42), Token::NewLine, Token::Eof]
    );
}

#[test]
fn tokenizes_assignment() {
    assert_eq!(
        tokenize_str("x = 1"),
        vec![
            name("x"),
            Token::Assign,
            Token::Number(1),
            Token::NewLine,
            Token::Eof,
        ]
    );
}

#[test]
fn tokenizes_keywords() {
    assert_eq!(
        tokenize_str("if:"),
        vec![Token::If, Token::Colon, Token::NewLine, Token::Eof]
    );
}

#[test]
fn tokenizes_indent_and_dedent() {
    let input = "if:\n    x\n";

    assert_eq!(
        tokenize_str(input),
        vec![
            Token::If,
            Token::Colon,
            Token::NewLine,
            Token::Indent,
            name("x"),
            Token::NewLine,
            Token::Dedent,
            Token::Eof,
        ]
    );
}

#[test]
fn dedents_to_zero_on_unindented_line() {
    let input = "if:\n    x\ny\n";

    assert_eq!(
        tokenize_str(input),
        vec![
            Token::If,
            Token::Colon,
            Token::NewLine,
            Token::Indent,
            name("x"),
            Token::NewLine,
            Token::Dedent,
            name("y"),
            Token::NewLine,
            Token::Eof,
        ]
    );
}

#[test]
fn blank_lines_do_not_change_indentation() {
    let input = "if:\n\n    x\n";

    assert_eq!(
        tokenize_str(input),
        vec![
            Token::If,
            Token::Colon,
            Token::NewLine,
            Token::NewLine,
            Token::Indent,
            name("x"),
            Token::NewLine,
            Token::Dedent,
            Token::Eof,
        ]
    );
}

#[test]
#[should_panic(expected = "IndentationError")]
fn indentation_mismatch_panics() {
    let input = "if:\n        x\n    y\n";

    let _ = tokenize_str(input);
}

#[test]
#[should_panic(expected = "SyntaxError: unknown token: @")]
fn unknown_token_panics() {
    let _ = tokenize_str("@");
}

#[test]
#[should_panic(expected = "SyntaxError: invalid integer literal")]
fn overflowing_integer_panics() {
    let _ = tokenize_str("999999999999999999999999999999");
}

#[test]
fn simple_function_definition() {
    let input = "def add(a: int, b: int) -> int:\n    return a + b\n";

    assert_eq!(
        tokenize_str(input),
        vec![
            Token::Def,
            name("add"),
            Token::LParen,
            name("a"),
            Token::Colon,
            name("int"),
            Token::Comma,
            name("b"),
            Token::Colon,
            name("int"),
            Token::RParen,
            Token::RArrow,
            name("int"),
            Token::Colon,
            Token::NewLine,
            Token::Indent,
            Token::Return,
            name("a"),
            Token::Plus,
            name("b"),
            Token::NewLine,
            Token::Dedent,
            Token::Eof,
        ]
    );
}

#[test]
fn function_definition_without_args() {
    let input = "def get_1() -> int:\n    return 1\n";

    assert_eq!(
        tokenize_str(input),
        vec![
            Token::Def,
            name("get_1"),
            Token::LParen,
            Token::RParen,
            Token::RArrow,
            name("int"),
            Token::Colon,
            Token::NewLine,
            Token::Indent,
            Token::Return,
            Token::Number(1),
            Token::NewLine,
            Token::Dedent,
            Token::Eof,
        ]
    );
}

#[test]
fn function_call() {
    let input = "foo()";

    assert_eq!(
        tokenize_str(input),
        vec![
            name("foo"),
            Token::LParen,
            Token::RParen,
            Token::NewLine,
            Token::Eof,
        ]
    );
}

#[test]
fn function_call_with_args() {
    let input = "foo(a, b)";

    assert_eq!(
        tokenize_str(input),
        vec![
            name("foo"),
            Token::LParen,
            name("a"),
            Token::Comma,
            name("b"),
            Token::RParen,
            Token::NewLine,
            Token::Eof,
        ]
    );
}
