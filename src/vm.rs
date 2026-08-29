use crate::compiler::Instruction;
use crate::object::Object;
use crate::symbol::{Interner, SymbolId};
use std::collections::HashMap;

#[derive(Debug)]
pub struct CallFrame {
    locals: Vec<Object>,
    _ip_offset: usize, // Not using by now
}

impl CallFrame {
    pub fn new(ip_offset: usize, num_locals: usize) -> Self {
        Self {
            locals: vec![Object::None; num_locals],
            _ip_offset: ip_offset,
        }
    }
}

pub struct VM {
    stack: Vec<Object>,
    frames: Vec<CallFrame>,
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
            frames: vec![CallFrame::new(0, 0)],
            globals: HashMap::new(),
        }
    }

    fn current_frame(&mut self) -> &mut CallFrame {
        self.frames.last_mut().unwrap()
    }

    pub fn run(
        &mut self,
        bytecode: &[Instruction],
        interner: &Interner,
        debug: bool,
    ) -> Result<Object, String> {
        let mut ip = 0;

        while ip < bytecode.len() {
            let instruction = &bytecode[ip];

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

                Instruction::LoadLocal(slot) => {
                    let frame = self.current_frame();
                    if *slot >= frame.locals.len() {
                        return Err(format!("SystemError: local slot {} out of bounds", slot));
                    }
                    let value = frame.locals[*slot].clone();
                    self.stack.push(value);
                }

                Instruction::StoreLocal(slot) => {
                    let value = self
                        .stack
                        .pop()
                        .ok_or("SystemError: stack underflow at STORE_LOCAL")?;

                    let frame = self.current_frame();
                    if *slot >= frame.locals.len() {
                        return Err(format!("SystemError: local slot {} out of bounds", slot));
                    }
                    frame.locals[*slot] = value;
                }

                Instruction::EnterBlock(num_locals) => {
                    self.frames.push(CallFrame::new(ip, *num_locals));
                }

                Instruction::ExitBlock => {
                    if self.frames.len() <= 1 {
                        return Err("SystemError: cannot exit global frame".to_string());
                    }
                    self.frames.pop();
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

                Instruction::Jump(target) => {
                    ip = *target;
                    continue;
                }

                Instruction::JumpIfFalse(target) => {
                    let condition = self
                        .stack
                        .pop()
                        .ok_or("SystemError: stack underflow at JUMP_IF_FALSE")?;

                    match condition {
                        Object::Bool(false) => {
                            ip = *target;
                            continue;
                        }
                        Object::Bool(true) => {}
                        _ => {
                            return Err(format!(
                                "TypeError: condition must be bool, got {}",
                                condition.type_name()
                            ));
                        }
                    }
                }
            }
            ip += 1;

            if debug {
                println!(
                    "  [IP:{:03}] {:?} | Stack: {:?} | Globals: {:?} | Frames ({}): {:?}",
                    ip,
                    instruction,
                    self.stack,
                    self.globals,
                    self.frames.len(),
                    self.frames,
                );
            }
        }
        Ok(self.stack.pop().unwrap_or(Object::None))
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
