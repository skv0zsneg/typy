use crate::parser::{Expr, Operator, Stmt};
use crate::symbol::{Interner, SymbolId};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Int,
    Bool,
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

    pub fn check(&mut self, stmts: &[Stmt], interner: &mut Interner) -> Result<(), String> {
        for stmt in stmts {
            self.check_stmt(stmt, interner)?;
        }
        Ok(())
    }

    fn check_stmt(&mut self, stmt: &Stmt, interner: &mut Interner) -> Result<(), String> {
        match stmt {
            Stmt::Expr(expr) => {
                self.check_expr(expr, interner)?;
                Ok(())
            }

            Stmt::Assign { name, value } => {
                let val_type = self.check_expr(value, interner)?;
                let sym_id = interner.intern(name);
                self.env.insert(sym_id, val_type);
                Ok(())
            }

            Stmt::If {
                condition,
                then_branch,
                elif_branches,
                else_branch,
            } => {
                let cond_type = self.check_expr(condition, interner)?;
                if cond_type != Type::Bool {
                    return Err(format!(
                        "TypeError: if condition must be bool, got '{}'",
                        cond_type.name()
                    ));
                }

                for (condition, branch) in elif_branches {
                    let cond_type = self.check_expr(condition, interner)?;
                    if cond_type != Type::Bool {
                        return Err(format!(
                            "TypeError: if condition must be bool, got '{}'",
                            cond_type.name()
                        ));
                    }
                    for stmt in branch {
                        self.check_stmt(stmt, interner)?;
                    }
                }

                for s in then_branch {
                    self.check_stmt(s, interner)?;
                }

                if let Some(else_block) = else_branch {
                    for s in else_block {
                        self.check_stmt(s, interner)?;
                    }
                }

                Ok(())
            }
        }
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
