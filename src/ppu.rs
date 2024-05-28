use crate::io::{GfxColor, Io, GFX_SIZE_X};
use crate::memory::{MemoryIF, BGP, IF, LCDC, LY, LYC, OBP0, OBP1, SCX, SCY, STAT, WX, WY};

pub struct Ppu {
    mode: Mode,
    clock_m: usize,
    line: usize,
}

#[derive(Copy, Clone)]
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
        let stat = memory.read_byte(STAT);
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
                        // VBlank interrupt
                        let i_flag = memory.read_byte(IF);
                        memory.write_byte(IF, i_flag | 0x01);
                    } else {
                        self.mode = Mode::Mode2;
                    }
                    memory.write_byte(LY, self.line as u8);
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
        // Update LCD status
        let mut stat = stat & 0xf8; // masked
        if self.line as u8 == memory.read_byte(LYC) {
            stat |= 0x04
        }
        stat += self.mode as u8;
        memory.write_byte(STAT, stat);
        // STAT interrupt
        if stat_int(stat) {
            let i_flag = memory.read_byte(IF);
            memory.write_byte(IF, i_flag | 0x02);
        }
        //
        Ok(())
    }
}
#[derive(Copy, Clone)]
struct PixelInfo {
    bg_over_obj: bool,
    color: GfxColor,
}
struct Obj {
    line: [Option<PixelInfo>; GFX_SIZE_X],
}

struct ObjAttr {
    y: u8,
    x: u8,
    tile_index: u8, // if 8 * 16, indices are tile_index & tile_index +1
    attr: u8,
}

impl Obj {
    fn new(ly: usize, memory: &impl MemoryIF) -> Obj {
        let size = {
            let lcdc = memory.read_byte(LCDC);
            lcdc & 0x04 != 0
        };
        let objs = Obj::set_objs(size, memory);
        let objs = Obj::filter_objs(objs, ly, size);
        let mut objs = Obj::sort_objs(objs);
        objs.reverse();

        let mut line = [None; GFX_SIZE_X];
        for obj in objs {
            Obj::write_a_obj(&mut line, obj, ly, size, memory);
        }
        Obj { line }
    }
    fn set_objs(size: bool, memory: &impl MemoryIF) -> Vec<ObjAttr> {
        let mut objs = Vec::new();
        let base = 0xfe00;
        for i in 0..40 {
            let addr = base + i * 4;
            let y = memory.read_byte(addr);
            let x = memory.read_byte(addr + 1);
            let tile_index = {
                let idx = memory.read_byte(addr + 2);
                if size {
                    idx & 0xfe
                } else {
                    idx
                }
            };
            let attr = memory.read_byte(addr + 3);
            objs.push(ObjAttr {
                y,
                x,
                tile_index,
                attr,
            });
        }
        objs
    }
    fn filter_objs(objs: Vec<ObjAttr>, ly: usize, size: bool) -> Vec<ObjAttr> {
        let ly = ly as isize;
        let mut filterd = Vec::new();
        for obj in objs {
            let y_min = obj.y as isize - 16;
            let y_max = if size { y_min + 16 } else { y_min + 8 } - 1;
            if ly >= y_min && ly <= y_max {
                filterd.push(obj);
                if filterd.len() >= 10 {
                    break;
                }
            }
        }
        filterd
    }
    fn sort_objs(objs: Vec<ObjAttr>) -> Vec<ObjAttr> {
        let mut sorted = objs;
        sorted.sort_by(|a, b| a.x.cmp(&b.x));
        sorted
    }
    fn write_a_obj(
        line: &mut [Option<PixelInfo>],
        obj: ObjAttr,
        ly: usize,
        size: bool,
        memory: &impl MemoryIF,
    ) {
        let ly = ly as isize;
        // j & tile adder
        let y = obj.y as isize - 16;
        let (j, tile_addr) = if size {
            // 8 * 16
            let dy = ly - y;
            if obj.attr & 0x30 != 0 {
                // Y flip: ON
                if dy < 8 {
                    (7 - dy, 0x8000 + (obj.tile_index as u16 + 1) * 16)
                } else {
                    (7 - (dy - 8), 0x8000 + obj.tile_index as u16 * 16)
                }
            } else {
                // Y flip: OFF
                if dy < 8 {
                    (dy, 0x8000 + obj.tile_index as u16 * 16)
                } else {
                    (dy - 8, 0x8000 + (obj.tile_index as u16 + 1) * 16)
                }
            }
        } else {
            // 8 * 8
            let addr = 0x8000 + obj.tile_index as u16 * 16;
            let j = if obj.attr & 0x30 != 0 {
                // Y flip: ON
                7 - (ly - y)
            } else {
                // Y flip: OFF
                ly - y
            };
            (j, addr)
        };
        let bg_over_obj = obj.attr & 0x80 != 0;
        // i
        let x = obj.x as isize - 8;
        for i0 in 0..8 {
            let i = if obj.attr & 0x20 != 0 {
                // x flip
                7 - i0
            } else {
                i0
            };
            let lx = x + i0;
            if lx < 0 || lx > GFX_SIZE_X as isize - 1 {
                break;
            }
            let lx = lx as usize;

            // get color id
            let color_id = get_a_color_id(i as u16, j as u16, tile_addr, memory);

            // color id -> color
            let palette_data = if obj.attr & 0x10 != 0 {
                memory.read_byte(OBP1)
            } else {
                memory.read_byte(OBP0)
            };
            let color = id2color(palette_data, color_id);

            // set color to line[]
            if color != GfxColor::W {
                line[lx] = Some(PixelInfo { bg_over_obj, color });
            }
        }
    }

