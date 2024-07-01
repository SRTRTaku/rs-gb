use crate::memory::{MemoryIF, DIV, DMA, IF};
use crate::Io;
use crate::Ppu;
use crate::Timer;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

const ROM_BANK_BIT_MAX: usize = 7;
const RAM_BANK_BIT_MAX: usize = 3;
const ROM_BANK_MAX: usize = 1 << ROM_BANK_BIT_MAX;
const RAM_BANK_MAX: usize = 1 << RAM_BANK_BIT_MAX;

pub struct Mmu {
    ram_enable: bool,
    rom_bank_bit: usize,
    ram_bank_bit: Option<usize>,
    rom_bank: usize,
    ram_bank: usize,
    rom: [u8; 0x4000 * ROM_BANK_MAX],  // Cartridge ROM 32k byte
    eram: [u8; 0x2000 * RAM_BANK_MAX], // Cargridge (External) RAM 8k byte
    wram: [u8; 0x2000],                // Working RAM 8k byte
    ioreg: [u8; 0x0080],               // I/O Registers
    zram: [u8; 0x0080],                // Zero-page Ram 128 byte
    ppu: Ppu,
    timer: Timer,
    oam_dma: Option<(usize, u16)>,
}

impl Mmu {
    pub fn new() -> Mmu {
        Mmu {
            ram_enable: false,
            rom_bank_bit: ROM_BANK_BIT_MAX,
            ram_bank_bit: Some(RAM_BANK_BIT_MAX),
            rom_bank: 1,
            ram_bank: 0,
            rom: [0; 0x4000 * ROM_BANK_MAX],
            eram: [0; 0x2000 * RAM_BANK_MAX],
            wram: [0; 0x2000],
            ioreg: [0; 0x0080],
            zram: [0; 0x0080],
            ppu: Ppu::new(),
            timer: Timer::new(),
            oam_dma: None,
        }
    }

    pub fn load(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
        let buf = BufReader::new(File::open(filename)?);
        for (i, byte_or_error) in buf.bytes().enumerate() {
            let byte = byte_or_error.unwrap();
            self.rom[i] = byte;
        }
        Ok(())
    }

    pub fn dump(&self, addr: u16) {
        let addr = addr as usize;
        let width = 0x0020;
        let begin = if addr > width { addr - width } else { 0 };
        let end = if addr <= 0xffff - width {
            addr + width
        } else {
            0xffff
        };
        // print header
        print!("     |");
        for i in 0..16 {
            print!("{:3x}", i);
        }
        println!();

        for row in (begin / 16)..=(end / 16) {
            let offset = row * 16;
            print!("{:04x} |", offset);
            for i in 0..16 {
                let a = offset + i;
                if a == addr {
                    print!("\x1b[7m");
                    print!(" {:02x}", self.read_byte(a as u16));
                    print!("\x1b[0m");
                } else {
                    print!(" {:02x}", self.read_byte(a as u16));
                }
            }
            println!();
        }

        println!("I/O registers:");
        for row in (0xff00 / 16)..=(0xff80 / 16) {
            let offset = row * 16;
            print!("{:04x} |", offset);
            for i in 0..16 {
                let a = offset + i;
                print!(" {:02x}", self.read_byte(a as u16));
            }
            println!();
        }
        println!("hram:");
        for row in (0xff80 / 16)..=(0xffff / 16) {
            let offset = row * 16;
            print!("{:04x} |", offset);
            for i in 0..16 {
                let a = offset + i;
                print!(" {:02x}", self.read_byte(a as u16));
            }
            println!();
        }
    }
    pub fn run_ppu(&mut self, io: &mut Io) -> Result<(), String> {
        let mut i_flg = self.read_byte(IF);
        self.ppu.run(io, &mut i_flg)?;
        self.write_byte(IF, i_flg);

        // dma
        let dma = self.read_byte(DMA);
        if dma != 0 {
            let addr = (dma as u16) * 0x0100;
            self.write_byte(DMA, 0);
            self.oam_dma = Some((0, addr))
        } else if let Some((clc, addr)) = self.oam_dma {
            let clc = clc + 1;
            if clc >= 160 {
                for i in 0..0xa0 {
                    let val = self.read_byte(addr + i);
                    self.write_byte(0xfe00 + i, val);
                }
                self.oam_dma = None;
            } else {
                self.oam_dma = Some((clc, addr));
            }
        }

        Ok(())
    }
    pub fn run_timer(&mut self, stop: bool) -> Result<(), String> {
        let mut i_flg = self.read_byte(IF);
        self.timer.run(&mut i_flg, stop)?;
        self.write_byte(IF, i_flg);
        Ok(())
    }
}

