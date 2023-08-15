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
