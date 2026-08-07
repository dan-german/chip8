mod consts;

use consts::*;
mod display;
use display::*;
mod keys;
use keys::*;

use ggez::{
    Context, ContextBuilder, GameResult,
    conf::WindowMode,
    event::{self, EventHandler},
    graphics::{Canvas, Color, Rect},
};

struct Game {
    display: Display,
    keys: Keys,
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

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: ggez::winit::event::MouseButton,
        x: f32,
        y: f32,
    ) -> Result<(), ggez::GameError> {
        let (r, c) = self.keys.key_at(x, y);
        println!("{}, {}", c, r);
        Ok(())
    }
}

fn main() -> GameResult {
    let pixels = [[false; PIXEL_COLS]; PIXEL_ROWS];
    // pixels[0][0] = true;
    // pixels[31][63] = true;

    let (ctx, event_loop) = ContextBuilder::new("my_game", "mew")
        .window_mode(WindowMode::default().dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT))
        .build()?;
    let game = Game {
        display: Display { pixels },
        keys: Keys,
    };
    event::run(ctx, event_loop, game)
}
