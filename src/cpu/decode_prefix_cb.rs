use super::inst::{Arg8, Inst, Reg16, Reg8};

pub fn decode_prefix_cb(code: u8) -> Inst {
    let r = match code % 8 {
        0 => Arg8::Reg(Reg8::B),
        1 => Arg8::Reg(Reg8::C),
        2 => Arg8::Reg(Reg8::D),
        3 => Arg8::Reg(Reg8::E),
        4 => Arg8::Reg(Reg8::H),
        5 => Arg8::Reg(Reg8::L),
        6 => Arg8::IndReg(Reg16::HL),
        7 => Arg8::Reg(Reg8::A),
        _ => panic!("can not reach here."),
    };
    match code {
        0x00..=0x07 => Inst::Rlc(r),
        0x08..=0x0f => Inst::Rrc(r),
        0x10..=0x17 => Inst::Rl(r),
        0x18..=0x1f => Inst::Rr(r),
        0x20..=0x27 => Inst::Sla(r),
        0x28..=0x2f => Inst::Sra(r),
        0x30..=0x37 => Inst::Swap(r),
        0x38..=0x3f => Inst::Srl(r),
        //
        0x40..=0x47 => Inst::Bit(0, r),
        0x48..=0x4f => Inst::Bit(1, r),
        0x50..=0x57 => Inst::Bit(2, r),
        0x58..=0x5f => Inst::Bit(3, r),
        0x60..=0x67 => Inst::Bit(4, r),
        0x68..=0x6f => Inst::Bit(5, r),
        0x70..=0x77 => Inst::Bit(6, r),
        0x78..=0x7f => Inst::Bit(7, r),
        //
        0x80..=0x87 => Inst::Res(0, r),
        0x88..=0x8f => Inst::Res(1, r),
        0x90..=0x97 => Inst::Res(2, r),
        0x98..=0x9f => Inst::Res(3, r),
        0xa0..=0xa7 => Inst::Res(4, r),
        0xa8..=0xaf => Inst::Res(5, r),
        0xb0..=0xb7 => Inst::Res(6, r),
        0xb8..=0xbf => Inst::Res(7, r),
        //
        0xc0..=0xc7 => Inst::Set(0, r),
        0xc8..=0xcf => Inst::Set(1, r),
        0xd0..=0xd7 => Inst::Set(2, r),
        0xd8..=0xdf => Inst::Set(3, r),
        0xe0..=0xe7 => Inst::Set(4, r),
        0xe8..=0xef => Inst::Set(5, r),
        0xf0..=0xf7 => Inst::Set(6, r),
        0xf8..=0xff => Inst::Set(7, r),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_prefix_cb_test() {
        let l = [
            (0x00, Inst::Rlc(Arg8::Reg(Reg8::B))),
            (0x08, Inst::Rrc(Arg8::Reg(Reg8::B))),
            (0x11, Inst::Rl(Arg8::Reg(Reg8::C))),
            (0x19, Inst::Rr(Arg8::Reg(Reg8::C))),
            (0x22, Inst::Sla(Arg8::Reg(Reg8::D))),
            (0x2a, Inst::Sra(Arg8::Reg(Reg8::D))),
            (0x33, Inst::Swap(Arg8::Reg(Reg8::E))),
            (0x3b, Inst::Srl(Arg8::Reg(Reg8::E))),
            (0x44, Inst::Bit(0, Arg8::Reg(Reg8::H))),
            (0x4c, Inst::Bit(1, Arg8::Reg(Reg8::H))),
            (0x55, Inst::Bit(2, Arg8::Reg(Reg8::L))),
            (0x5d, Inst::Bit(3, Arg8::Reg(Reg8::L))),
            (0x66, Inst::Bit(4, Arg8::IndReg(Reg16::HL))),
            (0x6e, Inst::Bit(5, Arg8::IndReg(Reg16::HL))),
            (0x77, Inst::Bit(6, Arg8::Reg(Reg8::A))),
            (0x7f, Inst::Bit(7, Arg8::Reg(Reg8::A))),
            (0x80, Inst::Res(0, Arg8::Reg(Reg8::B))),
            (0x88, Inst::Res(1, Arg8::Reg(Reg8::B))),
            (0x91, Inst::Res(2, Arg8::Reg(Reg8::C))),
            (0x99, Inst::Res(3, Arg8::Reg(Reg8::C))),
            (0xa2, Inst::Res(4, Arg8::Reg(Reg8::D))),
            (0xaa, Inst::Res(5, Arg8::Reg(Reg8::D))),
            (0xb3, Inst::Res(6, Arg8::Reg(Reg8::E))),
            (0xbb, Inst::Res(7, Arg8::Reg(Reg8::E))),
            (0xc4, Inst::Set(0, Arg8::Reg(Reg8::H))),
            (0xcc, Inst::Set(1, Arg8::Reg(Reg8::H))),
            (0xd5, Inst::Set(2, Arg8::Reg(Reg8::L))),
            (0xdd, Inst::Set(3, Arg8::Reg(Reg8::L))),
            (0xe6, Inst::Set(4, Arg8::IndReg(Reg16::HL))),
            (0xee, Inst::Set(5, Arg8::IndReg(Reg16::HL))),
            (0xf7, Inst::Set(6, Arg8::Reg(Reg8::A))),
            (0xff, Inst::Set(7, Arg8::Reg(Reg8::A))),
        ];
        for (c, des) in l {
            let i = decode_prefix_cb(c);
            assert_eq!(des, i);
        }
    }
}
