use cpu::Cpu;
use io::{EmuControl, GfxColor, Io, GFX_SIZE_X, GFX_SIZE_Y};
use mmu::MMU;
use ppu::Ppu;
use std::env;
use timer::Timer;

mod cpu;
mod io;
mod memory;
mod mmu;
mod ppu;
mod timer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("invalid argumnts");
        return;
    }

    let mut mmu = MMU::new();
    if let Err(e) = mmu.load(&args[1]) {
        println!("error {}", e);
        return;
    }

    let op_break_addr: Option<u16> = if args.len() >= 3 {
        Some(
            args[2]
                .trim()
                .parse()
                .expect("pc_break: cannot parse to u16"),
        )
    } else {
        None
    };

    let mut io = Io::new();
    let mut cpu = Cpu::new();
    let mut ppu = Ppu::new();
    let mut timer = Timer::new();

    println!("{}", cpu);
    mmu.dump(0x100);

    let mut f_step = false; // step execution

    loop {
        let mut key_pressed = false;
        loop {
            let (emu_control, _pressed) = io.get_key(&mut mmu);
            key_pressed = _pressed;
            match emu_control {
                Some(EmuControl::Quit) => return,
                Some(EmuControl::Run) => {
                    f_step = false;
                    break;
                }
                Some(EmuControl::Step) => {
                    f_step = true;
                    break;
                }
                Some(EmuControl::NextStep) => break,
                _ => (),
            }
            if !f_step {
                break;
            }
        }

        let pc = cpu.run(&mut mmu, key_pressed).unwrap();
        ppu.run(&mut mmu, &mut io).unwrap();
        timer.run(&mut mmu).unwrap();

        if let Some(break_addr) = op_break_addr {
            if pc == break_addr {
                f_step = true;
            }
        }
        if f_step {
            print!("\x1b[1;1H");
            print!("\x1b[2J");
            print!("{}", cpu);
            mmu.dump(pc);
        }
    }
}
