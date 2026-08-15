/// Tokens using in languages
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    /// Signed 64 bit number.
    Number(i64),

    /// Bolean "True"
    True,
    /// Bolean "False"
    False,

    /// Plus "+"
    Plus,
    /// Minus "-"
    Minus,
    /// Star "*"
    Star,
    /// Slash "/"
    Slash,

    /// Equal "=="
    Eq,
    /// Equal "!="
    NotEq,
    /// Greater ">"
    Greater,
    /// Less "<"
    Less,
    /// Greater equal ">="
    GreaterEq,
    /// Less equal "<="
    LessEq,

    /// Open paren "("
    LParen,
    /// Closed paren ")"       
    RParen,

    /// Name for variavle - string
    Name(String),

    /// Assign "="
    Assign,

    /// Flag end of code
    Eof,
}

/// Tokenize incoming code.
pub fn tokenize(input: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
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

            // Key words handling.
            match name.as_str() {
                "True" => tokens.push(Token::True),
                "False" => tokens.push(Token::False),
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
            _ => panic!("SyntaxError: unknown token: {}", ch),
        }
    }
    tokens.push(Token::Eof);
    tokens
}
