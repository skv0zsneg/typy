use crate::compiler::Instruction;
use crate::object::Object;
use crate::symbol::{Interner, SymbolId};
use std::collections::HashMap;

/// A call frame representing a lexical scope during execution.
///
/// Each frame contains:
/// - Local variables stored in slots
/// - An instruction pointer offset (currently unused but reserved for future use)
///
/// Frames are pushed when entering a block and popped when exiting.
/// The global scope is represented by the bottom-most frame.
#[derive(Debug)]
pub struct CallFrame {
    /// Local variables stored in slots indexed from 0.
    locals: Vec<Object>,
    /// The instruction pointer offset when this frame was created.
    /// Currently unused but reserved for future stack traces or debugging.
    _ip_offset: usize,
}

impl CallFrame {
    /// Creates a new call frame with the specified number of local variable slots.
    ///
    /// All slots are initialized to `Object::None`.
    pub fn new(ip_offset: usize, num_locals: usize) -> Self {
        Self {
            locals: vec![Object::None; num_locals],
            _ip_offset: ip_offset,
        }
    }
}

/// A stack-based virtual machine that executes bytecode instructions.
///
/// The VM maintains:
/// - An operand stack for intermediate values
/// - A stack of call frames for lexical scoping
/// - A global variable table
///
/// Instructions operate on the operand stack, pushing results and popping
/// operands. Local variables are accessed via frame slots, while global
/// variables are accessed via symbol IDs.
///
/// The VM executes instructions sequentially, incrementing the instruction
/// pointer after each instruction unless a jump instruction modifies it.
pub struct VM {
    /// The operand stack for intermediate values.
    stack: Vec<Object>,
    /// A stack of call frames. The last element is the current frame.
    frames: Vec<CallFrame>,
    /// Global variables indexed by symbol ID.
    globals: HashMap<SymbolId, Object>,
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

impl VM {
    /// Creates a new virtual machine with an empty stack and a global frame.
    pub fn new() -> Self {
        VM {
            stack: Vec::new(),
            frames: vec![CallFrame::new(0, 0)],
            globals: HashMap::new(),
        }
    }

    /// Returns a mutable reference to the current (topmost) call frame.
    ///
    /// # Panics
    ///
    /// Panics if the frame stack is empty, which should never happen since
    /// the global frame is always present.
    fn current_frame(&mut self) -> &mut CallFrame {
        self.frames
            .last_mut()
            .expect("frame stack should never be empty")
    }

    /// Executes a sequence of bytecode instructions and returns the final result.
    ///
    /// The VM processes instructions sequentially, maintaining an instruction
    /// pointer that advances after each instruction unless modified by a jump.
    ///
    /// If `debug` is true, the VM prints the state after each instruction.
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
                Instruction::Jump(target) => {
                    ip = *target;
                    if debug {
                        self.print_debug_state(instruction, ip);
                    }
                    continue;
                }
                Instruction::JumpIfFalse(target) => {
                    if let Some(new_ip) = self.execute_jump_if_false(*target)? {
                        ip = new_ip;
                        if debug {
                            self.print_debug_state(instruction, ip);
                        }
                        continue;
                    }
                }
                _ => {
                    self.execute_instruction(instruction, interner, ip)?;
                }
            }

            ip += 1;