impl MemoryIF for Mmu {
    fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            // BIOS / ROM0
            //0x0000..=0x3fff => {
            //let index = addr as usize;
            //if self.inbios && addr < 0x0100 {
            //self.bios[index]
            //} else {
            //self.rom[index]
            //}
            //}
            // ROM1 (unbanked) 16k
            0x0000..=0x3fff => {
                let index = addr as usize;
                self.rom[index]
            }
            0x4000..=0x7fff => {
                let index = (addr - 0x4000) as usize;
                self.rom[self.rom_bank * 0x4000 + index]
            }
            // Graphics: VRAM 8k
            0x8000..=0x9fff => {
                let index = (addr - 0x8000) as usize;
                self.ppu.read_vram(index)
            }
            // External RAM 8k
            0xa000..=0xbfff => {
                if self.ram_enable {
                    let index = (addr - 0xa000) as usize;
                    self.eram[self.ram_bank * 0x2000 + index]
                } else {
                    0xff
                }
            }
            // Working RAM 8k
            0xc000..=0xdfff => {
                let index = (addr - 0xc000) as usize;
                self.wram[index]
            }
            // Working RAM (shadow)
            0xe000..=0xfdff => {
                let index = (addr - 0xe000) as usize;
                self.wram[index]
            }
            // Graphics: sprite information
            0xfe00..=0xfe9f => {
                let index = (addr - 0xfe00) as usize;
                self.ppu.read_oam(index)
            }
            // not usable
            0xfea0..=0xfeff => panic!("not usable"),
            // I/O Register
            0xff00..=0xff7f => match addr {
                0xff04..=0xff07 => {
                    let index = (addr - 0xff04) as usize;
                    self.timer.read_timer_reg(index)
                }
                0xff40..=0xff4b => {
                    let index = (addr - 0xff40) as usize;
                    self.ppu.read_lcd_reg(index)
                }
                _ => {
                    let index = (addr - 0xff00) as usize;
                    self.ioreg[index]
                }
            },
            // Zero-page
            0xff80..=0xffff => {
                let index = (addr - 0xff80) as usize;
                self.zram[index]
            }
        }
    }
    fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            // ROM1 (unbanked) 16k
            // 0x0000..=0x7fff => {
            //     panic!("write_byte:rom 0x{:x}, val: {}", addr, val);
            // }
            // RAM Enable
            0x0000..=0x1fff => {
                if val & 0x0f == 0x0a {
                    if self.ram_bank_bit.is_none() {
                        panic!("write_byte: ram_bank_num is 0");
                    }
                    self.ram_enable = true;
                } else {
                    self.ram_enable = false;
                }
            }
            // ROM Bank Number
            0x2000..=0x3fff => {
                let val = val & 0x1f;
                let val = if val == 0 { 1 } else { val };
                let val = val & ((1 << self.rom_bank_bit) - 1);
                self.rom_bank = (self.rom_bank & 0xe0) + val as usize;
            }
            // RAM Bank Number
            0x4000..=0x5fff => {
                let ram = val & 0x03;
                let rom = (val & 0xc0) >> 6;
                self.ram_bank = ram as usize;
                self.rom_bank = (self.rom_bank & 0x1f) + (rom << 5) as usize;
            }
            // Banking Mode Select
            0x6000..=0x7fff => {
                panic!("write_byte:rom 0x{:x}, val: {}", addr, val);
            }
            // Graphics: VRAM 8k
            0x8000..=0x9fff => {
                let index = (addr - 0x8000) as usize;
                self.ppu.write_vram(index, val);
            }
            // External RAM 8k
            0xa000..=0xbfff => {
                let index = (addr - 0xa000) as usize;
                self.eram[self.ram_bank * 0x2000 + index] = val;
            }
            // Working RAM 8k
            0xc000..=0xdfff => {
                let index = (addr - 0xc000) as usize;
                self.wram[index] = val;
            }
            // Working RAM (shadow)
            0xe000..=0xfdff => {
                // let index = (addr - 0xe000) as usize;
                // self.wram[index]
                panic!("working ram (shadow)");
            }
            // Graphics: sprite information
            0xfe00..=0xfe9f => {
                let index = (addr - 0xfe00) as usize;
                self.ppu.write_oam(index, val);
            }
            // not usable
            0xfea0..=0xfeff => (), //panic!("not usable"),
            // I/O Register
            0xff00..=0xff7f => match addr {
                0xff04..=0xff07 => {
                    let index = (addr - 0xff04) as usize;
                    self.timer.write_timer_reg(index, val);
                }
                0xff40..=0xff4b => {
                    let index = (addr - 0xff40) as usize;
                    self.ppu.write_lcd_reg(index, val);
                }
                _ => {
                    let index = (addr - 0xff00) as usize;
                    self.ioreg[index] = val;
                }
            },
            // Zero-page
            0xff80..=0xffff => {
                let index = (addr - 0xff80) as usize;
                self.zram[index] = val;
            }
        }
    }
}
