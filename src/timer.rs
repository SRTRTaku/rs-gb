use crate::memory::{MemoryIF, DIV, IF, TAC, TIMA, TMA};

pub struct Timer {
    clock_div_m: usize,
    clock_tima_m: usize,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            clock_div_m: 0,
            clock_tima_m: 0,
        }
    }
    pub fn run(&mut self, memory: &mut impl MemoryIF) -> Result<(), String> {
        self.clock_div_m += 1;
        if self.clock_div_m >= 64 {
            self.clock_div_m = 0;
            let div = memory.read_byte(DIV);
            memory.write_byte(DIV, div.wrapping_add(1));
        }
        let tac = memory.read_byte(TAC);
        let m = match tac & 0x03 {
            0x00 => 256,
            0x01 => 4,
            0x02 => 16,
            0x03 => 64,
            _ => return Err("Timer::run: invalid tac value".to_string()),
        };
        if tac & 0x04 != 0 {
            self.clock_tima_m += 1;
            if self.clock_tima_m >= m {
                self.clock_tima_m = 0;
                let tima = memory.read_byte(TIMA);
                if tima == u8::MAX {
                    // overflow
                    let tma = memory.read_byte(TMA);
                    memory.write_byte(TIMA, tma);
                    // Timer interrupt
                    let i_flag = memory.read_byte(IF);
                    memory.write_byte(IF, i_flag | 0x04);
                } else {
                    memory.write_byte(TIMA, tima + 1);
                }
            }
        }
        Ok(())
    }
}
