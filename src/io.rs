use crate::memory::{MemoryIF, IF, JOYP};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::render::WindowCanvas;
use sdl2::video::WindowContext;
use sdl2::EventPump;

const WHITE: (u8, u8, u8) = (0xe0, 0xf8, 0xd0);
const LIGHT_GRAY: (u8, u8, u8) = (0x88, 0xc0, 0x70);
const DARK_GRAY: (u8, u8, u8) = (0x34, 0x68, 0x56);
const BLACK: (u8, u8, u8) = (0x08, 0x18, 0x20);
const PIXEL_SIZE: usize = 3;
const JOYPAD_NUM: usize = 8;
pub const GFX_SIZE_Y: usize = 144;
pub const GFX_SIZE_X: usize = 160;

#[derive(Copy, Clone, PartialEq)]
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
    GameKeyDown(Joypad),
    GameKeyUp(Joypad),
}

pub struct Io {
    canvas: WindowCanvas,
    event_pump: EventPump,
    texture_creator: TextureCreator<WindowContext>,
    joypad_state: [bool; JOYPAD_NUM],
    pub gfx: [GfxColor; GFX_SIZE_X * GFX_SIZE_Y],
}

impl Io {
    pub fn new() -> Io {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(
                "rs-gb",
                (GFX_SIZE_X * PIXEL_SIZE) as u32,
                (GFX_SIZE_Y * PIXEL_SIZE) as u32,
            )
            .position_centered()
            .build()
            .unwrap();

        let mut _canvas = window.into_canvas().build().unwrap();
        let mut _event_pump = sdl_context.event_pump().unwrap();

        let _texture_creator = _canvas.texture_creator();

        _canvas.set_draw_color(WHITE);
        _canvas.clear();
        _canvas.present();

        Io {
            canvas: _canvas,
            event_pump: _event_pump,
            texture_creator: _texture_creator,
            joypad_state: [false; JOYPAD_NUM],
            gfx: [GfxColor::W; GFX_SIZE_X * GFX_SIZE_Y],
        }
    }
    pub fn present(&mut self) {
        let mut texture = self
            .texture_creator
            .create_texture_streaming(
                PixelFormatEnum::RGB24,
                (GFX_SIZE_X * PIXEL_SIZE) as u32,
                (GFX_SIZE_Y * PIXEL_SIZE) as u32,
            )
            .unwrap();
        texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for _x in 0..(GFX_SIZE_X * PIXEL_SIZE) {
                    for _y in 0..(GFX_SIZE_Y * PIXEL_SIZE) {
                        let x = _x / PIXEL_SIZE;
                        let y = _y / PIXEL_SIZE;
                        let (r, g, b) = match self.gfx[y * GFX_SIZE_X + x] {
                            GfxColor::W => WHITE,
                            GfxColor::LG => LIGHT_GRAY,
                            GfxColor::DG => DARK_GRAY,
                            GfxColor::B => BLACK,
                        };
                        let offset = _y * pitch + _x * 3;
                        buffer[offset] = r;
                        buffer[offset + 1] = g;
                        buffer[offset + 2] = b;
                    }
                }
            })
            .unwrap();
        self.canvas
            .copy(
                &texture,
                None,
                Rect::new(
                    0,
                    0,
                    (GFX_SIZE_X * PIXEL_SIZE) as u32,
                    (GFX_SIZE_Y * PIXEL_SIZE) as u32,
                ),
            )
            .unwrap();

        self.canvas.present();
    }
    pub fn get_key(&mut self, memory: &mut impl MemoryIF) -> (Option<EmuControl>, bool) {
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
                    Keycode::Right => Some(GbKey::GameKeyDown(Joypad::Right)),
                    Keycode::Left => Some(GbKey::GameKeyDown(Joypad::Left)),
                    Keycode::Up => Some(GbKey::GameKeyDown(Joypad::Up)),
                    Keycode::Down => Some(GbKey::GameKeyDown(Joypad::Down)),
                    Keycode::S => Some(GbKey::GameKeyDown(Joypad::A)),
                    Keycode::A => Some(GbKey::GameKeyDown(Joypad::B)),
                    Keycode::Return => Some(GbKey::GameKeyDown(Joypad::Start)),
                    Keycode::Space => Some(GbKey::GameKeyDown(Joypad::Select)),
                    _ => None,
                },
                Event::KeyUp {
                    keycode: Some(key_code),
                    ..
                } => match key_code {
                    Keycode::Right => Some(GbKey::GameKeyUp(Joypad::Right)),
                    Keycode::Left => Some(GbKey::GameKeyUp(Joypad::Left)),
                    Keycode::Up => Some(GbKey::GameKeyUp(Joypad::Up)),
                    Keycode::Down => Some(GbKey::GameKeyUp(Joypad::Down)),
                    Keycode::S => Some(GbKey::GameKeyUp(Joypad::A)),
                    Keycode::A => Some(GbKey::GameKeyUp(Joypad::B)),
                    Keycode::Return => Some(GbKey::GameKeyUp(Joypad::Start)),
                    Keycode::Space => Some(GbKey::GameKeyUp(Joypad::Select)),
                    _ => None,
                },
                _ => None,
            };
            if let Some(gb_key) = key {
                match gb_key {
                    GbKey::Emu(emu_control) => return (Some(emu_control), false),
                    GbKey::GameKeyDown(joypad) => self.joypad_state[joypad as usize] = true,
                    GbKey::GameKeyUp(joypad) => self.joypad_state[joypad as usize] = false,
                }
            }
        }
        //self.set_joypad_state(joypads);
        //dbg!(&joypads);
        let joyp = memory.read_byte(JOYP);
        let (pressed, joyp_out) = self.set_joypad_input(joyp);
        memory.write_byte(JOYP, joyp_out);
        if pressed {
            let i_flag = memory.read_byte(IF);
            memory.write_byte(IF, i_flag | 0x10);
        }
        (None, pressed)
    }

    fn set_joypad_input(&self, joyp: u8) -> (bool, u8) {
        let select_buttons = joyp & 0x20 == 0;
        let select_dpad = joyp & 0x10 == 0;
        let mut joyp_out = (joyp & 0x30) + 0x0f;
        if select_dpad {
            if self.joypad_state[Joypad::Right as usize] {
                joyp_out &= !0x01
            }
            if self.joypad_state[Joypad::Left as usize] {
                joyp_out &= !0x02
            }
            if self.joypad_state[Joypad::Up as usize] {
                joyp_out &= !0x04
            }
            if self.joypad_state[Joypad::Down as usize] {
                joyp_out &= !0x08
            }
        } else if select_buttons {
            if self.joypad_state[Joypad::A as usize] {
                joyp_out &= !0x01
            }
            if self.joypad_state[Joypad::B as usize] {
                joyp_out &= !0x02
            }
            if self.joypad_state[Joypad::Select as usize] {
                joyp_out &= !0x04
            }
            if self.joypad_state[Joypad::Start as usize] {
                joyp_out &= !0x08
            }
        }
        // joypad interrupt
        let pressed = joyp_out & 0x0f != 0x0f;

        (pressed, joyp_out)
    }
}