            if debug {
                self.print_debug_state(instruction, ip);
            }
        }

        Ok(self.stack.pop().unwrap_or(Object::None))
    }

    /// Executes a single instruction.
    ///
    /// This method dispatches to specialized handlers for each instruction type.
    fn execute_instruction(
        &mut self,
        instruction: &Instruction,
        interner: &Interner,
        ip: usize,
    ) -> Result<(), String> {
        match instruction {
            Instruction::LoadConst(val) => {
                self.stack.push(val.clone());
            }

            Instruction::LoadName(sym_id) => {
                self.execute_load_name(*sym_id, interner)?;
            }

            Instruction::StoreName(sym_id) => {
                self.execute_store_name(*sym_id)?;
            }

            Instruction::LoadLocal(slot) => {
                self.execute_load_local(*slot)?;
            }

            Instruction::StoreLocal(slot) => {
                self.execute_store_local(*slot)?;
            }

            Instruction::EnterBlock(num_locals) => {
                self.frames.push(CallFrame::new(ip, *num_locals));
            }

            Instruction::ExitBlock => {
                self.execute_exit_block()?;
            }

            Instruction::Add => self.execute_binary_op(|a, b| a.add(b))?,
            Instruction::Subtract => self.execute_binary_op(|a, b| a.sub(b))?,
            Instruction::Multiply => self.execute_binary_op(|a, b| a.mul(b))?,
            Instruction::Divide => self.execute_binary_op(|a, b| a.div(b))?,

            Instruction::Eq => self.execute_compare_op(|a, b| a.eq(b))?,
            Instruction::NotEq => self.execute_compare_op(|a, b| a.ne(b))?,
            Instruction::Greater => self.execute_compare_op(|a, b| a.gt(b))?,
            Instruction::GreaterEq => self.execute_compare_op(|a, b| a.ge(b))?,
            Instruction::Less => self.execute_compare_op(|a, b| a.lt(b))?,
            Instruction::LessEq => self.execute_compare_op(|a, b| a.le(b))?,

            Instruction::Jump(_target) => {
                // Jump is handled in the main loop to modify ip
                unreachable!("Jump should be handled in main loop");
            }

            Instruction::JumpIfFalse(target) => {
                self.execute_jump_if_false(*target)?;
            }
        }

        Ok(())
    }

    /// Executes a LOAD_NAME instruction.
    ///
    /// Loads a global variable by symbol ID and pushes it onto the stack.
    fn execute_load_name(&mut self, sym_id: SymbolId, interner: &Interner) -> Result<(), String> {
        let value = self.globals.get(&sym_id).ok_or_else(|| {
            let name = interner.resolve(sym_id);
            format!("NameError: name '{}' is not defined", name)
        })?;
        self.stack.push(value.clone());
        Ok(())
    }

    /// Executes a STORE_NAME instruction.
    ///
    /// Pops a value from the stack and stores it in a global variable.
    fn execute_store_name(&mut self, sym_id: SymbolId) -> Result<(), String> {
        let value = self
            .stack
            .last()
            .ok_or("SystemError: stack underflow at STORE_NAME")?
            .clone();
        self.globals.insert(sym_id, value);
        Ok(())
    }

    /// Executes a LOAD_LOCAL instruction.
    ///
    /// Loads a local variable by slot index and pushes it onto the stack.
    fn execute_load_local(&mut self, slot: usize) -> Result<(), String> {
        let frame = self.current_frame();
        if slot >= frame.locals.len() {
            return Err(format!("SystemError: local slot {} out of bounds", slot));
        }
        let value = frame.locals[slot].clone();
        self.stack.push(value);
        Ok(())
    }

    /// Executes a STORE_LOCAL instruction.
    ///
    /// Pops a value from the stack and stores it in a local variable slot.
    fn execute_store_local(&mut self, slot: usize) -> Result<(), String> {
        let value = self
            .stack
            .pop()
            .ok_or("SystemError: stack underflow at STORE_LOCAL")?;

        let frame = self.current_frame();
        if slot >= frame.locals.len() {
            return Err(format!("SystemError: local slot {} out of bounds", slot));
        }
        frame.locals[slot] = value;
        Ok(())
    }

    /// Executes an EXIT_BLOCK instruction.
    ///
    /// Pops the current call frame and returns to the parent scope.
    fn execute_exit_block(&mut self) -> Result<(), String> {
        if self.frames.len() <= 1 {
            return Err("SystemError: cannot exit global frame".to_string());
        }
        self.frames.pop();
        Ok(())
    }

    /// Executes a binary operation on the top two stack values.
    ///
    /// Pops two values, applies the operation, and pushes the result.
    fn execute_binary_op<F>(&mut self, op: F) -> Result<(), String>
    where
        F: FnOnce(&Object, &Object) -> Result<Object, String>,
    {
        let right = self
            .stack
            .pop()
            .ok_or("SystemError: stack underflow (right operand)")?;
        let left = self
            .stack
            .pop()
            .ok_or("SystemError: stack underflow (left operand)")?;
        let result = op(&left, &right)?;
        self.stack.push(result);
        Ok(())
    }

    /// Executes a comparison operation on the top two stack values.
    ///
    /// Pops two values, applies the comparison, and pushes a boolean result.
    fn execute_compare_op<F>(&mut self, op: F) -> Result<(), String>
    where
        F: FnOnce(&Object, &Object) -> Result<bool, String>,
    {
        let right = self
            .stack
            .pop()
            .ok_or("SystemError: stack underflow (right operand)")?;
        let left = self
            .stack
            .pop()
            .ok_or("SystemError: stack underflow (left operand)")?;
        let result = op(&left, &right)?;
        self.stack.push(Object::Bool(result));
        Ok(())
    }

    /// Executes a JUMP_IF_FALSE instruction.
    ///
    /// Pops a boolean from the stack and jumps to the target if it is false.
    /// Returns the new instruction pointer, or None if no jump occurred.
    fn execute_jump_if_false(&mut self, target: usize) -> Result<Option<usize>, String> {
        let condition = self
            .stack
            .pop()
            .ok_or("SystemError: stack underflow at JUMP_IF_FALSE")?;

        match condition {
            Object::Bool(false) => Ok(Some(target)),
            Object::Bool(true) => Ok(None),
            _ => Err(format!(
                "TypeError: condition must be bool, got {}",
                condition.type_name()
            )),
        }
    }

    /// Prints the VM state for debugging purposes.
    fn print_debug_state(&self, instruction: &Instruction, ip: usize) {
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
