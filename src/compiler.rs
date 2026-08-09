use crate::parser::{Expr, Operator};
use crate::symbol::Interner;
use crate::symbol::SymbolId;
use crate::value::Value;

#[derive(Debug, Clone)]
pub enum Instruction {
    LoadConst(Value),
    LoadName(SymbolId),
    StoreName(SymbolId),
    Add,
    Subtract,
    Multiply,
    Divide,
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

    // interner передается как аргумент
    pub fn compile(mut self, expr: &Expr, interner: &mut Interner) -> Vec<Instruction> {
        self.compile_expr(expr, interner);
        self.code
    }

    fn compile_expr(&mut self, expr: &Expr, interner: &mut Interner) {
        match expr {
            Expr::Number(n) => {
                self.code
                    .push(Instruction::LoadConst(Value::Int(*n )));
            }
            Expr::Name(name) => {
                let sym_id = interner.intern(name);
                self.code.push(Instruction::LoadName(sym_id));
            }
            Expr::Assign { name, value } => {
                self.compile_expr(value, interner);
                let sym_id = interner.intern(name);
                self.code.push(Instruction::StoreName(sym_id));
            }
            Expr::BinaryOp { left, op, right } => {
                self.compile_expr(left, interner);
                self.compile_expr(right, interner);
                match op {
                    Operator::Plus => self.code.push(Instruction::Add),
                    Operator::Minus => self.code.push(Instruction::Subtract),
                    Operator::Star => self.code.push(Instruction::Multiply),
                    Operator::Slash => self.code.push(Instruction::Divide),
                }
            }
        }
    }
}
