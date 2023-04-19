mod decode;
mod execute;
mod inst;
use crate::memory::MemoryIF;

pub struct Cpu {
    clock_m: usize,
    // t = 4m
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    f: u8,
    pc: u16,
    sp: u16,
    // Clock for last instr
    m: usize,
    // t = 4m
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            clock_m: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            f: 0,
            pc: 0,
            sp: 0,
            m: 0,
        }
    }
    pub fn run(&mut self, memory: &impl MemoryIF) {
        let (inst, addvance) = self.decode(memory);
        self.pc += addvance;
        self.execute(inst);
    }
}
