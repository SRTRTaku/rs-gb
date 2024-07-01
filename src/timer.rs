use crate::memory::{DIV, TAC, TIMA, TMA};

pub struct Timer {
    clock_div_m: usize,
    clock_tima_m: usize,
    timer_regs: [u8; 4],
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            clock_div_m: 0,
            clock_tima_m: 0,
            timer_regs: [0; 4],
        }
    }
    pub fn read_timer_reg(&self, index: usize) -> u8 {
        self.timer_regs[index]
    }
    pub fn write_timer_reg(&mut self, index: usize, val: u8) {
        let val = if index == 0 /*DIV - DIV*/ { 0 } else { val };
        self.timer_regs[index] = val;
    }
    pub fn run(&mut self, i_flg: &mut u8, stop: bool) -> Result<(), String> {
        if stop {
            self.clock_div_m = 0;
            self.timer_regs[0 /*DIV - DIV*/] = 0;
        } else {
            self.clock_div_m += 1;
            if self.clock_div_m >= 64 {
                self.clock_div_m = 0;
                let div = self.timer_regs[0 /*DIV - DIV*/];
                self.timer_regs[0 /*DIV - DIV*/] = div.wrapping_add(1);
            }
        }
        let tac = self.timer_regs[(TAC - DIV) as usize];
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
                let tima = self.timer_regs[(TIMA - DIV) as usize];
                if tima == u8::MAX {
                    // overflow
                    let tma = self.timer_regs[(TMA - DIV) as usize];
                    self.timer_regs[(TIMA - DIV) as usize] = tma;
                    // Timer interrupt
                    *i_flg |= 0x04;
                } else {
                    self.timer_regs[(TIMA - DIV) as usize] = tima + 1;
                }
            }
        }
        Ok(())
    }
}