    fn write_obj_before_gb(&self, ly: usize, io: &mut Io) {
        for lx in 0..GFX_SIZE_X {
            if let Some(pixel_info) = self.line[lx] {
                if pixel_info.bg_over_obj {
                    io.gfx[ly * GFX_SIZE_X + lx] = pixel_info.color;
                }
            }
        }
    }
    fn write_obj_after_gb(&self, ly: usize, io: &mut Io) {
        for lx in 0..GFX_SIZE_X {
            if let Some(pixel_info) = self.line[lx] {
                if !pixel_info.bg_over_obj {
                    io.gfx[ly * GFX_SIZE_X + lx] = pixel_info.color;
                }
            }
        }
    }
}

fn stat_int(stat: u8) -> bool {
    let mode = stat & 0x03;
    let eq = stat & 0x04;
    ((stat & 0x08 != 0) && (mode == 0))
        || ((stat & 0x10 != 0) && (mode == 1))
        || ((stat & 0x20 != 0) && (mode == 2))
        || ((stat & 0x40 != 0) && (eq != 0))
}

fn write_a_scanline(ly: usize, memory: &mut impl MemoryIF, io: &mut Io) {
    let lcdc = memory.read_byte(LCDC);
    if lcdc & 0x80 == 0x80 {
        // LCD  PPU enable: ON
        let obj = Obj::new(ly, memory);
        obj.write_obj_before_gb(ly, io);
        if lcdc & 0x01 == 0x01 {
            // BG & Window enable priority: ON
            write_bg(ly, memory, io);
            if lcdc & 0x20 == 0x20 {
                // Window enable: ON
                write_window(ly, memory, io);
            }
        } else {
            // BG & Window enable priority: OFF
            write_blank(ly, io); // both background and window bcome blank (white)
        }
        obj.write_obj_after_gb(ly, io);
    } else {
        // LCD  PPU enable: OFF
        write_blank(ly, io); // displays as a white shiter than color #0
    }
}
fn write_blank(ly: usize, io: &mut Io) {
    for lx in 0..GFX_SIZE_X {
        io.gfx[ly * GFX_SIZE_X + lx] = GfxColor::W;
    }
}

fn write_bg(ly: usize, memory: &mut impl MemoryIF, io: &mut Io) {
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
        } else if tile_id < 128 {
            0x9000 + tile_id * 16
        } else {
            0x8800 + (tile_id - 128) * 16
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

fn write_window(ly: usize, memory: &mut impl MemoryIF, io: &mut Io) {
    let tile_map_area_addr = if memory.read_byte(LCDC) & 0x40 == 0x40 {
        0x9c00
    } else {
        0x9800
    };
    let wy = memory.read_byte(WY) as usize;
    let wx = memory.read_byte(WX) as usize;

    if ly < wy {
        return;
    }
    let y = ly - wy;

    for lx in 0..GFX_SIZE_X {
        if lx + 7 < wx {
            // lx < wx - 7
            continue;
        }
        let x = lx + 7 - wx; // x = lx - (wx - 7)

        // get tile map address
        let y_tile = y / 8;
        let x_tile = x / 8;
        let tile_map_addr = tile_map_area_addr + (y_tile * 32 + x_tile) as u16;
        // get tile ID
        let tile_id = memory.read_byte(tile_map_addr) as u16;
        // get tile data address
        let tile_data_addr = if memory.read_byte(LCDC) & 0x10 == 0x10 {
            0x8000 + tile_id * 16
        } else if tile_id < 128 {
            0x9000 + tile_id * 16
        } else {
            0x8800 + (tile_id - 128) * 16
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

fn get_a_color_id(i: u16, j: u16, addr: u16, memory: &impl MemoryIF) -> u8 {
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
