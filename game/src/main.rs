// ggez docs: https://docs.rs/ggez/0.5.1/ggez/
// Help from: https://github.com/AndrewJakubowicz/ggezFlappyCrabby

use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::input::keyboard::{KeyCode, is_key_pressed};
use ggez::input::keyboard;
use ggez::audio::{SoundSource, Source};

mod window;
mod player;
mod bullet;

// This struct handles all the
// players and objects in the game.
struct MyGame {
	player1: player::Player,
	bullets: Vec<bullet::Bullet>
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
			is_shooting: false,
			sound: space_sound
		};

        MyGame {
			player1,
			bullets: Vec::new()
		}
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
		// Handle keypresses and movement.
		self.player1.movement(ctx, KeyCode::W, KeyCode::S, KeyCode::A, KeyCode::D);

		for bull in &mut self.bullets {
			bull.x_pos += bull.speed;
		}

		self.bullets.retain(|b| b.x_pos < window::WIDTH);

		if self.player1.is_shooting {
			// create a new Bullet and to MyGame.bullet.
			let new_bullet = bullet::Bullet {
				x_pos: self.player1.x_pos,
				y_pos: self.player1.y_pos,
				width: 30.0,
				height: 15.0,
				speed: 30.0,
				life: 1.0
			};

			self.bullets.push(new_bullet);
			// play "fire" sound
		}

		Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
		self.player1.rect(ctx)?;

		// bullets
		for bull in &self.bullets {
			bull.draw(ctx)?;
		}

        graphics::present(ctx)
	}
	
	// fire
	fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: keyboard::KeyMods, _repeat: bool) {
		if let KeyCode::Space = keycode {
			self.player1.is_shooting = true;
		}
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: keyboard::KeyMods) {
        if let KeyCode::Space = keycode {
			self.player1.is_shooting = false;
		}
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