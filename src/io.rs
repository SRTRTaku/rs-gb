use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
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
pub enum GbKey {
    Quit,
    Run,
    Step,
    NextStep,
    //A,
    //B,
    //Right,
    //Left,
    //Up,
    //Down,
    //Start,
    //Select,
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
    pub fn draw_graphics(&mut self) {
        for y in 0..GFX_SIZE_Y {
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
        self.canvas.present();
    }
    pub fn get_key(&mut self) -> Option<GbKey> {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return Some(GbKey::Quit),
                Event::KeyDown {
                    keycode: Some(key_code),
                    ..
                } => match key_code {
                    Keycode::F5 => return Some(GbKey::Run),
                    Keycode::F7 => return Some(GbKey::Step),
                    Keycode::F10 => return Some(GbKey::NextStep),
                    _ => (),
                },
                _ => (),
            }
        }
        None
    }
}
