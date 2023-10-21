use crate::io::Io;
use crate::memory::MemoryIF;

pub struct Ppu {
    mode: Mode,
    clock_m: usize,
    line: usize,
}

enum Mode {
    Mode0,
    Mode1,
    Mode2,
    Mode3,
}

impl Ppu {
    pub fn new() -> Ppu {
        Ppu {
            mode: Mode::Mode2,
            clock_m: 0,
            line: 0,
        }
    }
    pub fn run(&mut self, memory: &mut impl MemoryIF, io: &mut Io) -> Result<(), String> {
        self.clock_m += 1;
        match self.mode {
            // OAM scan
            Mode::Mode2 => {
                if self.clock_m >= 20 {
                    self.clock_m = 0;
                    self.mode = Mode::Mode3;
                }
            }
            // Drawing pixels
            Mode::Mode3 => {
                if self.clock_m >= 43 {
                    self.clock_m = 0;
                    self.mode = Mode::Mode0;

                    // write a scanline to the framebuffer
                }
            }
            // Horizontal blank
            Mode::Mode0 => {
                if self.clock_m >= 51 {
                    self.clock_m = 0;
                    self.line += 1;

                    if self.line >= 144 {
                        self.mode = Mode::Mode1;
                    } else {
                        self.mode = Mode::Mode2;
                    }
                    memory.write_byte(0xff44, self.line as u8);
                }
            }
            // Vertical blank
            Mode::Mode1 => {
                if self.clock_m >= 114 {
                    self.clock_m = 0;
                    self.line += 1;
                    if self.line > 153 {
                        self.mode = Mode::Mode2;
                        self.line = 0;
                    }
                    memory.write_byte(0xff44, self.line as u8);
                }
            }
        }
        Ok(())
    }
}
