use crate::memory::MemoryIF;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

pub struct MMU {
    inbios: bool,
    bios: [u8; 0x0100], // GameBoy BIOS code 256 byte
    rom: [u8; 0x8000],  // Cartridge ROM 32k byte
    vram: [u8; 0x2000], // Graphics RAM 8k byte
    eram: [u8; 0x2000], // Cargridge (External) RAM 8k byte
    wram: [u8; 0x2000], // Working RAM 8k byte
    zram: [u8; 0x0080], // Zero-page Ram 128 byte
}

impl MMU {
    pub fn new() -> MMU {
        MMU {
            inbios: true,
            bios: [0; 0x0100],
            rom: [0; 0x8000],
            vram: [0; 0x2000],
            eram: [0; 0x2000],
            wram: [0; 0x2000],
            zram: [0; 0x0080],
        }
    }

    pub fn load(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
        let buf = BufReader::new(File::open(filename)?);
        for (i, byte_or_error) in buf.bytes().enumerate() {
            if i >= 0x8000 {
                break;
            }
            let byte = byte_or_error.unwrap();
            self.rom[i] = byte;
        }
        Ok(())
    }

    fn out_bios(&mut self) {
        self.inbios = false;
    }

    pub fn dump(&self) {
        println!("rom");
        let begin = 0x0000;
        let end = 0x1000;
        // print header
        print!("     |");
        for i in 0..16 {
            print!("{:3x}", i);
        }
        println!();

        for row in (begin / 16)..(0x8000 / 16) {
            let offset = row * 16;
            print!("{:04x} |", offset);
            for i in 0..16 {
                print!(" {:02x}", self.rom[offset + i]);
            }
            println!();

            // omit
            if offset > end {
                break;
            }
        }
    }
}

impl MemoryIF for MMU {
    fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            // BIOS / ROM0
            0x0000..=0x3fff => {
                let index = addr as usize;
                if self.inbios && addr < 0x0100 {
                    self.bios[index]
                } else {
                    self.rom[index]
                }
            }
            // ROM1 (unbanked) 16k
            0x4000..=0x7fff => {
                let index = addr as usize;
                self.rom[index]
            }
            // Graphics: VRAM 8k
            0x8000..=0x9fff => {
                let index = (addr - 0x8000) as usize;
                self.vram[index]
            }
            // External RAM 8k
            0xa000..=0xbfff => {
                let index = (addr - 0xa000) as usize;
                self.eram[index]
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
            // Fraphics: sprite information
            0xfe00..=0xfe9f => todo!(),
            // not usable
            0xfea0..=0xfeff => panic!("not usable"),
            // I/O Register
            0xff00..=0xff7f => todo!(),
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
            0x0000..=0x7fff => {
                panic!("rom");
            }
            // Graphics: VRAM 8k
            0x8000..=0x9fff => {
                let index = (addr - 0x8000) as usize;
                self.vram[index] = val;
            }
            // External RAM 8k
            0xa000..=0xbfff => {
                let index = (addr - 0xa000) as usize;
                self.eram[index] = val;
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
            // Fraphics: sprite information
            0xfe00..=0xfe9f => todo!(),
            // not usable
            0xfea0..=0xfeff => panic!("not usable"),
            // I/O Register
            0xff00..=0xff7f => todo!(),
            // Zero-page
            0xff80..=0xffff => {
                let index = (addr - 0xff80) as usize;
                self.zram[index] = val;
            }
        }
    }
}
