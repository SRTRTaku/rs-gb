pub trait MemoryIF {
    fn read_byte(&self, addr: u16) -> u8;
    fn read_word(&self, addr: u16) -> u16;
    fn write_byte(&self, addr: u16, val: u8);
    fn write_word(&self, addr: u16, val: u16);
}
