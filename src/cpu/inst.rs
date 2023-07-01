#[derive(Debug, PartialEq)]
pub enum Inst {
    Ld8(Arg8, Arg8),
    Ld16(Arg16, Arg16),
    Push16(Reg16),
    Pop16(Reg16),
    // 8-bit Arithmetic/ Logic instructions
    Add(Arg8, Arg8),
    Adc(Arg8, Arg8),
    Sub(Arg8, Arg8),
    Sbc(Arg8, Arg8),
    And(Arg8, Arg8),
    Xor(Arg8, Arg8),
    Or(Arg8, Arg8),
    Cp(Arg8, Arg8),
    Inc(Arg8),
    Dec(Arg8),
    Daa,
    Cpl,
    // 16-bit Arithmetic/ Logic instructions
    Add16(Arg16, Arg16),
    Inc16(Arg16),
    Dec16(Arg16),
    Add16SP(i8),
    Ld16HLSP(i8),
    // Rotate and Shift instructions
    Rlca,
    Rla,
    Rrca,
    Rra,
    Rlc(Arg8),
    Rl(Arg8),
    Rrc(Arg8),
    Rr(Arg8),
    Sla(Arg8),
    Swap(Arg8),
    Sra(Arg8),
    Srl(Arg8),
    // Sighle-bit Operation instructions
    Bit(u8, Arg8),
    Set(u8, Arg8),
    Res(u8, Arg8),
    // CPU Control instructions
    Ccf,
    Scf,
    Nop,
    Halt,
    Stop,
    Di, // disable interrupts, IME = 0
    Ei, // enable interrupts, IME = 1
    // Jump instructions
    Jp(u16),
    JpHL,
    Jpf(JpFlag, u16),
    Jr(i8),
    Jrf(JpFlag, i8),
    Call(u16),
    Callf(JpFlag, u16),
    Ret,
    Retf(JpFlag),
    Reti,
    Rst(u8),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Arg8 {
    Reg(Reg8),
    Immed(u8),
    Ind(u16),
    IndReg(Reg16),
    IndIncHL,
    IndDecHL,
    IndIo(u8), // FF00 + n
    IndIoC,    // FF00 + C
}

#[derive(Clone, Debug, PartialEq)]
pub enum Reg8 {
    A,
    // F, not specified
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

#[derive(Clone, Debug, PartialEq)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

#[derive(Debug, PartialEq)]
pub enum JpFlag {
    Nz,
    Z,
    Nc,
    C,
}

#[derive(Debug, PartialEq)]
pub enum FlagReg {
    Z, // Zero
    N, // Subtraction
    H, // HalfCarry
    C, // Carry
}
