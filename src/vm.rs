use std::collections::HashMap;

use crate::compiler::Instruction;

pub struct VM {
    stack: Vec<i32>,
    globals: HashMap<String, i32>,
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: Vec::new(),
            globals: HashMap::new(),
        }
    }

    pub fn run(&mut self, bytecode: &[Instruction], debug: bool) -> Result<i32, String> {
        for (ip, instruction) in bytecode.iter().enumerate() {
            match instruction {
                Instruction::LoadConst(n) => self.stack.push(*n),

                Instruction::LoadName(name) => {
                    let value = self
                        .globals
                        .get(name)
                        .ok_or_else(|| format!("NameError: '{}' is not defined", name))?;
                    self.stack.push(*value);
                }

                Instruction::StoreName(name) => {
                    let value = self.stack.pop().ok_or("Stack underflow at STORE")?;
                    self.globals.insert(name.clone(), value);
                }

                Instruction::Add => {
                    let b = self.stack.pop().ok_or("Stack underflow at ADD")?;
                    let a = self.stack.pop().ok_or("Stack underflow at ADD")?;
                    self.stack.push(a + b);
                }

                Instruction::Subtract => {
                    let b = self.stack.pop().ok_or("Stack underflow at SUB")?;
                    let a = self.stack.pop().ok_or("Stack underflow at SUB")?;
                    self.stack.push(a - b);
                }

                Instruction::Multiply => {
                    let b = self.stack.pop().ok_or("Stack underflow at MUL")?;
                    let a = self.stack.pop().ok_or("Stack underflow at MUL")?;
                    self.stack.push(a * b);
                }

                Instruction::Divide => {
                    let b = self.stack.pop().ok_or("Stack underflow at DIV")?;
                    let a = self.stack.pop().ok_or("Stack underflow at DIV")?;
                    if b == 0 {
                        return Err("Division by zero".to_string());
                    }
                    self.stack.push(a / b);
                }
            }
            if debug {
                println!(
                    "  [IP:{}] Stack: {:?}, Globals: {:?}",
                    ip, self.stack, self.globals
                );
            }
        }

        if self.stack.len() == 1 {
            Ok(self.stack[0])
        } else {
            Err(format!("Invalid stack state: {:?}", self.stack))
        }
    }
}

pub fn run_vm(bytecode: &[Instruction], debug: bool) -> Result<i32, String> {
    let mut vm = VM::new();
    vm.run(bytecode, debug)
}
