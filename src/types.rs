use crate::parser::{Expr, Operator};
use crate::symbol::{Interner, SymbolId};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Int,
    Bool,
    // TODO: Float, Str, Function, etc.
}

impl Type {
    pub fn name(&self) -> &'static str {
        match self {
            Type::Int => "int",
            Type::Bool => "bool",
        }
    }
}

pub struct Checker {
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

    pub fn check(&mut self, expr: &Expr, interner: &mut Interner) -> Result<Type, String> {
        self.check_expr(expr, interner)
    }

    fn check_expr(&mut self, expr: &Expr, interner: &mut Interner) -> Result<Type, String> {
        match expr {
            Expr::Number(_) => Ok(Type::Int),

            Expr::Bool(_) => Ok(Type::Bool),

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

                match op {
                    Operator::Plus | Operator::Minus | Operator::Star | Operator::Slash => {
                        if l_type == Type::Int && r_type == Type::Int {
                            Ok(Type::Int)
                        } else {
                            Err(format!(
                                "TypeError: unsupported operand types for arithmetic: '{}' and '{}'",
                                l_type.name(),
                                r_type.name()
                            ))
                        }
                    }

                    Operator::Eq
                    | Operator::NotEq
                    | Operator::Less
                    | Operator::Greater
                    | Operator::LessEq
                    | Operator::GreaterEq => {
                        if l_type == r_type {
                            Ok(Type::Bool)
                        } else {
                            Err(format!(
                                "TypeError: cannot compare '{}' and '{}'",
                                l_type.name(),
                                r_type.name()
                            ))
                        }
                    }
                }
            }
        }
    }
}
