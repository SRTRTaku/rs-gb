#[derive(Debug, PartialEq)]
pub enum Inst {
    Ld8(Arg8, Arg8),
    Ld16(Arg16, Arg16),
    // 8-bit Arithmetic/ Logic instructions
    Inc8(Arg8),
    Dec8(Arg8),
    // 16-bit Arithmetic/ Logic instructions
    Add16(Arg16, Arg16),
    Inc16(Arg16),
    Dec16(Arg16),
    // Rotate and Shift instructions
    Rlca,
    Rla,
    Rrca,
    Rra,
    // Single-bit Operation instructions
    // CPU Control instructions
    Nop,
    Stop,
    // Jump instructions
}

#[derive(Debug, PartialEq)]
pub enum Arg8 {
    Reg(Reg8),
    Immed(u8),
    Ind(u16),
    IndReg(Reg16),
    IndIncHL,
    IndDecHL,
    Io(u8), // FF00 + n
    IndIoC, // FF00 + C
}

#[derive(PartialEq, Debug)]
pub enum Reg8 {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug, PartialEq)]
pub enum Arg16 {
    Reg(Reg16),
    Immed(u16),
    Ind(u16),
}

#[derive(Debug, PartialEq)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    CP,
}

enum FlagRegister {
    Zero,        // n
    Subtraction, // n,
    HalfCarry,   // h
    Carry,       // c
}
