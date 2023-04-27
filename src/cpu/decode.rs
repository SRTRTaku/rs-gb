use super::inst::{Arg16, Arg8, Inst, JpFlag, Reg16, Reg8};
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
        0x20 => {
            let n = memory.read_byte(pc + 1);
            addvance = 2;
            Inst::Jrf(JpFlag::Nz, n as i8)
        }
        0x21 => {
            let nn = memory.read_word(pc + 1);
            addvance = 3;
            Inst::Ld16(Arg16::Reg(Reg16::HL), Arg16::Immed(nn))
        }
        0x22 => Inst::Ld8(Arg8::IndIncHL, Arg8::Reg(Reg8::A)),
        0x23 => Inst::Inc16(Arg16::Reg(Reg16::HL)),
        0x24 => Inst::Inc8(Arg8::Reg(Reg8::H)),
        0x25 => Inst::Dec8(Arg8::Reg(Reg8::H)),
        0x26 => {
            let n = memory.read_byte(pc + 1);
            addvance = 2;
            Inst::Ld8(Arg8::Reg(Reg8::H), Arg8::Immed(n))
        }
        0x27 => Inst::Daa,
        0x28 => {
            let n = memory.read_byte(pc + 1);
            addvance = 2;
            Inst::Jrf(JpFlag::Z, n as i8)
        }
        0x29 => Inst::Add16(Arg16::Reg(Reg16::HL), Arg16::Reg(Reg16::HL)),
        0x2a => Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::IndIncHL),
        0x2b => Inst::Dec16(Arg16::Reg(Reg16::HL)),
        0x2c => Inst::Inc8(Arg8::Reg(Reg8::L)),
        0x2d => Inst::Dec8(Arg8::Reg(Reg8::L)),
        0x2e => {
            let n = memory.read_byte(pc + 1);
            addvance = 2;
            Inst::Ld8(Arg8::Reg(Reg8::L), Arg8::Immed(n))
        }
        0x2f => Inst::Cpl,
        0x30 => {
            let n = memory.read_byte(pc + 1);
            addvance = 2;
            Inst::Jrf(JpFlag::Nc, n as i8)
        }
        0x31 => {
            let nn = memory.read_word(pc + 1);
            addvance = 3;
            Inst::Ld16(Arg16::Reg(Reg16::SP), Arg16::Immed(nn))
        }
        0x32 => Inst::Ld8(Arg8::IndDecHL, Arg8::Reg(Reg8::A)),
        0x33 => Inst::Inc16(Arg16::Reg(Reg16::SP)),
        0x34 => Inst::Inc8(Arg8::IndReg(Reg16::HL)),
        0x35 => Inst::Dec8(Arg8::IndReg(Reg16::HL)),
        0x36 => {
            let n = memory.read_byte(pc + 1);
            addvance = 2;
            Inst::Ld8(Arg8::IndReg(Reg16::HL), Arg8::Immed(n))
        }
        0x37 => Inst::Scf,
        0x38 => {
            let n = memory.read_byte(pc + 1);
            addvance = 2;
            Inst::Jrf(JpFlag::C, n as i8)
        }
        0x39 => Inst::Add16(Arg16::Reg(Reg16::HL), Arg16::Reg(Reg16::SP)),
        0x3a => Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::IndDecHL),
        0x3b => Inst::Dec16(Arg16::Reg(Reg16::SP)),
        0x3c => Inst::Inc8(Arg8::Reg(Reg8::A)),
        0x3d => Inst::Dec8(Arg8::Reg(Reg8::A)),
        0x3e => {
            let n = memory.read_byte(pc + 1);
            addvance = 2;
            Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::Immed(n))
        }
        0x3f => Inst::Ccf,
        0x40 => Inst::Ld8(Arg8::Reg(Reg8::B), Arg8::Reg(Reg8::B)),
        0x41 => Inst::Ld8(Arg8::Reg(Reg8::B), Arg8::Reg(Reg8::C)),
        0x42 => Inst::Ld8(Arg8::Reg(Reg8::B), Arg8::Reg(Reg8::D)),
        0x43 => Inst::Ld8(Arg8::Reg(Reg8::B), Arg8::Reg(Reg8::E)),
        0x44 => Inst::Ld8(Arg8::Reg(Reg8::B), Arg8::Reg(Reg8::H)),
        0x45 => Inst::Ld8(Arg8::Reg(Reg8::B), Arg8::Reg(Reg8::L)),
        0x46 => Inst::Ld8(Arg8::Reg(Reg8::B), Arg8::IndReg(Reg16::HL)),
        0x47 => Inst::Ld8(Arg8::Reg(Reg8::B), Arg8::Reg(Reg8::A)),
        0x48 => Inst::Ld8(Arg8::Reg(Reg8::C), Arg8::Reg(Reg8::B)),
        0x49 => Inst::Ld8(Arg8::Reg(Reg8::C), Arg8::Reg(Reg8::C)),
        0x4a => Inst::Ld8(Arg8::Reg(Reg8::C), Arg8::Reg(Reg8::D)),
        0x4b => Inst::Ld8(Arg8::Reg(Reg8::C), Arg8::Reg(Reg8::E)),
        0x4c => Inst::Ld8(Arg8::Reg(Reg8::C), Arg8::Reg(Reg8::H)),
        0x4d => Inst::Ld8(Arg8::Reg(Reg8::C), Arg8::Reg(Reg8::L)),
        0x4e => Inst::Ld8(Arg8::Reg(Reg8::C), Arg8::IndReg(Reg16::HL)),
        0x4f => Inst::Ld8(Arg8::Reg(Reg8::C), Arg8::Reg(Reg8::A)),
        0x50 => Inst::Ld8(Arg8::Reg(Reg8::D), Arg8::Reg(Reg8::B)),
        0x51 => Inst::Ld8(Arg8::Reg(Reg8::D), Arg8::Reg(Reg8::C)),
        0x52 => Inst::Ld8(Arg8::Reg(Reg8::D), Arg8::Reg(Reg8::D)),
        0x53 => Inst::Ld8(Arg8::Reg(Reg8::D), Arg8::Reg(Reg8::E)),
        0x54 => Inst::Ld8(Arg8::Reg(Reg8::D), Arg8::Reg(Reg8::H)),
        0x55 => Inst::Ld8(Arg8::Reg(Reg8::D), Arg8::Reg(Reg8::L)),
        0x56 => Inst::Ld8(Arg8::Reg(Reg8::D), Arg8::IndReg(Reg16::HL)),
        0x57 => Inst::Ld8(Arg8::Reg(Reg8::D), Arg8::Reg(Reg8::A)),
        0x58 => Inst::Ld8(Arg8::Reg(Reg8::E), Arg8::Reg(Reg8::B)),
        0x59 => Inst::Ld8(Arg8::Reg(Reg8::E), Arg8::Reg(Reg8::C)),
        0x5a => Inst::Ld8(Arg8::Reg(Reg8::E), Arg8::Reg(Reg8::D)),
        0x5b => Inst::Ld8(Arg8::Reg(Reg8::E), Arg8::Reg(Reg8::E)),
        0x5c => Inst::Ld8(Arg8::Reg(Reg8::E), Arg8::Reg(Reg8::H)),
        0x5d => Inst::Ld8(Arg8::Reg(Reg8::E), Arg8::Reg(Reg8::L)),
        0x5e => Inst::Ld8(Arg8::Reg(Reg8::E), Arg8::IndReg(Reg16::HL)),
        0x5f => Inst::Ld8(Arg8::Reg(Reg8::E), Arg8::Reg(Reg8::A)),
        0x60 => Inst::Ld8(Arg8::Reg(Reg8::H), Arg8::Reg(Reg8::B)),
        0x61 => Inst::Ld8(Arg8::Reg(Reg8::H), Arg8::Reg(Reg8::C)),
        0x62 => Inst::Ld8(Arg8::Reg(Reg8::H), Arg8::Reg(Reg8::D)),
        0x63 => Inst::Ld8(Arg8::Reg(Reg8::H), Arg8::Reg(Reg8::E)),
        0x64 => Inst::Ld8(Arg8::Reg(Reg8::H), Arg8::Reg(Reg8::H)),
        0x65 => Inst::Ld8(Arg8::Reg(Reg8::H), Arg8::Reg(Reg8::L)),
        0x66 => Inst::Ld8(Arg8::Reg(Reg8::H), Arg8::IndReg(Reg16::HL)),
        0x67 => Inst::Ld8(Arg8::Reg(Reg8::H), Arg8::Reg(Reg8::A)),
        0x68 => Inst::Ld8(Arg8::Reg(Reg8::L), Arg8::Reg(Reg8::B)),
        0x69 => Inst::Ld8(Arg8::Reg(Reg8::L), Arg8::Reg(Reg8::C)),
        0x6a => Inst::Ld8(Arg8::Reg(Reg8::L), Arg8::Reg(Reg8::D)),
        0x6b => Inst::Ld8(Arg8::Reg(Reg8::L), Arg8::Reg(Reg8::E)),
        0x6c => Inst::Ld8(Arg8::Reg(Reg8::L), Arg8::Reg(Reg8::H)),
        0x6d => Inst::Ld8(Arg8::Reg(Reg8::L), Arg8::Reg(Reg8::L)),
        0x6e => Inst::Ld8(Arg8::Reg(Reg8::L), Arg8::IndReg(Reg16::HL)),
        0x6f => Inst::Ld8(Arg8::Reg(Reg8::L), Arg8::Reg(Reg8::A)),
        0x70 => Inst::Ld8(Arg8::IndReg(Reg16::HL), Arg8::Reg(Reg8::B)),
        0x71 => Inst::Ld8(Arg8::IndReg(Reg16::HL), Arg8::Reg(Reg8::C)),
        0x72 => Inst::Ld8(Arg8::IndReg(Reg16::HL), Arg8::Reg(Reg8::D)),
        0x73 => Inst::Ld8(Arg8::IndReg(Reg16::HL), Arg8::Reg(Reg8::E)),
        0x74 => Inst::Ld8(Arg8::IndReg(Reg16::HL), Arg8::Reg(Reg8::H)),
        0x75 => Inst::Ld8(Arg8::IndReg(Reg16::HL), Arg8::Reg(Reg8::L)),
        0x76 => Inst::Halt,
        0x77 => Inst::Ld8(Arg8::IndReg(Reg16::HL), Arg8::Reg(Reg8::A)),
        0x78 => Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::Reg(Reg8::B)),
        0x79 => Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::Reg(Reg8::C)),
        0x7a => Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::Reg(Reg8::D)),
        0x7b => Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::Reg(Reg8::E)),
        0x7c => Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::Reg(Reg8::H)),
        0x7d => Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::Reg(Reg8::L)),
        0x7e => Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::IndReg(Reg16::HL)),
        0x7f => Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::Reg(Reg8::A)),
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
    //
    // 0x20
    //
    #[test]
    fn decode_jr_nz_i8() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x20);
        m.write_byte(pc + 1, 0xff);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Jrf(JpFlag::Nz, -1), i);
        assert_eq!(2, a);
    }
    #[test]
    fn decode_ld_hl_u16() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x21);
        m.write_word(pc + 1, 0x1234);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld16(Arg16::Reg(Reg16::HL), Arg16::Immed(0x1234)), i);
        assert_eq!(3, a);
    }
    #[test]
    fn decode_ld_pihl_a() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x22);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::IndIncHL, Arg8::Reg(Reg8::A)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_inc_hl() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x23);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Inc16(Arg16::Reg(Reg16::HL)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_inc_h() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x24);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Inc8(Arg8::Reg(Reg8::H)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_dec_h() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x25);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Dec8(Arg8::Reg(Reg8::H)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_h_u8() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x26);
        m.write_byte(pc + 1, 0x12);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::H), Arg8::Immed(0x12)), i);
        assert_eq!(2, a);
    }
    #[test]
    fn decode_daa() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x27);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Daa, i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_jr_z_i8() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x28);
        m.write_byte(pc + 1, 0xff);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Jrf(JpFlag::Z, -1), i);
        assert_eq!(2, a);
    }
    #[test]
    fn decode_add_hl_hl() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x29);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Add16(Arg16::Reg(Reg16::HL), Arg16::Reg(Reg16::HL)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_a_pihl() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x2a);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::IndIncHL), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_dec_hl() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x2b);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Dec16(Arg16::Reg(Reg16::HL)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_inc_l() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x2c);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Inc8(Arg8::Reg(Reg8::L)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_dec_l() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x2d);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Dec8(Arg8::Reg(Reg8::L)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_l_u8() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x2e);
        m.write_byte(pc + 1, 0x12);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::L), Arg8::Immed(0x12)), i);
        assert_eq!(2, a);
    }
    #[test]
    fn decode_cpl() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x2f);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Cpl, i);
        assert_eq!(1, a);
    }
    //
    // 0x30
    //
    #[test]
    fn decode_jr_nc_i8() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x30);
        m.write_byte(pc + 1, 0xff);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Jrf(JpFlag::Nc, -1), i);
        assert_eq!(2, a);
    }
    #[test]
    fn decode_ld_sp_u16() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x31);
        m.write_word(pc + 1, 0x1234);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld16(Arg16::Reg(Reg16::SP), Arg16::Immed(0x1234)), i);
        assert_eq!(3, a);
    }
    #[test]
    fn decode_ld_pdhl_a() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x32);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::IndDecHL, Arg8::Reg(Reg8::A)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_inc_sp() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x33);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Inc16(Arg16::Reg(Reg16::SP)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_inc_phl() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x34);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Inc8(Arg8::IndReg(Reg16::HL)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_dec_phl() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x35);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Dec8(Arg8::IndReg(Reg16::HL)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_phl_u8() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x36);
        m.write_byte(pc + 1, 0x12);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::IndReg(Reg16::HL), Arg8::Immed(0x12)), i);
        assert_eq!(2, a);
    }
    #[test]
    fn decode_scf() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x37);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Scf, i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_jr_c_i8() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x38);
        m.write_byte(pc + 1, 0xff);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Jrf(JpFlag::C, -1), i);
        assert_eq!(2, a);
    }
    #[test]
    fn decode_add_hl_sp() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x39);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Add16(Arg16::Reg(Reg16::HL), Arg16::Reg(Reg16::SP)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_a_pdhl() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x3a);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::IndDecHL), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_dec_sp() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x3b);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Dec16(Arg16::Reg(Reg16::SP)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_inc_a() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x3c);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Inc8(Arg8::Reg(Reg8::A)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_dec_a() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x3d);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Dec8(Arg8::Reg(Reg8::A)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_a_u8() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x3e);
        m.write_byte(pc + 1, 0x12);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::Immed(0x12)), i);
        assert_eq!(2, a);
    }
    #[test]
    fn decode_ccf() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x3f);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ccf, i);
        assert_eq!(1, a);
    }
    //
    // 0x40
    //
    #[test]
    fn decode_ld_b_b() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x40);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::B), Arg8::Reg(Reg8::B)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_b_c() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x41);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::B), Arg8::Reg(Reg8::C)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_b_d() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x42);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::B), Arg8::Reg(Reg8::D)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_b_e() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x43);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::B), Arg8::Reg(Reg8::E)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_b_h() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x44);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::B), Arg8::Reg(Reg8::H)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_b_l() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x45);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::B), Arg8::Reg(Reg8::L)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_b_phl() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x46);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::B), Arg8::IndReg(Reg16::HL)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_b_a() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x47);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::B), Arg8::Reg(Reg8::A)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_c_b() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x48);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::C), Arg8::Reg(Reg8::B)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_c_c() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x49);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::C), Arg8::Reg(Reg8::C)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_c_d() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x4a);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::C), Arg8::Reg(Reg8::D)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_c_e() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x4b);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::C), Arg8::Reg(Reg8::E)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_c_h() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x4c);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::C), Arg8::Reg(Reg8::H)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_c_l() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x4d);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::C), Arg8::Reg(Reg8::L)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_c_phl() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x4e);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::C), Arg8::IndReg(Reg16::HL)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_c_a() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x4f);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::C), Arg8::Reg(Reg8::A)), i);
        assert_eq!(1, a);
    }
    //
    // 0x50
    //
    #[test]
    fn decode_ld_d_b() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x50);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::D), Arg8::Reg(Reg8::B)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_d_c() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x51);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::D), Arg8::Reg(Reg8::C)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_d_d() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x52);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::D), Arg8::Reg(Reg8::D)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_d_e() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x53);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::D), Arg8::Reg(Reg8::E)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_d_h() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x54);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::D), Arg8::Reg(Reg8::H)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_d_l() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x55);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::D), Arg8::Reg(Reg8::L)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_d_phl() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x56);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::D), Arg8::IndReg(Reg16::HL)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_d_a() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x57);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::D), Arg8::Reg(Reg8::A)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_e_b() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x58);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::E), Arg8::Reg(Reg8::B)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_e_c() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x59);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::E), Arg8::Reg(Reg8::C)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_e_d() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x5a);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::E), Arg8::Reg(Reg8::D)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_e_e() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x5b);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::E), Arg8::Reg(Reg8::E)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_e_h() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x5c);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::E), Arg8::Reg(Reg8::H)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_e_l() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x5d);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::E), Arg8::Reg(Reg8::L)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_e_phl() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x5e);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::E), Arg8::IndReg(Reg16::HL)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_e_a() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x5f);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::E), Arg8::Reg(Reg8::A)), i);
        assert_eq!(1, a);
    }
    //
    // 0x60
    //
    #[test]
    fn decode_ld_h_b() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x60);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::H), Arg8::Reg(Reg8::B)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_h_c() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x61);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::H), Arg8::Reg(Reg8::C)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_h_d() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x62);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::H), Arg8::Reg(Reg8::D)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_h_e() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x63);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::H), Arg8::Reg(Reg8::E)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_h_h() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x64);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::H), Arg8::Reg(Reg8::H)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_h_l() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x65);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::H), Arg8::Reg(Reg8::L)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_h_phl() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x66);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::H), Arg8::IndReg(Reg16::HL)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_h_a() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x67);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::H), Arg8::Reg(Reg8::A)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_l_b() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x68);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::L), Arg8::Reg(Reg8::B)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_l_c() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x69);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::L), Arg8::Reg(Reg8::C)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_l_d() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x6a);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::L), Arg8::Reg(Reg8::D)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_l_e() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x6b);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::L), Arg8::Reg(Reg8::E)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_l_h() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x6c);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::L), Arg8::Reg(Reg8::H)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_l_l() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x6d);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::L), Arg8::Reg(Reg8::L)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_l_phl() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x6e);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::L), Arg8::IndReg(Reg16::HL)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_l_a() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x6f);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::L), Arg8::Reg(Reg8::A)), i);
        assert_eq!(1, a);
    }
    //
    // 0x70
    //
    #[test]
    fn decode_ld_phl_b() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x70);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::IndReg(Reg16::HL), Arg8::Reg(Reg8::B)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_phl_c() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x71);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::IndReg(Reg16::HL), Arg8::Reg(Reg8::C)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_phl_d() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x72);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::IndReg(Reg16::HL), Arg8::Reg(Reg8::D)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_phl_e() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x73);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::IndReg(Reg16::HL), Arg8::Reg(Reg8::E)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_phl_h() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x74);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::IndReg(Reg16::HL), Arg8::Reg(Reg8::H)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_phl_l() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x75);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::IndReg(Reg16::HL), Arg8::Reg(Reg8::L)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_phl_phl() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x76);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Halt, i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_phl_a() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x77);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::IndReg(Reg16::HL), Arg8::Reg(Reg8::A)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_a_b() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x78);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::Reg(Reg8::B)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_a_c() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x79);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::Reg(Reg8::C)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_a_d() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x7a);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::Reg(Reg8::D)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_a_e() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x7b);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::Reg(Reg8::E)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_a_h() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x7c);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::Reg(Reg8::H)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_a_l() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x7d);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::Reg(Reg8::L)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_a_phl() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x7e);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::IndReg(Reg16::HL)), i);
        assert_eq!(1, a);
    }
    #[test]
    fn decode_ld_a_a() {
        let mut m = TestMemory::new();
        let pc = 0x0100;
        m.write_byte(pc, 0x7f);
        let (i, a) = decode(pc, &m);
        assert_eq!(Inst::Ld8(Arg8::Reg(Reg8::A), Arg8::Reg(Reg8::A)), i);
        assert_eq!(1, a);
    }
}
