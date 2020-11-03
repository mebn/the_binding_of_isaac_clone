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
mod enemy;

struct Assets {
	shooting: Source
}

// This struct handles all the
// players and objects in the game.
struct MyGame {
	player1: player::Player,
	bullets: Vec<bullet::Bullet>,
	enemies: Vec<enemy::Enemy>,
	assets: Assets
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
		// Load/create resources here: images, fonts, sounds, etc.
		let assets = Assets {
			shooting: Source::new(_ctx, "/pew.wav").unwrap()
		};

		let x_pos = 100.0;
		let y_pos = 100.0;
		let player1 = player::Player {
			x_pos,
			y_pos,
			width: 25.0,
			height: 25.0,
			speed: 7.0,
			fire_speed: 0.5, // half a sec.
		};

        MyGame {
			player1,
			bullets: Vec::new(),
			enemies: enemy::spawn_enemies(5, window::WIDTH, window::HEIGHT, x_pos, y_pos),
			assets
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

		// remove dead eneemies.
		enemy::remove_the_dead(&mut self.enemies);

		// Updates enemies positions.
		for ene in &mut self.enemies {
			ene.update_pos(self.player1.x_pos, self.player1.y_pos);
		}

		// Bullet hit enemy.
		for bull in &mut self.bullets {
			for ene in &mut self.enemies {
				// Copied from https://developer.mozilla.org/en-US/docs/Games/Techniques/2D_collision_detection
				if bull.x_pos < ene.x_pos + ene.width &&
				   bull.x_pos + bull.width > ene.x_pos &&
				   bull.y_pos < ene.y_pos + ene.height &&
				   bull.y_pos + bull.height > ene.y_pos {
					ene.is_alive = false;
					bull.did_hit = true;
				}
			}
		}

		if self.enemies.is_empty() {
			println!("You killed them all! You monster!");
			// Wait for player to enter new room
			// then spawn more enemies.
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

		// Draw enemies.
		for ene in &self.enemies {
			ene.draw(ctx)?;
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