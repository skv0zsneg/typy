use crate::object::Object;
use crate::parser::{Expr, Operator, Stmt};
use crate::symbol::{Interner, SymbolId};
use crate::types::Type;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum Instruction {
    LoadConst(Object),

    LoadName(SymbolId),
    StoreName(SymbolId),

    LoadLocal(usize),
    StoreLocal(usize),

    EnterBlock(usize),
    ExitBlock,

    Add,
    Subtract,
    Multiply,
    Divide,

    Eq,
    NotEq,
    Less,
    Greater,
    LessEq,
    GreaterEq,

    Jump(usize),
    JumpIfFalse(usize),
}

pub struct CompilerScope {
    locals: Vec<SymbolId>,
}

pub struct Compiler {
    scopes: Vec<CompilerScope>,
    code: Vec<Instruction>,
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            scopes: vec![CompilerScope { locals: Vec::new() }],
            code: Vec::new(),
        }
    }

    // Helpers

    fn emit(&mut self, instruction: Instruction) {
        self.code.push(instruction);
    }

    fn current_address(&self) -> usize {
        self.code.len()
    }

    fn emit_jump_placeholder(&mut self, instruction: Instruction) -> usize {
        let idx = self.code.len();
        self.code.push(instruction);
        idx
    }

    fn patch_jump(&mut self, idx: usize, target: usize) {
        match &mut self.code[idx] {
            Instruction::Jump(t) => *t = target,
            Instruction::JumpIfFalse(t) => *t = target,
            _ => panic!("Cannot patch non-jump instruction"),
        }
    }

    // Scope management

    fn enter_scope(&mut self) {
        self.scopes.push(CompilerScope { locals: Vec::new() });
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare_local(&mut self, sym_id: SymbolId) -> usize {
        let scope = self.scopes.last_mut().unwrap();
        let index = scope.locals.len();
        scope.locals.push(sym_id);
        index
    }

    fn resolve_local(&self, sym_id: SymbolId) -> Option<usize> {
        let scope = self.scopes.last().unwrap();
        scope.locals.iter().position(|&id| id == sym_id)
    }

    fn is_in_block(&self) -> bool {
        self.scopes.len() > 1
    }

    fn count_block_locals(&self, stmts: &[Stmt], interner: &mut Interner) -> usize {
        let mut seen = HashSet::new();
        for stmt in stmts {
            if let Stmt::VariableDecl { name, .. } = stmt {
                let sym_id = interner.intern(name);
                seen.insert(sym_id);
            }
        }
        seen.len()
    }

    // Block Comlile

    fn compile_block(&mut self, stmts: &[Stmt], interner: &mut Interner) {
        let num_locals = self.count_block_locals(stmts, interner);
        self.emit(Instruction::EnterBlock(num_locals));

        self.enter_scope();
        for stmt in stmts {
            self.compile_stmt(stmt, interner);
        }
        self.exit_scope();

        self.emit(Instruction::ExitBlock);
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
                    match typ {
                        Type::Int => self.emit(Instruction::LoadConst(Object::Int(0))),
                        Type::Bool => self.emit(Instruction::LoadConst(Object::Bool(false))),
                    }
                }

                if self.is_in_block() {
                    let slot = self.declare_local(sym_id);
                    self.emit(Instruction::StoreLocal(slot));
                } else {
                    self.emit(Instruction::StoreName(sym_id));
                }
            }

            Stmt::Assign { name, value } => {
                self.compile_expr(value, interner);
                let sym_id = interner.intern(name);

                if let Some(slot) = self.resolve_local(sym_id) {
                    self.emit(Instruction::StoreLocal(slot));
                } else {
                    self.emit(Instruction::StoreName(sym_id));
                }
            }

            Stmt::If {
                condition,
                then_branch,
                elif_branches,
                else_branch,
            } => {
                let mut end_jumps = Vec::new();

                self.compile_expr(condition, interner);
                let mut false_jump_idx = self.emit_jump_placeholder(Instruction::JumpIfFalse(0));

                self.compile_block(then_branch, interner);

                let jump_end_idx = self.emit_jump_placeholder(Instruction::Jump(0));
                end_jumps.push(jump_end_idx);

                for (elif_condition, elif_branch) in elif_branches {
                    let elif_start = self.current_address();
                    self.patch_jump(false_jump_idx, elif_start);

                    self.compile_expr(elif_condition, interner);
                    false_jump_idx = self.emit_jump_placeholder(Instruction::JumpIfFalse(0));

                    self.compile_block(elif_branch, interner);

                    let jump_end_idx = self.emit_jump_placeholder(Instruction::Jump(0));
                    end_jumps.push(jump_end_idx);
                }

                let fallback_start = self.current_address();
                self.patch_jump(false_jump_idx, fallback_start);

                if let Some(else_block) = else_branch {
                    self.compile_block(else_block, interner);
                }

                let end = self.current_address();
                for jump_idx in end_jumps {
                    self.patch_jump(jump_idx, end);
                }
            }
        }
    }

    fn compile_expr(&mut self, expr: &Expr, interner: &mut Interner) {
        match expr {
            Expr::Number(n) => {
                self.emit(Instruction::LoadConst(Object::Int(*n)));
            }
            Expr::Bool(b) => {
                self.emit(Instruction::LoadConst(Object::Bool(*b)));
            }
            Expr::Name(name) => {
                let sym_id = interner.intern(name);
                if let Some(slot) = self.resolve_local(sym_id) {
                    self.emit(Instruction::LoadLocal(slot));
                } else {
                    self.emit(Instruction::LoadName(sym_id));
                }
            }
            Expr::BinaryOp { left, op, right } => {
                self.compile_expr(left, interner);
                self.compile_expr(right, interner);
                match op {
                    Operator::Plus => self.emit(Instruction::Add),
                    Operator::Minus => self.emit(Instruction::Subtract),
                    Operator::Star => self.emit(Instruction::Multiply),
                    Operator::Slash => self.emit(Instruction::Divide),
                    Operator::Eq => self.emit(Instruction::Eq),
                    Operator::NotEq => self.emit(Instruction::NotEq),
                    Operator::Greater => self.emit(Instruction::Greater),
                    Operator::GreaterEq => self.emit(Instruction::GreaterEq),
                    Operator::Less => self.emit(Instruction::Less),
                    Operator::LessEq => self.emit(Instruction::LessEq),
                }
            }
        }
    }
}
