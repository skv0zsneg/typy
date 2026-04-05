use crate::parser::Expr;
use crate::parser::Operator;

#[derive(Debug, Clone)]
pub enum Instruction {
    LoadConst(i32),
    LoadName(String),
    StoreName(String),
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub fn compile(expr: &Expr) -> Vec<Instruction> {
    let mut code = Vec::new();
    compile_expr(expr, &mut code);
    code
}

fn compile_expr(expr: &Expr, code: &mut Vec<Instruction>) {
    match expr {
        Expr::Number(n) => {
            code.push(Instruction::LoadConst(*n));
        }
        Expr::Name(name) => {
            code.push(Instruction::LoadName(name.clone()));
        }
        Expr::Assign { name, value } => {
            compile_expr(value, code);
            code.push(Instruction::StoreName(name.clone()));
        }
        Expr::BinaryOp { left, op, right } => {
            compile_expr(left, code);
            compile_expr(right, code);
            match op {
                Operator::Plus => code.push(Instruction::Add),
                Operator::Minus => code.push(Instruction::Subtract),
                Operator::Star => code.push(Instruction::Multiply),
                Operator::Slash => code.push(Instruction::Divide),
            }
        }
    }
}
