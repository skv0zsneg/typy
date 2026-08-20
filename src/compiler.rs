use crate::object::Object;
use crate::parser::{Expr, Operator, Stmt};
use crate::symbol::{Interner, SymbolId};

#[derive(Debug, Clone)]
pub enum Instruction {
    // Vars & constants
    LoadConst(Object),
    LoadName(SymbolId),
    StoreName(SymbolId),
    Pop,

    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,

    // Comparison
    Eq,
    NotEq,
    Less,
    Greater,
    LessEq,
    GreaterEq,

    // Jump on conditions
    Jump(usize),
    JumpIfFalse(usize),
}

pub struct Compiler {
    code: Vec<Instruction>,
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Compiler {
    pub fn new() -> Self {
        Compiler { code: Vec::new() }
    }

    pub fn compile(mut self, stmts: &[Stmt], interner: &mut Interner) -> Vec<Instruction> {
        for stmt in stmts {
            self.compile_stmt(stmt, interner);
        }
        self.code
    }

    fn compile_stmt(&mut self, stmt: &Stmt, interner: &mut Interner) {
        match stmt {
            Stmt::Expr(expr) => {
                self.compile_expr(expr, interner);
            }

            Stmt::Assign { name, value } => {
                self.compile_expr(value, interner);
                let sym_id = interner.intern(name);
                self.code.push(Instruction::StoreName(sym_id));
                self.code.push(Instruction::Pop);
            }

            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.compile_expr(condition, interner);

                let jump_false_idx = self.code.len();
                self.code.push(Instruction::JumpIfFalse(0));

                for s in then_branch {
                    self.compile_stmt(s, interner);
                }

                match else_branch {
                    Some(else_block) => {
                        let jump_end_idx = self.code.len();
                        self.code.push(Instruction::Jump(0));

                        let else_start = self.code.len();
                        self.code[jump_false_idx] = Instruction::JumpIfFalse(else_start);

                        for s in else_block {
                            self.compile_stmt(s, interner);
                        }

                        let end_idx = self.code.len();
                        self.code[jump_end_idx] = Instruction::Jump(end_idx);
                    }
                    None => {
                        let end_idx = self.code.len();
                        self.code[jump_false_idx] = Instruction::JumpIfFalse(end_idx);
                    }
                }
            }
        }
    }

    fn compile_expr(&mut self, expr: &Expr, interner: &mut Interner) {
        match expr {
            Expr::Number(n) => {
                self.code.push(Instruction::LoadConst(Object::Int(*n)));
            }
            Expr::Bool(b) => {
                self.code.push(Instruction::LoadConst(Object::Bool(*b)));
            }
            Expr::Name(name) => {
                let sym_id = interner.intern(name);
                self.code.push(Instruction::LoadName(sym_id));
            }
            Expr::BinaryOp { left, op, right } => {
                self.compile_expr(left, interner);
                self.compile_expr(right, interner);
                match op {
                    Operator::Plus => self.code.push(Instruction::Add),
                    Operator::Minus => self.code.push(Instruction::Subtract),
                    Operator::Star => self.code.push(Instruction::Multiply),
                    Operator::Slash => self.code.push(Instruction::Divide),
                    Operator::Eq => self.code.push(Instruction::Eq),
                    Operator::NotEq => self.code.push(Instruction::NotEq),
                    Operator::Greater => self.code.push(Instruction::Greater),
                    Operator::GreaterEq => self.code.push(Instruction::GreaterEq),
                    Operator::Less => self.code.push(Instruction::Less),
                    Operator::LessEq => self.code.push(Instruction::LessEq),
                }
            }
        }
    }
}
