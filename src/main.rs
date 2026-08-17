use chip8::lib::*;

use ggez::{
    ContextBuilder, GameResult,
    conf::WindowMode,
    event::{self},
};

fn main() -> GameResult {
    let pixels = [[false; PIXEL_COLS]; PIXEL_ROWS];
    // pixels[0][0] = true;
    // pixels[31][63] = true;

    let (ctx, event_loop) = ContextBuilder::new("my_game", "me")
        .window_mode(WindowMode::default().dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT))
        .build()?;
    let game = Game {
        display: Display { pixels },
        keys: Keys,
        cpu: CPU::new()
    };
    event::run(ctx, event_loop, game)
}
