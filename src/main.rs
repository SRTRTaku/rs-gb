use mmu::MMU;
use std::env;

mod cpu;
mod memory;
mod mmu;

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

    mmu.dump();
}
