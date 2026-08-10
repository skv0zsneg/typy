use crate::compiler::Instruction;
use crate::object::Object;
use crate::symbol::{Interner, SymbolId};
use std::collections::HashMap;

pub struct VM {
    stack: Vec<Object>,
    globals: HashMap<SymbolId, Object>,
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: Vec::new(),
            globals: HashMap::new(),
        }
    }

    pub fn run(
        &mut self,
        bytecode: &[Instruction],
        interner: &Interner,
        debug: bool,
    ) -> Result<Object, String> {
        for (ip, instruction) in bytecode.iter().enumerate() {
            if debug {
                println!(
                    "  [IP:{:03}] {:?} | Stack: {:?}",
                    ip, instruction, self.stack
                );
            }

            match instruction {
                Instruction::LoadConst(val) => {
                    self.stack.push(val.clone());
                }

                Instruction::LoadName(sym_id) => {
                    let value = self.globals.get(sym_id).ok_or_else(|| {
                        let name = interner.resolve(*sym_id);
                        format!("NameError: name '{}' is not defined", name)
                    })?;
                    self.stack.push(value.clone());
                }

                Instruction::StoreName(sym_id) => {
                    let value = self
                        .stack
                        .last()
                        .ok_or("SystemError: stack underflow at STORE_NAME")?
                        .clone();
                    self.globals.insert(*sym_id, value);
                }

                Instruction::Add => self.binary_op(|a, b| a.add(b))?,
                Instruction::Subtract => self.binary_op(|a, b| a.sub(b))?,
                Instruction::Multiply => self.binary_op(|a, b| a.mul(b))?,
                Instruction::Divide => self.binary_op(|a, b| a.div(b))?,

                Instruction::Eq => self.compare_op(|a, b| a.eq(b))?,
                Instruction::NotEq => self.compare_op(|a, b| a.ne(b))?,
                Instruction::Greater => self.compare_op(|a, b| a.gt(b))?,
                Instruction::GreaterEq => self.compare_op(|a, b| a.ge(b))?,
                Instruction::Less => self.compare_op(|a, b| a.lt(b))?,
                Instruction::LessEq => self.compare_op(|a, b| a.le(b))?,
            }
        }

        self.stack
            .pop()
            .ok_or_else(|| "SystemError: empty stack after execution".to_string())
    }

    fn binary_op<F>(&mut self, op: F) -> Result<(), String>
    where
        F: FnOnce(&Object, &Object) -> Result<Object, String>,
    {
        let right = self
            .stack
            .pop()
            .ok_or("SystemError: stack underflow (right)")?;
        let left = self
            .stack
            .pop()
            .ok_or("SystemError: stack underflow (left)")?;
        let result = op(&left, &right)?;
        self.stack.push(result);
        Ok(())
    }

    fn compare_op<F>(&mut self, op: F) -> Result<(), String>
    where
        F: FnOnce(&Object, &Object) -> Result<bool, String>,
    {
        let right = self
            .stack
            .pop()
            .ok_or("SystemError: stack underflow (right)")?;
        let left = self
            .stack
            .pop()
            .ok_or("SystemError: stack underflow (left)")?;
        let result = op(&left, &right)?;
        self.stack.push(Object::Bool(result));
        Ok(())
    }
}
