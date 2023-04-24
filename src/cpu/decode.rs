use super::inst::{Arg16, Arg8, Inst, Reg16, Reg8};
use crate::memory::MemoryIF;

pub fn decode(pc: u16, memory: &impl MemoryIF) -> (Inst, u16) {
    let mut addvance = 1;
    let inst = match memory.read_byte(pc) {
        0x00 => Inst::Nop,
        0x01 => {
            let nn = memory.read_word(pc + 1);
            addvance = 3;
            Inst::Ld16(Arg16::Reg(Reg16::BC), Arg16::Immed(nn))
        }
        0x02 => Inst::Ld8(Arg8::IndReg(Reg16::BC), Arg8::Reg(Reg8::A)),
        0x03 => Inst::Inc16(Arg16::Reg(Reg16::BC)),
        0x04 => Inst::Inc8(Arg8::Reg(Reg8::B)),
        0x05 => Inst::Dec8(Arg8::Reg(Reg8::B)),
        0x06 => {
            let n = memory.read_byte(pc + 1);
            addvance = 2;
            Inst::Ld8(Arg8::Reg(Reg8::B), Arg8::Immed(n))
        }
        0x07 => Inst::Rlca,
        0x08 => {
            let nn = memory.read_word(pc + 1);
            addvance = 3;
            Inst::Ld16(Arg16::Ind(nn), Arg16::Reg(Reg16::SP))
        }
        0x09 => Inst::Add16(Arg16::Reg(Reg16::HL), Arg16::Reg(Reg16::BC)),
        0x0a => Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::IndReg(Reg16::BC)),
        0x0b => Inst::Dec16(Arg16::Reg(Reg16::BC)),
        0x0c => Inst::Inc8(Arg8::Reg(Reg8::C)),
        0x0d => Inst::Dec8(Arg8::Reg(Reg8::C)),
        0x0e => {
            let n = memory.read_byte(pc + 1);
            addvance = 2;
            Inst::Ld8(Arg8::Reg(Reg8::C), Arg8::Immed(n))
        }
        0x0f => Inst::Rrca,
        0x10 => Inst::Stop,
        0x11 => {
            let nn = memory.read_word(pc + 1);
            addvance = 3;
            Inst::Ld16(Arg16::Reg(Reg16::DE), Arg16::Immed(nn))
        }
        0x12 => Inst::Ld8(Arg8::IndReg(Reg16::DE), Arg8::Reg(Reg8::A)),
        0x13 => Inst::Inc16(Arg16::Reg(Reg16::DE)),
        0x14 => Inst::Inc8(Arg8::Reg(Reg8::D)),
        0x15 => Inst::Dec8(Arg8::Reg(Reg8::D)),
        0x16 => {
            let n = memory.read_byte(pc + 1);
            addvance = 2;
            Inst::Ld8(Arg8::Reg(Reg8::D), Arg8::Immed(n))
        }
        0x17 => Inst::Rla,
        0x18 => {
            let n = memory.read_byte(pc + 1);
            addvance = 2;
            Inst::Jr(n as i8)
        }
        0x19 => Inst::Add16(Arg16::Reg(Reg16::HL), Arg16::Reg(Reg16::DE)),
        0x1a => Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::IndReg(Reg16::DE)),
        0x1b => Inst::Dec16(Arg16::Reg(Reg16::DE)),
        0x1c => Inst::Inc8(Arg8::Reg(Reg8::E)),
        0x1d => Inst::Dec8(Arg8::Reg(Reg8::E)),
        0x1e => {
            let n = memory.read_byte(pc + 1);
            addvance = 2;
            Inst::Ld8(Arg8::Reg(Reg8::E), Arg8::Immed(n))
        }
        0x1f => Inst::Rra,
        /*
        0x00 => todo!(),
        0x01 => todo!(),
        0x02 => todo!(),
        0x03 => todo!(),
        0x04 => todo!(),
        0x05 => todo!(),
        0x06 => todo!(),
        0x07 => todo!(),
        0x08 => todo!(),
        0x09 => todo!(),
        0x0a => todo!(),
        0x0b => todo!(),
        0x0c => todo!(),
        0x0d => todo!(),
        0x0e => todo!(),
        0x0f => todo!(),
        */
        _ => todo!(),
    };
    (inst, addvance)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestMemory {
        memory: [u8; 0x200],
    }
    impl TestMemory {
        fn new() -> TestMemory {
            TestMemory { memory: [0; 0x200] }
        }
    }
    impl MemoryIF for TestMemory {
        fn read_byte(&self, addr: u16) -> u8 {
            self.memory[addr as usize]
        }
        fn read_word(&self, addr: u16) -> u16 {
            let h = self.memory[addr as usize] as u16;
            let l = self.memory[addr as usize + 1] as u16;
            (h << 8) | l
        }
        fn write_byte(&mut self, addr: u16, val: u8) {
            self.memory[addr as usize] = val;
        }
        fn write_word(&mut self, addr: u16, val: u16) {
            let h = (val >> 8) as u8;
            let l = (val & 0x00ff) as u8;
            self.memory[addr as usize] = h;
            self.memory[addr as usize + 1] = l;
        }
    }

    //
    // 0x00
    //
    #[test]
    fn decode_nop() {
        let m = TestMemory::new();
        let pc = 0x0100;
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Nop, i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_bc_u16() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x01);
        m.write_word(pc + 1, 0x1234);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld16(Arg16::Reg(Reg16::BC), Arg16::Immed(0x1234)), i);
        assert_eq!(3, a);
    }
    #[test]
    fn decode_ld_pbc_a() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x02);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::IndReg(Reg16::BC), Arg8::Reg(Reg8::A)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_inc_bc() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x03);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Inc16(Arg16::Reg(Reg16::BC)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_inc_b() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x04);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Inc8(Arg8::Reg(Reg8::B)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_dec_b() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x05);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Dec8(Arg8::Reg(Reg8::B)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_b_u8() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x06);
        m.write_byte(pc + 1, 0x12);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::B), Arg8::Immed(0x12)), i);
        assert_eq!(2, a);
    }
    #[test]
    fn decode_rlca() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x07);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Rlca, i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_pu16_sp() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x08);
        m.write_word(pc + 1, 0x0200);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld16(Arg16::Ind(0x0200), Arg16::Reg(Reg16::SP)), i);
        assert_eq!(3, a);
    }
    #[test]
    fn decode_add_hl_bc() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x09);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Add16(Arg16::Reg(Reg16::HL), Arg16::Reg(Reg16::BC)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_a_pbc() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x0a);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::IndReg(Reg16::BC)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_dec_bc() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x0b);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Dec16(Arg16::Reg(Reg16::BC)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_inc_c() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x0c);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Inc8(Arg8::Reg(Reg8::C)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_dec_c() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x0d);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Dec8(Arg8::Reg(Reg8::C)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_c_u8() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x0e);
        m.write_byte(pc + 1, 0x12);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::C), Arg8::Immed(0x12)), i);
        assert_eq!(2, a);
    }
    #[test]
    fn decode_rrca() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x0f);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Rrca, i);
        assert_eq!(1, a);
    }
    //
    // 0x10
    //
    #[test]
    fn decode_stop() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x10);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Stop, i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_de_u16() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x11);
        m.write_word(pc + 1, 0x1234);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld16(Arg16::Reg(Reg16::DE), Arg16::Immed(0x1234)), i);
        assert_eq!(3, a);
    }
    #[test]
    fn decode_ld_pde_a() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x12);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::IndReg(Reg16::DE), Arg8::Reg(Reg8::A)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_inc_de() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x13);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Inc16(Arg16::Reg(Reg16::DE)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_inc_d() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x14);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Inc8(Arg8::Reg(Reg8::D)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_dec_d() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x15);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Dec8(Arg8::Reg(Reg8::D)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_d_u8() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x16);
        m.write_byte(pc + 1, 0x12);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::D), Arg8::Immed(0x12)), i);
        assert_eq!(2, a);
    }
    #[test]
    fn decode_rla() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x17);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Rla, i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_jr_i8() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x18);
        m.write_byte(pc + 1, 0xff);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Jr(-1), i);
        assert_eq!(2, a);
    }
    #[test]
    fn decode_add_hl_de() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x19);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Add16(Arg16::Reg(Reg16::HL), Arg16::Reg(Reg16::DE)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_a_pde() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x1a);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::IndReg(Reg16::DE)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_dec_de() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x1b);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Dec16(Arg16::Reg(Reg16::DE)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_inc_e() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x1c);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Inc8(Arg8::Reg(Reg8::E)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_dec_e() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x1d);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Dec8(Arg8::Reg(Reg8::E)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_e_u8() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x1e);
        m.write_byte(pc + 1, 0x12);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::E), Arg8::Immed(0x12)), i);
        assert_eq!(2, a);
    }
    #[test]
    fn decode_rra() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x1f);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Rra, i);
        assert_eq!(1, a);
    }
}
