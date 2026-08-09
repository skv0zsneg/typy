use crate::parser::{Expr, Operator};
use crate::symbol::{Interner, SymbolId};
use crate::types::Type;
use std::collections::HashMap;

pub struct Checker {
    // Храним только окружение (типы переменных)
    env: HashMap<SymbolId, Type>,
}

impl Default for Checker {
    fn default() -> Self {
        Self::new()
    }
}

impl Checker {
    pub fn new() -> Self {
        Checker {
            env: HashMap::new(),
        }
    }

    // Теперь interner передается как аргумент
    pub fn check(&mut self, expr: &Expr, interner: &mut Interner) -> Result<Type, String> {
        self.check_expr(expr, interner)
    }

    fn check_expr(&mut self, expr: &Expr, interner: &mut Interner) -> Result<Type, String> {
        match expr {
            Expr::Number(_) => Ok(Type::Int),

            Expr::Name(name) => {
                let sym_id = interner.intern(name);
                self.env
                    .get(&sym_id)
                    .copied()
                    .ok_or_else(|| format!("NameError: name '{}' is not defined", name))
            }

            Expr::Assign { name, value } => {
                let val_type = self.check_expr(value, interner)?;
                let sym_id = interner.intern(name);
                self.env.insert(sym_id, val_type);
                Ok(val_type)
            }

            Expr::BinaryOp { left, op, right } => {
                let l_type = self.check_expr(left, interner)?;
                let r_type = self.check_expr(right, interner)?;

                match (l_type, r_type, op) {
                    (Type::Int, Type::Int, Operator::Plus) => Ok(Type::Int),
                    (Type::Int, Type::Int, Operator::Minus) => Ok(Type::Int),
                    (Type::Int, Type::Int, Operator::Star) => Ok(Type::Int),
                    (Type::Int, Type::Int, Operator::Slash) => Ok(Type::Int),
                    _ => Err(format!(
                        "TypeError: unsupported operand types for {:?}: '{}' and '{}'",
                        op,
                        l_type.name(),
                        r_type.name()
                    )),
                }
            }
        }
    }
}
