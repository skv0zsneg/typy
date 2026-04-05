#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(i32),  // 1, 2, 3, ..., 4 294 967 295
    Plus,         // +
    Minus,        // -
    Star,         // *
    Slash,        // /
    LParen,       // (
    RParen,       // )
    Name(String), // var_name
    Assign,       // =
    EOF,          // <Flag For Code Finish>
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars();

    while let Some(ch) = chars.next() {
        if ch.is_whitespace() {
            continue;
        }

        if ch.is_digit(10) {
            let mut num_str = ch.to_string();
            // TODO: How to iterate without cloning?
            for next_ch in chars.clone() {
                if next_ch.is_digit(10) {
                    num_str.push(next_ch);
                    chars.next();
                } else {
                    break;
                }
            }
            let num = num_str.parse::<i32>().unwrap();
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

// === Tests ===

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple() {
        let tokens = tokenize("1 + 2");
        assert_eq!(
            tokens,
            vec![Token::Number(1), Token::Plus, Token::Number(2), Token::EOF]
        );
    }
}
