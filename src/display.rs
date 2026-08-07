use ggez::{
    Context, GameResult,
    graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect},
};
use crate::consts::*;

pub struct Display {
    pub pixels: [[bool; PIXEL_COLS]; PIXEL_ROWS],
}

impl Display {
    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas, bounds: Rect) -> GameResult {
        for row in 0..PIXEL_ROWS {
            for col in 0..PIXEL_COLS {
                let x = bounds.x + (col as f32) * (PIXEL_SIZE + 1.0);
                let y = bounds.y + (row as f32) * (PIXEL_SIZE + 1.0);
                let color = if self.pixels[row][col] {
                    Color::from_rgb(200, 200, 200)
                } else {
                    Color::from_rgb(50, 50, 50)
                };
                let square = Mesh::new_rectangle(
                    ctx,
                    DrawMode::fill(),
                    Rect::new(x, y, PIXEL_SIZE, PIXEL_SIZE),
                    color,
                )?;
                canvas.draw(&square, DrawParam::default());
            }
        }
        Ok(())
    }
}
