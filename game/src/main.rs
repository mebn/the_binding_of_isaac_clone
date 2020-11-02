// ggez docs: https://docs.rs/ggez/0.5.1/ggez/
// Help from: https://github.com/AndrewJakubowicz/ggezFlappyCrabby

use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::input::keyboard::{KeyCode, is_key_pressed};
use ggez::input::keyboard;
use ggez::audio::{Source};

use ggez::timer;

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
			fire_speed: 0.5, // half a sec.
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
		// Handle cooldown when shooting bullets.
		const DESIRED_FPS: u32 = 60;
		while timer::check_update_time(ctx, DESIRED_FPS) {
			let seconds = 1.0 / (DESIRED_FPS as f32);
			self.player1.fire_speed -= seconds;
		}

		// Handle movement for player1.
		self.player1.movement(ctx);
		self.player1.no_wall_hax(window::WIDTH, window::HEIGHT);

		// Moves the bullet.
		bullet::move_bullet(&mut self.bullets);
		// Removes bullets outside of window.
		bullet::remove_old_bullets(&mut self.bullets, window::WIDTH, window::HEIGHT);
		// Creates a new bullet on correct keypress.
		if is_key_pressed(ctx, keyboard::KeyCode::Right) && self.player1.fire_speed < 0.0 {
			let new_bullet = bullet::Bullet::new(self.player1.x_pos, self.player1.y_pos, KeyCode::Right);
			self.bullets.push(new_bullet);
			self.player1.fire_speed = 0.5;
		}else if is_key_pressed(ctx, keyboard::KeyCode::Left) && self.player1.fire_speed < 0.0 {
			let new_bullet = bullet::Bullet::new(self.player1.x_pos, self.player1.y_pos, KeyCode::Left);
			self.bullets.push(new_bullet);
			self.player1.fire_speed = 0.5;
		}else if is_key_pressed(ctx, keyboard::KeyCode::Up) && self.player1.fire_speed < 0.0 {
			let new_bullet = bullet::Bullet::new(self.player1.x_pos, self.player1.y_pos, KeyCode::Up);
			self.bullets.push(new_bullet);
			self.player1.fire_speed = 0.5;
		}else if is_key_pressed(ctx, keyboard::KeyCode::Down) && self.player1.fire_speed < 0.0 {
			let new_bullet = bullet::Bullet::new(self.player1.x_pos, self.player1.y_pos, KeyCode::Down);
			self.bullets.push(new_bullet);
			self.player1.fire_speed = 0.5;
		}

		Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
		self.player1.rect(ctx)?;

		// Draw bullets.
		for bull in &self.bullets {
			bull.draw(ctx)?;
		}

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