use crate::object::Object;
use crate::parser::{Expr, Operator, Stmt};
use crate::symbol::{Interner, SymbolId};
use crate::types::Type;

#[derive(Debug, Clone)]
pub enum Instruction {
    // Vars & constants
    LoadConst(Object),
    LoadName(SymbolId),
    StoreName(SymbolId),

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

            Stmt::VariableDecl {
                name,
                typ,
                initializer,
            } => {
                let sym_id = interner.intern(name);

                if let Some(init) = initializer {
                    self.compile_expr(init, interner);
                } else {
                    // Zero-Initialization
                    match typ {
                        Type::Int => self.code.push(Instruction::LoadConst(Object::Int(0))),
                        Type::Bool => self.code.push(Instruction::LoadConst(Object::Bool(false))),
                    }
                }

                self.code.push(Instruction::StoreName(sym_id));
            }

            Stmt::Assign { name, value } => {
                self.compile_expr(value, interner);
                let sym_id = interner.intern(name);
                self.code.push(Instruction::StoreName(sym_id));
            }

            Stmt::If {
                condition,
                then_branch,
                elif_branches,
                else_branch,
            } => {
                let mut end_jumps = Vec::new();

                self.compile_expr(condition, interner);
                let mut false_jump_idx = self.code.len();
                self.code.push(Instruction::JumpIfFalse(0));

                for stmt in then_branch {
                    self.compile_stmt(stmt, interner);
                }

                let jump_end_idx = self.code.len();
                self.code.push(Instruction::Jump(0));
                end_jumps.push(jump_end_idx);

                for (elif_condition, elif_branch) in elif_branches {
                    let elif_start = self.code.len();
                    self.code[false_jump_idx] = Instruction::JumpIfFalse(elif_start);

                    self.compile_expr(elif_condition, interner);
                    false_jump_idx = self.code.len();
                    self.code.push(Instruction::JumpIfFalse(0));

                    for stmt in elif_branch {
                        self.compile_stmt(stmt, interner);
                    }

                    let jump_end_idx = self.code.len();
                    self.code.push(Instruction::Jump(0));
                    end_jumps.push(jump_end_idx);
                }

                let fallback_start = self.code.len();
                self.code[false_jump_idx] = Instruction::JumpIfFalse(fallback_start);

                if let Some(else_block) = else_branch {
                    for stmt in else_block {
                        self.compile_stmt(stmt, interner);
                    }
                }

                let end = self.code.len();
                for jump_idx in end_jumps {
                    self.code[jump_idx] = Instruction::Jump(end);
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
