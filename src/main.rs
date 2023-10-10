use cpu::Cpu;
use mmu::MMU;
use ppu::Ppu;
use std::env;
use std::io;

mod cpu;
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

    let mut cpu = Cpu::new();
    let mut ppu = Ppu::new();

    println!("{:#?}", cpu);
    mmu.dump();

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();

        cpu.run(&mut mmu).unwrap();
        ppu.run(&mut mmu).unwrap();

        println!("{:#?}", cpu);
        mmu.dump();
    }
}
