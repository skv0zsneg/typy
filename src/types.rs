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

pub struct TypeChecker {
    scopes: Vec<HashMap<SymbolId, Type>>,
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            scopes: vec![HashMap::new()],
        }
    }

    fn current_scope(&mut self) -> &mut HashMap<SymbolId, Type> {
        self.scopes.last_mut().unwrap()
    }

    fn enter_block(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn exit_block(&mut self) {
        self.scopes.pop();
    }

    fn resolve(&self, sym_id: SymbolId) -> Option<&Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(typ) = scope.get(&sym_id) {
                return Some(typ);
            }
        }
        None
    }

    fn declare(&mut self, sym_id: SymbolId, typ: Type, interner: &Interner) -> Result<(), String> {
        let current = self.current_scope();
        if current.contains_key(&sym_id) {
            return Err(format!(
                "Variable '{}' already declared in this scope",
                interner.resolve(sym_id)
            ));
        }
        current.insert(sym_id, typ);
        Ok(())
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

            Stmt::VariableDecl {
                name,
                typ,
                initializer,
            } => {
                let sym_id = interner.intern(name);

                if let Some(init) = initializer {
                    let init_type = self.check_expr(init, interner)?;
                    if init_type != *typ {
                        return Err(format!(
                            "TypeError: mismatched types '{}' expected to be '{}', got '{}'",
                            name,
                            typ.name(),
                            init_type.name()
                        ));
                    }
                }

                self.declare(sym_id, *typ, interner)?;
                Ok(())
            }

            Stmt::Assign { name, value } => {
                let sym_id = interner.intern(name);
                let expected_type = match self.resolve(sym_id) {
                    Some(expected_type) => *expected_type,
                    None => return Err(format!("TypeError: variable '{}' not defined", name)),
                };

                let val_type = self.check_expr(value, interner)?;
                if expected_type != val_type {
                    return Err(format!(
                        "TypeError: mismatched types '{}' expected to be '{}', but got '{}'",
                        name,
                        expected_type.name(),
                        val_type.name()
                    ));
                }
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
                        "TypeError: if condition must be 'bool', got '{}'",
                        cond_type.name()
                    ));
                }

                self.enter_block();
                for stmt in then_branch {
                    self.check_stmt(stmt, interner)?;
                }
                self.exit_block();

                for (elif_cond, elif_branch) in elif_branches {
                    let elif_cond_type = self.check_expr(elif_cond, interner)?;
                    if elif_cond_type != Type::Bool {
                        return Err(format!(
                            "TypeError: elif condition must be 'bool', got '{}'",
                            elif_cond_type.name()
                        ));
                    }

                    self.enter_block();
                    for stmt in elif_branch {
                        self.check_stmt(stmt, interner)?;
                    }
                    self.exit_block();
                }

                if let Some(else_block) = else_branch {
                    self.enter_block();
                    for stmt in else_block {
                        self.check_stmt(stmt, interner)?;
                    }
                    self.exit_block();
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
                self.resolve(sym_id)
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
