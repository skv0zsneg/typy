#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(i64),

    True,
    False,

    Plus,
    Minus,
    Star,
    Slash,

    Eq,
    NotEq,
    Greater,
    Less,
    GreaterEq,
    LessEq,

    LParen,
    RParen,

    Name(String),

    Assign,

    Colon,
    NewLine,
    Indent,
    Dedent,

    If,
    Else,

    Eof,
}

pub fn tokenize(input: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    let mut indent_stack = vec![0];
    let mut at_line_start = true;

    while let Some(ch) = chars.next() {
        if ch == '\n' {
            tokens.push(Token::NewLine);
            at_line_start = true;
            continue;
        }

        if at_line_start {
            if ch == ' ' || ch == '\t' {
                let mut indent_level = if ch == ' ' { 1 } else { 4 };
                while let Some(&next_c) = chars.peek() {
                    if next_c == ' ' {
                        indent_level += 1;
                        chars.next();
                    } else if next_c == '\t' {
                        indent_level += 4;
                        chars.next();
                    } else {
                        break;
                    }
                }

                if chars.peek() == Some(&'\n') || chars.peek().is_none() {
                    continue;
                }

                let current_top = *indent_stack.last().unwrap();
                if indent_level > current_top {
                    indent_stack.push(indent_level);
                    tokens.push(Token::Indent);
                } else if indent_level < current_top {
                    while let Some(&top) = indent_stack.last() {
                        if top > indent_level {
                            indent_stack.pop();
                            tokens.push(Token::Dedent);
                        } else {
                            break;
                        }
                    }
                    if *indent_stack.last().unwrap() != indent_level {
                        panic!(
                            "IndentationError: unindent does not match any outer indentation level"
                        );
                    }
                }

                at_line_start = false;
            } else {
                while let Some(&top) = indent_stack.last() {
                    if top > 0 {
                        indent_stack.pop();
                        tokens.push(Token::Dedent);
                    } else {
                        break;
                    }
                }
                at_line_start = false;
            }
        }

        if ch.is_whitespace() {
            continue;
        }

        if ch.is_ascii_digit() {
            let mut num_str = ch.to_string();
            while let Some(&next_ch) = chars.peek() {
                if next_ch.is_ascii_digit() {
                    num_str.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            tokens.push(Token::Number(num_str.parse::<i64>().unwrap()));
            continue;
        }

        if ch.is_alphabetic() || ch == '_' {
            let mut name = ch.to_string();
            while let Some(&next_ch) = chars.peek() {
                if next_ch.is_alphanumeric() || next_ch == '_' {
                    name.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            match name.as_str() {
                "True" => tokens.push(Token::True),
                "False" => tokens.push(Token::False),
                "if" => tokens.push(Token::If),
                "else" => tokens.push(Token::Else),
                _ => tokens.push(Token::Name(name)),
            }
            continue;
        }

        if ch == '<' {
            if chars.peek() == Some(&'=') {
                chars.next();
                tokens.push(Token::LessEq);
            } else {
                tokens.push(Token::Less);
            }
            continue;
        }

        if ch == '>' {
            if chars.peek() == Some(&'=') {
                chars.next();
                tokens.push(Token::GreaterEq);
            } else {
                tokens.push(Token::Greater);
            }
            continue;
        }

        if ch == '=' {
            if chars.peek() == Some(&'=') {
                chars.next();
                tokens.push(Token::Eq);
            } else {
                tokens.push(Token::Assign);
            }
            continue;
        }

        if ch == '!' {
            if chars.peek() == Some(&'=') {
                chars.next();
                tokens.push(Token::NotEq);
            } else {
                panic!("SyntaxError: unknown token: !");
            }
            continue;
        }

        match ch {
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Star),
            '/' => tokens.push(Token::Slash),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            ':' => tokens.push(Token::Colon),
            _ => panic!("SyntaxError: unknown token: {}", ch),
        }
    }

    if let Some(last_token) = tokens.last()
        && *last_token != Token::NewLine
        && !tokens.is_empty()
    {
        tokens.push(Token::NewLine);
    }

    while indent_stack.len() > 1 {
        indent_stack.pop();
        tokens.push(Token::Dedent);
    }

    tokens.push(Token::Eof);
    tokens
}
