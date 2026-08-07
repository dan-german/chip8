use ggez::{
    Context, GameResult, graphics::{Canvas, Color, DrawMode, DrawParam, Drawable, Mesh, Rect, Text, TextFragment},
};

use crate::consts::*;

pub struct Keys;

const CHARS: &str = "123A456B789C*0#D";

impl Keys {
    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas, bounds: Rect) -> GameResult {
        for row in 0..4 {
            for col in 0..4 {
                let x = bounds.x + (col as f32) * KEY_SIZE;
                let y = bounds.y + (row as f32) * KEY_SIZE;
                let idx = (col + row * 4) as usize;
                let fragment = TextFragment::new(&CHARS[idx..idx + 1]).scale(32.0).color(Color::from_rgb(50, 50, 50));
                let text = Text::new(fragment);
                let square = Mesh::new_rectangle(
                    ctx,
                    DrawMode::fill(),
                    Rect::new(x, y, KEY_SIZE - 1.0, KEY_SIZE - 1.0),
                    Color::from_rgb(155, 255, 180),
                )?;
                canvas.draw(&square, DrawParam::default());
                let text_dim = text.dimensions(ctx);
                let text_x = x + (KEY_SIZE - text_dim.w) / 2.0;
                let text_y = y + (KEY_SIZE - text_dim.h) / 2.0;
                canvas.draw(&text, DrawParam::default().dest([text_x, text_y]));
            }
        }
        Ok(())
    }

    pub fn key_at(&self, x: f32 , y: f32) -> (i32, i32) { 
        let keypad_size = DISPLAY_HEIGHT;
        let col = (((x - PIXEL_SECTION_WIDTH) / keypad_size) * 4.0).floor();
        let row = (((y) / keypad_size) * 4.0).floor();
        return (row as i32, col as i32);
    }
}
