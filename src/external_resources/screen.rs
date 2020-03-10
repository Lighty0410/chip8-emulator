use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;

use super::SCALE_FACTOR;

impl super::Screen {
    pub fn draw(&mut self, pixels: &[[u8; 64]; 32]) -> Result<(), String> {
        for (y, row) in pixels.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = (x as u32) * SCALE_FACTOR;
                let y = (y as u32) * SCALE_FACTOR;

                let color = color(col);

                self.canvas.set_draw_color(color);

                self.canvas
                    .fill_rect(Rect::new(x as i32, y as i32, SCALE_FACTOR, SCALE_FACTOR))
                    .or_else(|e| Err(format!("cannot draw rectangle: {}", e)))?;
            }
        }
        Ok(())
    }
}

fn color(value: u8) -> pixels::Color {
    if value == 0 {
        pixels::Color::RGB(0, 0, 0)
    } else {
        pixels::Color::RGB(0, 250, 0)
    }
}
