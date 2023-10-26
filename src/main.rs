use cpu::Cpu;
use io::{GfxColor, Io, GFX_SIZE_X, GFX_SIZE_Y};
use mmu::MMU;
use ppu::Ppu;
use std::env;

mod cpu;
mod io;
mod memory;
mod mmu;
mod ppu;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("invalid argumnts");
        return;
    }

    let mut mmu = MMU::new();
    if let Err(e) = mmu.load(&args[1]) {
        println!("error {}", e);
        return;
    }

    let mut io = Io::new();
    let mut cpu = Cpu::new();
    let mut ppu = Ppu::new();

    println!("{:#?}", cpu);
    mmu.dump();

    loop {
        // loop {
        match io.get_key() {
            Some(-1) => return,
            //Some(_) => break,
            _ => (),
        }
        // }

        cpu.run(&mut mmu).unwrap();
        ppu.run(&mut mmu, &mut io).unwrap();

        // println!("{:#?}", cpu);
        // mmu.dump();
    }
}
