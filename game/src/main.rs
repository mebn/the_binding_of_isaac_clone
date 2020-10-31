// ggez docs: https://docs.rs/ggez/0.5.1/ggez/
// Help from: https://github.com/AndrewJakubowicz/ggezFlappyCrabby

use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::input::keyboard::{KeyCode, is_key_pressed};
use ggez::audio::{SoundSource, Source};

mod window;
mod player;

// This struct handles all the
// players and objects in the game.
struct MyGame {
	player1: player::Player
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
		// Load/create resources here: images, fonts, sounds, etc.
		let space_sound: Source = Source::new(_ctx, "/coin.wav").unwrap();

		let player1 = player::Player {
			x_pos: 100.0,
			y_pos: 100.0,
			width: 50.0,
			height: 50.0,
			speed: 10.0,
			sound: space_sound
		};

        MyGame {
			player1
		}
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
		// Handle keypresses and movement.
		self.player1.movement(_ctx, KeyCode::W, KeyCode::S, KeyCode::A, KeyCode::D);
		if is_key_pressed(_ctx, event::KeyCode::Space) {
			self.player1.sound.play()?;
		}

		Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
		
		// Draws a rectangle on the screen.
		self.player1.rect(ctx)?;

        graphics::present(ctx)
    }
}

fn main() {
	let (mut ctx, mut event_loop) = window::build_window();
	let mut my_game = MyGame::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}