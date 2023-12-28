//// I/O Rregisters
// LCD
pub const LCDC: u16 = 0xff40;
pub const STAT: u16 = 0xff41;
pub const SCY: u16 = 0xff42;
pub const SCX: u16 = 0xff43;
pub const LY: u16 = 0xff44;
pub const LYC: u16 = 0xff45;
pub const BGP: u16 = 0xff47;
//// Interrupt
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
