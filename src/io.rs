use crate::memory::{MemoryIF, IF, JOYP};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

const WHITE: Color = Color::RGB(0xe0, 0xf8, 0xd0);
const LIGHT_GRAY: Color = Color::RGB(0x88, 0xc0, 0x70);
const DARK_GRAY: Color = Color::RGB(0x34, 0x68, 0x56);
const BLACK: Color = Color::RGB(0x08, 0x18, 0x20);
const PIXEL_SIZE: u32 = 3;
pub const GFX_SIZE_Y: usize = 144;
pub const GFX_SIZE_X: usize = 160;

#[derive(Copy, Clone)]
pub enum GfxColor {
    W,
    LG,
    DG,
    B,
}
pub enum EmuControl {
    Quit,
    Run,
    Step,
    NextStep,
}
#[derive(Debug)]
pub enum Joypad {
    A,
    B,
    Right,
    Left,
    Up,
    Down,
    Start,
    Select,
}
pub enum GbKey {
    Emu(EmuControl),
    Game(Joypad),
}

pub struct Io {
    canvas: WindowCanvas,
    event_pump: EventPump,
    pub gfx: [GfxColor; GFX_SIZE_X * GFX_SIZE_Y],
}

impl Io {
    pub fn new() -> Io {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(
                "rs-gb",
                GFX_SIZE_X as u32 * PIXEL_SIZE,
                GFX_SIZE_Y as u32 * PIXEL_SIZE,
            )
            .position_centered()
            .build()
            .unwrap();

        let mut _canvas = window.into_canvas().build().unwrap();
        let mut _event_pump = sdl_context.event_pump().unwrap();

        _canvas.set_draw_color(WHITE);
        _canvas.clear();
        _canvas.present();

        Io {
            canvas: _canvas,
            event_pump: _event_pump,
            gfx: [GfxColor::W; GFX_SIZE_X * GFX_SIZE_Y],
        }
    }
    pub fn draw_a_line(&mut self, y: usize) {
        for x in 0..GFX_SIZE_X {
            let _x = (x * PIXEL_SIZE as usize) as i32;
            let _y = (y * PIXEL_SIZE as usize) as i32;
            match self.gfx[y * GFX_SIZE_X + x] {
                GfxColor::W => self.canvas.set_draw_color(WHITE),
                GfxColor::LG => self.canvas.set_draw_color(LIGHT_GRAY),
                GfxColor::DG => self.canvas.set_draw_color(DARK_GRAY),
                GfxColor::B => self.canvas.set_draw_color(BLACK),
            }
            self.canvas
                .fill_rect(Rect::new(_x, _y, PIXEL_SIZE, PIXEL_SIZE))
                .unwrap();
        }
    }
    pub fn present(&mut self) {
        self.canvas.present();
    }
    pub fn get_key(&mut self, memory: &mut impl MemoryIF) -> (Option<EmuControl>, bool) {
        let mut joypads = Vec::new();
        for event in self.event_pump.poll_iter() {
            let key = match event {
                Event::Quit { .. } => Some(GbKey::Emu(EmuControl::Quit)),
                Event::KeyDown {
                    keycode: Some(key_code),
                    ..
                } => match key_code {
                    Keycode::F5 => Some(GbKey::Emu(EmuControl::Run)),
                    Keycode::F7 => Some(GbKey::Emu(EmuControl::Step)),
                    Keycode::F10 => Some(GbKey::Emu(EmuControl::NextStep)),
                    Keycode::Right => Some(GbKey::Game(Joypad::Right)),
                    Keycode::Left => Some(GbKey::Game(Joypad::Left)),
                    Keycode::Up => Some(GbKey::Game(Joypad::Up)),
                    Keycode::Down => Some(GbKey::Game(Joypad::Down)),
                    Keycode::S => Some(GbKey::Game(Joypad::A)),
                    Keycode::A => Some(GbKey::Game(Joypad::B)),
                    Keycode::Return => Some(GbKey::Game(Joypad::Start)),
                    Keycode::RShift => Some(GbKey::Game(Joypad::Select)),
                    _ => None,
                },
                _ => None,
            };
            if let Some(gb_key) = key {
                match gb_key {
                    GbKey::Emu(emu_control) => return (Some(emu_control), false),
                    GbKey::Game(joypad) => {
                        joypads.push(joypad);
                    }
                }
            }
        }
        let pressed = set_joypad_input(memory, &joypads);
        (None, pressed)
    }
}

fn set_joypad_input(memory: &mut impl MemoryIF, joypads: &[Joypad]) -> bool {
    // joypad input
    let joyp = memory.read_byte(JOYP);
    let select_buttons = joyp & 0x20 == 0x00;
    let select_dpad = joyp & 0x10 == 0x00;
    let mut joyp_out = (joyp & 0x30) + 0x0f;
    for joypad in joypads {
        if select_dpad {
            match joypad {
                Joypad::Right => joyp_out &= !0x01,
                Joypad::Left => joyp_out &= !0x02,
                Joypad::Up => joyp_out &= !0x04,
                Joypad::Down => joyp_out &= !0x08,
                _ => (),
            }
        } else if select_buttons {
            match joypad {
                Joypad::A => joyp_out &= !0x01,
                Joypad::B => joyp_out &= !0x02,
                Joypad::Select => joyp_out &= !0x04,
                Joypad::Start => joyp_out &= !0x08,
                _ => (),
            }
        }
    }
    // joypad interrupt
    let pressed = if joyp_out & 0x0f != 0x0f {
        let i_flag = memory.read_byte(IF);
        memory.write_byte(IF, i_flag | 0x10);
        true
    } else {
        false
    };

    memory.write_byte(JOYP, joyp_out);
    pressed
}
