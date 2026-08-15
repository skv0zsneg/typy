use crate::tokenizer::Token;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(i64),
    Name(String),
    Bool(bool),
    Assign {
        name: String,
        value: Box<Expr>,
    },
    BinaryOp {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    // Arithmetic
    Plus,
    Minus,
    Star,
    Slash,

    // Comparison
    Eq,
    NotEq,
    Less,
    Greater,
    LessEq,
    GreaterEq,
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn eat(&mut self, expected: Token) -> Result<(), String> {
        if self.current() == &expected {
            self.pos += 1;
            Ok(())
        } else {
            Err(format!(
                "SyntaxError: expected {:?}, but got {:?}",
                expected,
                self.current()
            ))
        }
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        let result = self.parse_statement()?;
        self.eat(Token::Eof)?;
        Ok(result)
    }

    /// Parsing Expression
    /// ------------------
    /// `expression = addition ('>' | '>=' | '<' | '<=' | '==' | '!=') addition`
    fn parse_expression(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_addition()?;
        loop {
            let op = match self.current() {
                Token::Eq => Some(Operator::Eq),
                Token::NotEq => Some(Operator::NotEq),
                Token::Less => Some(Operator::Less),
                Token::Greater => Some(Operator::Greater),
                Token::LessEq => Some(Operator::LessEq),
                Token::GreaterEq => Some(Operator::GreaterEq),
                _ => None,
            };
            if let Some(operator) = op {
                self.pos += 1;
                let right = self.parse_addition()?;
                left = Expr::BinaryOp {
                    left: Box::new(left),
                    op: operator,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        Ok(left)
    }

    /// Parse Addition
    /// --------------
    /// `addition = term ('+' | '-') term`
    fn parse_addition(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;
        loop {
            let op = match self.current() {
                Token::Plus => Some(Operator::Plus),
                Token::Minus => Some(Operator::Minus),
                _ => None,
            };
            if let Some(operator) = op {
                self.pos += 1;
                let right = self.parse_term()?;
                left = Expr::BinaryOp {
                    left: Box::new(left),
                    op: operator,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        Ok(left)
    }

    /// Parsing Term
    /// ------------
    /// `term = factor ('*' | '/') factor`
    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_factor()?;
        loop {
            let op = match self.current() {
                Token::Star => Some(Operator::Star),
                Token::Slash => Some(Operator::Slash),
                _ => None,
            };
            if let Some(operator) = op {
                self.pos += 1;
                let right = self.parse_factor()?;
                left = Expr::BinaryOp {
                    left: Box::new(left),
                    op: operator,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        Ok(left)
    }

    /// Parsing Factor
    /// -------------
    /// `factor = NUMBER | NAME | 'True' | 'False' | '(' expression ')'`
    fn parse_factor(&mut self) -> Result<Expr, String> {
        match self.current() {
            Token::Number(n) => {
                let value = *n;
                self.pos += 1;
                Ok(Expr::Number(value))
            }

            Token::Name(name) => {
                let var_name = name.clone();
                self.pos += 1;
                Ok(Expr::Name(var_name))
            }

            Token::True => {
                self.pos += 1;
                Ok(Expr::Bool(true))
            }

            Token::False => {
                self.pos += 1;
                Ok(Expr::Bool(false))
            }

            Token::LParen => {
                self.pos += 1;
                let expr = self.parse_expression()?;
                self.eat(Token::RParen)?;
                Ok(expr)
            }

            _ => Err(format!(
                "SyntaxError: expected number or '(' but got {:?}",
                self.current()
            )),
        }
    }

    /// Parse Statement
    /// ---------------
    /// `statement = assignment | expression`
    ///
    /// `assignment = NAME '=' expression`
    pub fn parse_statement(&mut self) -> Result<Expr, String> {
        if let Token::Name(name) = self.current().clone() {
            let current_pos = self.pos;
            self.pos += 1;

            if self.current() == &Token::Assign {
                self.pos += 1;
                let value = self.parse_expression()?;
                return Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                });
            } else {
                self.pos = current_pos;
            }
        }

        self.parse_expression()
    }
}
