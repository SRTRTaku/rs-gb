/// I/O Rregisters
// Joypad
pub const JOYP: u16 = 0xff00;
// Timer and Divider
pub const DIV: u16 = 0xff04;
pub const TIMA: u16 = 0xff05;
pub const TMA: u16 = 0xff06;
pub const TAC: u16 = 0xff07;
// LCD
pub const LCDC: u16 = 0xff40;
pub const STAT: u16 = 0xff41;
pub const SCY: u16 = 0xff42;
pub const SCX: u16 = 0xff43;
pub const LY: u16 = 0xff44;
pub const LYC: u16 = 0xff45;
pub const DMA: u16 = 0xff46;
pub const BGP: u16 = 0xff47;
pub const OBP0: u16 = 0xff48;
pub const OBP1: u16 = 0xff49;
pub const WY: u16 = 0xff4a;
pub const WX: u16 = 0xff4b;
/// Interrupt
pub const IF: u16 = 0xff0f; // Interrupt flag
pub const IE: u16 = 0xffff; // Interrupt enable

pub trait MemoryIF {
    fn read_byte(&self, addr: u16) -> u8;
    fn read_word(&self, addr: u16) -> u16 {
        let l = self.read_byte(addr) as u16;
        let h = self.read_byte(addr + 1) as u16;
        (h << 8) | l
    }
    fn write_byte(&mut self, addr: u16, val: u8);
    fn write_word(&mut self, addr: u16, val: u16) {
        let h = (val >> 8) as u8;
        let l = (val & 0x00ff) as u8;
        self.write_byte(addr, l);
        self.write_byte(addr + 1, h);
    }
}
