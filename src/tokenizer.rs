/// Tokens using in languages
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    /// Signed 32 bit number.
    Number(i64),
    /// Plus "+"
    Plus,
    /// Minus "-"
    Minus,
    /// Star "*"
    Star,
    /// Slash "/"
    Slash,
    /// Open paren "("
    LParen,
    /// Closed paren ")"       
    RParen,
    /// Name for variavle - string
    Name(String),
    /// Assign "="
    Assign,
    /// Flag end of code
    EOF,
}

/// Tokenize incoming code.
pub fn tokenize(input: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars();

    while let Some(ch) = chars.next() {
        if ch.is_whitespace() {
            continue;
        }

        if ch.is_ascii_digit() {
            let mut num_str = ch.to_string();
            // TODO: How to iterate without cloning?
            for next_ch in chars.clone() {
                if next_ch.is_ascii_digit() {
                    num_str.push(next_ch);
                    chars.next();
                } else {
                    break;
                }
            }
            let num = num_str.parse::<i64>().unwrap();
            tokens.push(Token::Number(num));
            continue;
        }

        if ch.is_alphabetic() || ch == '_' {
            let mut name = ch.to_string();
            // TODO: How to iterate without cloning?
            for next_ch in chars.clone() {
                if next_ch.is_alphanumeric() || next_ch == '_' {
                    name.push(next_ch);
                    chars.next();
                } else {
                    break;
                }
            }
            tokens.push(Token::Name(name));
            continue;
        }

        match ch {
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Star),
            '/' => tokens.push(Token::Slash),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '=' => tokens.push(Token::Assign),
            _ => panic!("Unknown token: {}", ch),
        }
    }
    tokens.push(Token::EOF);
    tokens
}

