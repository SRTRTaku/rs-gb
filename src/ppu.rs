use crate::io::{GfxColor, Io, GFX_SIZE_X, GFX_SIZE_Y};
use crate::memory::MemoryIF;

const LCDC: u16 = 0xff40;
const SCY: u16 = 0xff42;
const SCX: u16 = 0xff43;
const BGP: u16 = 0xff47;

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
                    write_a_scanline(self.line, memory, io);
                    io.draw_a_line(self.line);
                }
            }
            // Horizontal blank
            Mode::Mode0 => {
                if self.clock_m >= 51 {
                    self.clock_m = 0;
                    self.line += 1;

                    if self.line >= 144 {
                        io.present();
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

fn write_a_scanline(ly: usize, memory: &mut impl MemoryIF, io: &mut Io) {
    //write BG
    let tile_map_area_addr = if memory.read_byte(LCDC) & 0x08 == 0x08 {
        0x9c00
    } else {
        0x9800
    };
    let scy = memory.read_byte(SCY) as usize;
    let scx = memory.read_byte(SCX) as usize;

    for lx in 0..GFX_SIZE_X {
        let y = (scy + ly) % 256;
        let x = (scx + lx) % 256;

        // get tile map address
        let y_tile = y / 8;
        let x_tile = x / 8;
        let tile_map_addr = tile_map_area_addr + (y_tile * 32 + x_tile) as u16;
        // get tile ID
        let tile_id = memory.read_byte(tile_map_addr) as u16;
        // get tile data address
        let tile_data_addr = if memory.read_byte(LCDC) & 0x10 == 0x10 {
            0x8000 + tile_id * 16
        } else {
            if tile_id < 128 {
                0x9000 + tile_id * 16
            } else {
                0x8800 + (tile_id - 128) * 16
            }
        };
        // get color ID
        let j = (y % 8) as u16;
        let i = (x % 8) as u16;
        let color_id = get_a_color_id(i, j, tile_data_addr, memory);

        // set color for gfx array
        let palette_data = memory.read_byte(BGP);
        io.gfx[ly * GFX_SIZE_X + lx] = id2color(palette_data, color_id);
    }
}

fn get_a_color_id(i: u16, j: u16, addr: u16, memory: &mut impl MemoryIF) -> u8 {
    let b0 = (memory.read_byte(addr + 2 * j) >> (7 - i)) & 0x01;
    let b1 = (memory.read_byte(addr + 2 * j + 1) >> (7 - i)) & 0x01;
    2 * b1 + b0
}

fn id2color(palette_data: u8, color_id: u8) -> GfxColor {
    if color_id > 3 {
        panic!()
    }
    let value = (palette_data >> (color_id * 2)) & 0x03;
    match value {
        0 => GfxColor::W,
        1 => GfxColor::LG,
        2 => GfxColor::DG,
        3 => GfxColor::B,
        _ => panic!(),
    }
}
