use crate::keys::*;
use crate::consts::*;
use crate::display::*;
use crate::cpu::*;

use ggez::{
    Context, GameResult,
    event::{EventHandler},
    graphics::{Canvas, Color, Rect},
};

pub struct Game {
    pub display: Display,
    pub keys: Keys,
    pub cpu: CPU,
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(150, 150, 150));
        let left = Rect::new(0.0, 0.0, PIXEL_SECTION_WIDTH, DISPLAY_HEIGHT);
        let right = Rect::new(left.w, 0.0, DISPLAY_HEIGHT, DISPLAY_HEIGHT);
        self.display.draw(ctx, &mut canvas, left)?;
        self.keys.draw(ctx, &mut canvas, right)?;
        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: ggez::winit::event::MouseButton,
        x: f32,
        y: f32,
    ) -> Result<(), ggez::GameError> {
        let (r, c) = self.keys.key_at(x, y);
        if r > 0 && c > 0 { 
            self.cpu.keypad[(c + r * 3) as usize] = true;
        }
        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: ggez::winit::event::MouseButton,
        x: f32,
        y: f32,
    ) -> Result<(), ggez::GameError> {
        let (r, c) = self.keys.key_at(x, y);
        if r > 0 && c > 0 { 
            self.cpu.keypad[(c + r * 3) as usize] = false;
        }
        Ok(())
    }
}
