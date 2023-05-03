use super::inst::{Arg8, Inst, Reg8};

pub fn decode_prefix_cb(code: u8) -> Inst {
    match code {
        0x00 => Inst::Rlc(Arg8::Reg(Reg8::B)),
        _ => todo!(),
    }
}
