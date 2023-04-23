#[derive(Debug, PartialEq)]
pub enum Inst {
    Nop,
    Stop,
    Ld8(Arg8, Arg8),
    Ld16(Arg16, Arg16),
}

#[derive(Debug, PartialEq)]
pub enum Arg8 {
    Reg(Reg8),
    Immed(u8),
    Ind(Arg16),
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
