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
        _ => todo!(),
    }
}
