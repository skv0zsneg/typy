use crate::tokenizer::Token;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(i64),
    Bool(bool),
    Name(String),
    BinaryOp {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Assign {
        name: String,
        value: Expr,
    },
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,

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

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = Vec::new();

        while self.current() == &Token::NewLine {
            self.pos += 1;
        }
        while self.current() != &Token::Eof {
            let stmt = self.parse_statement()?;
            stmts.push(stmt);

            while self.current() == &Token::NewLine {
                self.pos += 1;
            }
        }

        Ok(stmts)
    }

    /// Parse Statement
    /// ---------------
    /// `statement = (if-statement | NAME '=' expression | expression)`
    fn parse_statement(&mut self) -> Result<Stmt, String> {
        if self.current() == &Token::If {
            return self.parse_if_statement();
        }
        if let Token::Name(name) = self.current().clone() {
            let saved_pos = self.pos;
            self.pos += 1;

            if self.current() == &Token::Assign {
                self.pos += 1;
                let value = self.parse_expression()?;
                return Ok(Stmt::Assign { name, value });
            }
            self.pos = saved_pos;
        }
        let expr = self.parse_expression()?;
        Ok(Stmt::Expr(expr))
    }

    /// Parse If Statement
    /// ------------------
    /// `if-statement = 'if' expression ':' block ['else' block]`
    fn parse_if_statement(&mut self) -> Result<Stmt, String> {
        self.eat(Token::If)?;
        let condition = self.parse_expression()?;
        self.eat(Token::Colon)?;
        let then_branch = self.parse_block()?;

        let else_branch = if self.current() == &Token::Else {
            self.pos += 1;
            self.eat(Token::Colon)?;
            Some(self.parse_block()?)
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>, String> {
        self.eat(Token::NewLine)?;
        self.eat(Token::Indent)?;

        let mut stmts = Vec::new();
        while self.current() == &Token::NewLine {
            self.pos += 1;
        }

        while self.current() != &Token::Dedent && self.current() != &Token::Eof {
            stmts.push(self.parse_statement()?);

            while self.current() == &Token::NewLine {
                self.pos += 1;
            }
        }

        self.eat(Token::Dedent)?;

        if stmts.is_empty() {
            return Err("SyntaxError: expected an indented block".to_string());
        }

        Ok(stmts)
    }

    /// Parse Expression
    /// ----------------
    /// `expresson = addition ('==' | '!=' | '<' | '>' | '<=' | '>=') addition`
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
            Token::True => {
                self.pos += 1;
                Ok(Expr::Bool(true))
            }
            Token::False => {
                self.pos += 1;
                Ok(Expr::Bool(false))
            }
            Token::Name(name) => {
                let var_name = name.clone();
                self.pos += 1;
                Ok(Expr::Name(var_name))
            }
            Token::LParen => {
                self.pos += 1;
                let expr = self.parse_expression()?;
                self.eat(Token::RParen)?;
                Ok(expr)
            }
            _ => Err(format!(
                "SyntaxError: expected expression but got {:?}",
                self.current()
            )),
        }
    }
}
