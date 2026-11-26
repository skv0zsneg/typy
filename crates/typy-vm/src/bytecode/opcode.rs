#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    /// Push signed 32-bit integer to stack.
    ///
    /// # Stack effect
    ///
    /// `[...] -> [... , number]`
    ///
    /// # Encoding `0x01`
    ///
    /// ```
    /// use typy_vm::bytecode::Opcode;
    ///
    /// assert_eq!(Opcode::PushI32 as u8, 0x01);
    /// ```
    PushI32 = 0x01,

    /// Adds two signed 32-bit integers.
    ///
    /// # Stack effect
    ///
    /// `[... , lhs, rhs] -> [... , result]`
    ///
    /// # Encoding `0x02`
    ///
    /// ```
    /// use typy_vm::bytecode::Opcode;
    ///
    /// assert_eq!(Opcode::AddI32 as u8, 0x02);
    /// ```
    AddI32 = 0x02,

    /// Subtracts two signed 32-bit integers.
    ///
    /// # Stack effect
    ///
    /// `[... , lhs, rhs] -> [... , result]`
    ///
    /// # Encoding `0x03`
    ///
    /// ```
    /// use typy_vm::bytecode::Opcode;
    ///
    /// assert_eq!(Opcode::SubI32 as u8, 0x03);
    /// ```
    SubI32 = 0x03,

    /// Multiplies two signed 32-bit integers.
    ///
    /// # Stack effect
    ///
    /// `[... , lhs, rhs] -> [... , result]`
    ///
    /// # Encoding `0x04`
    ///
    /// ```
    /// use typy_vm::bytecode::Opcode;
    ///
    /// assert_eq!(Opcode::MulI32 as u8, 0x04);
    /// ```
    MulI32 = 0x04,

    /// Halts the virtual machine.
    ///
    /// # Stack effect
    ///
    /// `[...] -> [...]`
    ///
    /// # Encoding `0xff`
    ///
    /// ```
    /// use typy_vm::bytecode::Opcode;
    ///
    /// assert_eq!(Opcode::Halt as u8, 0xff);
    /// ```
    Halt = 0xFF,
}
