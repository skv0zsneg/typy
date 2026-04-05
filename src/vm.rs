use crate::compiler::Instruction;

pub fn run_vm(bytecode: &[Instruction], debug: bool) -> Result<i32, String> {
    let mut stack = Vec::new();

    for (ip, instruction) in bytecode.iter().enumerate() {
        match instruction {
            Instruction::LoadConst(n) => stack.push(*n),
            Instruction::Add => {
                let b = stack.pop().ok_or("Stack underflow at ADD")?;
                let a = stack.pop().ok_or("Stack underflow at ADD")?;
                stack.push(a + b);
            }
            Instruction::Subtract => {
                let b = stack.pop().ok_or("Stack underflow at SUB")?;
                let a = stack.pop().ok_or("Stack underflow at SUB")?;
                stack.push(a - b);
            }
            Instruction::Multiply => {
                let b = stack.pop().ok_or("Stack underflow at MUL")?;
                let a = stack.pop().ok_or("Stack underflow at MUL")?;
                stack.push(a * b);
            }
            Instruction::Divide => {
                let b = stack.pop().ok_or("Stack underflow at DIV")?;
                let a = stack.pop().ok_or("Stack underflow at DIV")?;
                if b == 0 {
                    return Err("Division by zero".to_string());
                }
                stack.push(a / b);
            }
        }
        if debug {
            println!("  [IP:{}] Stack: {:?}", ip, stack);
        }
    }

    if stack.len() == 1 {
        Ok(stack[0])
    } else {
        Err(format!("Invalid stack state: {:?}", stack))
    }
}
