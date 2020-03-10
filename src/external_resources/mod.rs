use sdl2::pixels;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub mod input;
pub mod rom;
pub mod screen;

const SCALE_FACTOR: u32 = 20;
const SCREEN_WIDTH: u32 = 64 * SCALE_FACTOR;
const SCREEN_HEIGHT: u32 = 32 * SCALE_FACTOR;

pub struct Input {
    events: sdl2::EventPump,
}

pub struct Screen {
    canvas: Canvas<Window>,
}

impl Input {
    pub fn new(sdl_context: &sdl2::Sdl) -> Result<Self, String> {
        let events = sdl_context
            .event_pump()
            .or_else(|e| Err(format!("cannot create SDL event: {}", e)))?;

        Ok(Input { events })
    }
}

impl Screen {
    pub fn new(sdl_context: &sdl2::Sdl) -> Result<Self, String> {
        let video_subsys = sdl_context.video().unwrap();
        let window = video_subsys
            .window(
                "rust-sdl2_gfx: draw line & FPSManager",
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
            )
            .position_centered()
            .opengl()
            .build()
            .or_else(|err| Err(format!("cannot init video subsystem {}", err)))?;

        let canvas = window.into_canvas().build();

        match canvas {
            Ok(mut canv) => {
                canv.set_draw_color(pixels::Color::RGB(0, 0, 0));
                canv.clear();
                canv.present();

                Ok(Screen { canvas: canv })
            }
            Err(e) => return Err(format!("can't create canvas: {}", e)),
        }
    }
}
