use crate::io::{GfxColor, Io, GFX_SIZE_X, GFX_SIZE_Y};
use crate::memory::{BGP, LCDC, LY, LYC, OBP0, OBP1, SCX, SCY, STAT, WX, WY};

const VRAM: u16 = 0x8000;

pub struct Ppu {
    mode: Mode,
    clock_m: usize,
    line: usize,
    window_internal_line: Option<usize>,
    stat_int_prev: bool,
    set_blank: bool,
    vram: [u8; 0x2000], // Graphics RAM 8k byte
    oam: [u8; 0x00a0],  // Object Attribute Memory
    lcd_regs: [u8; 0xc],
}

#[derive(Copy, Clone, PartialEq)]
enum Mode {
    Mode0,
    Mode1,
    Mode2,
    Mode3,
}

impl Ppu {
    pub fn new() -> Ppu {
        let mut lcd_regs = [0; 0xc];
        lcd_regs[0] = 0x80;
        Ppu {
            mode: Mode::Mode2,
            clock_m: 0,
            line: 0,
            window_internal_line: None,
            stat_int_prev: false,
            set_blank: false,
            vram: [0; 0x2000],
            oam: [0; 0x00a0],
            lcd_regs,
        }
    }
    pub fn read_vram(&self, index: usize) -> u8 {
        if self.is_enable() {
            match self.mode {
                Mode::Mode3 => 0xff,   // inaccessible
                _ => self.vram[index], // accessible
            }
        } else {
            self.vram[index] // accessible
        }
    }
    pub fn write_vram(&mut self, index: usize, val: u8) {
        if self.is_enable() {
            match self.mode {
                Mode::Mode3 => (),           // inaccessible
                _ => self.vram[index] = val, // accessible
            }
        } else {
            self.vram[index] = val // accessible
        }
    }
    pub fn read_oam(&self, index: usize) -> u8 {
        if self.is_enable() {
            match self.mode {
                Mode::Mode2 | Mode::Mode3 => 0xff, // inaccessible
                _ => self.oam[index],              // accessible
            }
        } else {
            self.oam[index] // accessible
        }
    }
    pub fn write_oam(&mut self, index: usize, val: u8) {
        if self.is_enable() {
            match self.mode {
                Mode::Mode2 | Mode::Mode3 => (), // inaccessible
                _ => self.oam[index] = val,      // accessible
            }
        } else {
            self.oam[index] = val // accessible
        }
    }
    pub fn read_lcd_reg(&self, index: usize) -> u8 {
        self.lcd_regs[index]
    }
    pub fn write_lcd_reg(&mut self, index: usize, val: u8) {
        self.lcd_regs[index] = val;
        if index == 0 && val & 0x80 != 0 {
            self.set_blank = true;
        }
    }
    fn is_enable(&self) -> bool {
        self.lcd_regs[0 /*(LCDC - LCDC)*/] & 0x80 != 0x00
    }
    pub fn run(&mut self, io: &mut Io, i_flg: &mut u8) -> Result<(), String> {
        if self.is_enable() {
            self.clock_m += 1;
            let stat = self.lcd_regs[(STAT - LCDC) as usize];
            match self.mode {
                // OAM scan
                Mode::Mode2 => {
                    if self.clock_m == 1 {
                        let wy = self.lcd_regs[(WY - LCDC) as usize] as usize;
                        if self.line == wy {
                            self.window_internal_line = Some(0);
                        }
                    }
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
                        self.write_a_scanline(io);
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
                            *i_flg |= 0x01
                        } else {
                            self.mode = Mode::Mode2;
                        }
                        self.lcd_regs[(LY - LCDC) as usize] = self.line as u8;
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
                            self.window_internal_line = None;
                        }
                        self.lcd_regs[(LY - LCDC) as usize] = self.line as u8;
                    }
                }
            }
            // Update LCD status
            let mut stat = stat & 0xf8; // masked
            if self.line as u8 == self.lcd_regs[(LYC - LCDC) as usize] {
                stat |= 0x04
            }
            stat += self.mode as u8;
            self.lcd_regs[(STAT - LCDC) as usize] = stat;
            // STAT interrupt
            let stat_int = set_stat_int(stat);
            if stat_int && !self.stat_int_prev {
                *i_flg |= 0x02;
            }
            self.stat_int_prev = stat_int;
            //
        } else if self.set_blank {
            self.mode = Mode::Mode2;
            self.clock_m = 0;
            self.line = 0;
            self.window_internal_line = None;
            self.stat_int_prev = false;
            self.set_blank = false;

            for ly in 0..GFX_SIZE_Y {
                Ppu::write_blank(ly, io);
            }
        }
        Ok(())
    }

    fn write_a_scanline(&mut self, io: &mut Io) {
        let lcdc = self.lcd_regs[0 /* LCDC - LCDC */];
        let ly = self.line;
        let obj = Obj::new(ly, &self.vram, &self.oam, &self.lcd_regs);
        obj.write_obj_before_gb(ly, io);
        if lcdc & 0x01 != 0 {
            // BG & Window enable priority: ON
            self.write_bg(io);

            if let Some(win_line) = self.window_internal_line {
                let wx = self.lcd_regs[(WX - LCDC) as usize] as usize;
                if wx <= GFX_SIZE_X - 1 + 7 && lcdc & 0x20 != 0 {
                    // wx - 7 <= GFX_SIZE_X - 1
                    // Window enable: ON
                    self.write_window(win_line, io);
                    self.window_internal_line = Some(win_line + 1);
                }
            }
        } else {
            // BG & Window enable priority: OFF
            Ppu::write_blank(self.line, io); // both background and window bcome blank (white)
        }
        obj.write_obj_after_gb(ly, io);
    }

    fn write_blank(ly: usize, io: &mut Io) {
        for lx in 0..GFX_SIZE_X {
            io.gfx[ly * GFX_SIZE_X + lx] = GfxColor::W;
        }
    }

    fn write_bg(&self, io: &mut Io) {
        let lcdc = self.lcd_regs[0 /* LCDC - LCDC */];
        let tile_map_area_addr = if lcdc & 0x08 == 0x08 { 0x9c00 } else { 0x9800 };
        let scy = self.lcd_regs[(SCY - LCDC) as usize] as usize;
        let scx = self.lcd_regs[(SCX - LCDC) as usize] as usize;
        let ly = self.line;

        for lx in 0..GFX_SIZE_X {
            let y = (scy + ly) % 256;
            let x = (scx + lx) % 256;

            // get tile map address
            let y_tile = y / 8;
            let x_tile = x / 8;
            let tile_map_addr = tile_map_area_addr + (y_tile * 32 + x_tile) as u16;
            // get tile ID
            let tile_id = self.vram[(tile_map_addr - VRAM) as usize] as u16;
            // get tile data address
            let tile_data_addr = if lcdc & 0x10 == 0x10 {
                0x8000 + tile_id * 16
            } else if tile_id < 128 {
                0x9000 + tile_id * 16
            } else {
                0x8800 + (tile_id - 128) * 16
            };
            // get color ID
            let j = (y % 8) as u16;
            let i = (x % 8) as u16;
            let tile_data_index = (tile_data_addr - VRAM) as usize;
            let color_id = get_a_color_id(i, j, &self.vram[tile_data_index..tile_data_index + 16]);

            // set color for gfx array
            let palette_data = self.lcd_regs[(BGP - LCDC) as usize];
            io.gfx[ly * GFX_SIZE_X + lx] = id2color(palette_data, color_id);
        }
    }

    fn write_window(&self, win_line: usize, io: &mut Io) {
        let lcdc = self.lcd_regs[0 /* LCDC - LCDC */];
        let tile_map_area_addr = if lcdc & 0x40 == 0x40 { 0x9c00 } else { 0x9800 };
        let wx = self.lcd_regs[(WX - LCDC) as usize] as usize;
        let ly = self.line;

        for lx in 0..GFX_SIZE_X {
            if lx + 7 < wx {
                // lx < wx - 7
                continue;
            }
            let x = lx + 7 - wx; // x = lx - (wx - 7)

            // get tile map address
            let y_tile = win_line / 8;
            let x_tile = x / 8;
            let tile_map_addr = tile_map_area_addr + (y_tile * 32 + x_tile) as u16;
            // get tile ID
            let tile_id = self.vram[(tile_map_addr - VRAM) as usize] as u16;
            // get tile data address
            let tile_data_addr = if lcdc & 0x10 == 0x10 {
                0x8000 + tile_id * 16
            } else if tile_id < 128 {
                0x9000 + tile_id * 16
            } else {
                0x8800 + (tile_id - 128) * 16
            };
            // get color ID
            let j = (win_line % 8) as u16;
            let i = (x % 8) as u16;
            let tile_data_index = (tile_data_addr - VRAM) as usize;
            let color_id = get_a_color_id(i, j, &self.vram[tile_data_index..tile_data_index + 16]);

            // set color for gfx array
            let palette_data = self.lcd_regs[(BGP - LCDC) as usize];
            io.gfx[ly * GFX_SIZE_X + lx] = id2color(palette_data, color_id);
        }
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
    fn new(ly: usize, vram: &[u8], oam: &[u8], lcd_regs: &[u8]) -> Obj {
        let size = {
            let lcdc = lcd_regs[0 /* LCDC - LCDC */];
            lcdc & 0x04 != 0
        };
        let objs = Obj::set_objs(size, oam);
        let objs = Obj::filter_objs(objs, ly, size);
        let mut objs = Obj::sort_objs(objs);
        objs.reverse();

        let mut line = [None; GFX_SIZE_X];
        for obj in objs {
            Obj::write_a_obj(&mut line, obj, ly, size, vram, lcd_regs);
        }
        Obj { line }
    }
    fn set_objs(size: bool, oam: &[u8]) -> Vec<ObjAttr> {
        let mut objs = Vec::new();
        for i in 0..40 {
            let index = i * 4;
            let y = oam[index];
            let x = oam[index + 1];
            let tile_index = {
                let idx = oam[index + 2];
                if size {
                    idx & 0xfe
                } else {
                    idx
                }
            };
            let attr = oam[index + 3];
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
        vram: &[u8],
        lcd_regs: &[u8],
    ) {
        let ly = ly as isize;
        // j & tile adder
        let y = obj.y as isize - 16;
        let (j, tile_data_index) = if size {
            // 8 * 16
            let dy = ly - y;
            if obj.attr & 0x30 != 0 {
                // Y flip: ON
                if dy < 8 {
                    (7 - dy, (obj.tile_index as u16 + 1) * 16)
                } else {
                    (7 - (dy - 8), obj.tile_index as u16 * 16)
                }
            } else {
                // Y flip: OFF
                if dy < 8 {
                    (dy, obj.tile_index as u16 * 16)
                } else {
                    (dy - 8, (obj.tile_index as u16 + 1) * 16)
                }
            }
        } else {
            // 8 * 8
            let index = obj.tile_index as u16 * 16;
            let j = if obj.attr & 0x30 != 0 {
                // Y flip: ON
                7 - (ly - y)
            } else {
                // Y flip: OFF
                ly - y
            };
            (j, index)
        };
        let tile_data_index = tile_data_index as usize;
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
            let color_id = get_a_color_id(
                i as u16,
                j as u16,
                &vram[tile_data_index..tile_data_index + 16],
            );

            // color id -> color
            let palette_data = if obj.attr & 0x10 != 0 {
                lcd_regs[(OBP1 - LCDC) as usize]
            } else {
                lcd_regs[(OBP0 - LCDC) as usize]
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

fn set_stat_int(stat: u8) -> bool {
    let mode = stat & 0x03;
    let eq = stat & 0x04;
    ((stat & 0x08 != 0) && (mode == 0))
        || ((stat & 0x10 != 0) && (mode == 1))
        || ((stat & 0x20 != 0) && (mode == 2))
        || ((stat & 0x40 != 0) && (eq != 0))
}
fn get_a_color_id(i: u16, j: u16, tile_data: &[u8]) -> u8 {
    let b0 = (tile_data[(2 * j) as usize] >> (7 - i)) & 0x01;
    let b1 = (tile_data[(2 * j + 1) as usize] >> (7 - i)) & 0x01;
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
