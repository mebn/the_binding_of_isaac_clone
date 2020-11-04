// ggez docs: https://docs.rs/ggez/0.5.1/ggez/
// Help from: https://github.com/AndrewJakubowicz/ggezFlappyCrabby

use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::graphics::{Image};
use ggez::input::keyboard::{KeyCode, is_key_pressed};
use ggez::input::keyboard;
use ggez::audio::{Source};

use cgmath::{Point2};

use ggez::timer;

mod window;
mod player;
mod bullet;
mod enemy;
mod room;
mod map;

struct Assets {
	shooting: Source,
	basements: graphics::Image,
	left_door_open: graphics::Image,
	left_door_closed: graphics::Image,
	right_door_open: graphics::Image,
	right_door_closed: graphics::Image,
	top_door_open: graphics::Image,
	top_door_closed: graphics::Image,
	bot_door_open: graphics::Image,
	bot_door_closed: graphics::Image,
}

struct MyGame {
	player1: player::Player,
	bullets: Vec<bullet::Bullet>,
	enemies: Vec<enemy::Enemy>,
	rooms: Vec<Vec<room::Room>>,
	map: Vec<Vec<bool>>,
	assets: Assets
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
		// Load/create resources here: images, fonts, sounds, etc.
		let assets = Assets {
			shooting: Source::new(ctx, "/pew.wav").unwrap(),
			basements: Image::new(ctx, "/bg.png").unwrap(),
			left_door_open: Image::new(ctx, "/left_door_open.png").unwrap(),
			left_door_closed: Image::new(ctx, "/left_door_closed.png").unwrap(),
			top_door_open: Image::new(ctx, "/top_door_open.png").unwrap(),
			top_door_closed: Image::new(ctx, "/top_door_closed.png").unwrap(),
			right_door_open: Image::new(ctx, "/right_door_open.png").unwrap(),
			right_door_closed: Image::new(ctx, "/right_door_closed.png").unwrap(),
			bot_door_open: Image::new(ctx, "/bot_door_open.png").unwrap(),
			bot_door_closed: Image::new(ctx, "/bot_door_closed.png").unwrap(),
		};

		// Rooms
		let mut rooms: Vec<Vec<room::Room>> = Vec::new();
		for row in 0..13 {
			let mut row_vec: Vec<room::Room> = Vec::new();
			for col in 0..13 {
				row_vec.push(room::Room::new((row, col)));
			}

			rooms.push(row_vec);
		}
		rooms[map::PLAYER_SPAWN.0][map::PLAYER_SPAWN.1] = room::Room::init();


		let player1 = player::Player {
			x_pos: window::WIDTH / 2.0,
			y_pos: window::HEIGHT / 2.0,
			width: 60.0,
			height: 60.0,
			speed: 7.0,
			fire_speed: 0.5, // half a sec reload time.
			current_room: map::PLAYER_SPAWN,
		};

        MyGame {
			player1,
			bullets: Vec::new(),
			enemies: Vec::new(),
			rooms,
			map: map::generate_map(),
			assets
		}
	}

	fn handle_player_door_movement(&mut self, direction: u32) {
		self.bullets = Vec::new();
		self.rooms[self.player1.current_room.0][self.player1.current_room.1].player_is_here = false;

		if direction == 0 {
			self.player1.current_room = (self.player1.current_room.0, self.player1.current_room.1 - 1);
		} else if direction == 1 {
			self.player1.current_room = (self.player1.current_room.0 - 1, self.player1.current_room.1);
		}else if direction == 2 {
			self.player1.current_room = (self.player1.current_room.0, self.player1.current_room.1 + 1);
		} else if direction == 3 {
			self.player1.current_room = (self.player1.current_room.0 + 1, self.player1.current_room.1);
		}
	
		// check if player has already been in the room
		if !self.rooms[self.player1.current_room.0][self.player1.current_room.1].is_finished {
			// spawn enemies
			self.enemies = enemy::spawn_enemies(
				self.rooms[self.player1.current_room.0][self.player1.current_room.0].num_of_enemies,
				self.player1.x_pos,
				self.player1.y_pos);
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
		self.player1.no_wall_hax();

		// Check if player killed all enemies and if so, mark the room.
		if self.enemies.is_empty() {
			self.rooms[self.player1.current_room.0][self.player1.current_room.1].is_finished = true;
		}

		// check if player enters new room.
		if self.rooms[self.player1.current_room.0][self.player1.current_room.1].is_finished {
			let possible_rooms = map::room_next(self.player1.current_room, &self.map);

			if possible_rooms[0] && self.player1.what_door() == "left" {
				self.player1.x_pos = window::WIDTH - window::INNER_WIDTH - self.player1.width * 2.0;
				self.handle_player_door_movement(0);
			} else if possible_rooms[1] && self.player1.what_door() == "top" {
				self.player1.y_pos = window::HEIGHT - window::INNER_WIDTH - self.player1.height * 2.0;
				self.handle_player_door_movement(1);
			} else if possible_rooms[2] && self.player1.what_door() == "right" {
				self.player1.x_pos = window::INNER_WIDTH + self.player1.width;
				self.handle_player_door_movement(2);
			} else if possible_rooms[3] && self.player1.what_door() == "bottom" {
				self.player1.y_pos = 0.0 + window::INNER_WIDTH + self.player1.height;
				self.handle_player_door_movement(3);
			}
		}

		// Moves the bullet.
		bullet::move_bullet(&mut self.bullets);
		// Removes bullets outside of window.
		bullet::remove_old_bullets(&mut self.bullets);
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

		// remove dead eneemies.
		enemy::remove_the_dead(&mut self.enemies);

		Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		graphics::clear(ctx, graphics::BLACK);

		let dst: Point2<f32> = Point2::new(0.0, 0.0);
		let param = graphics::DrawParam::new().dest(dst);

		// Draw background.
		graphics::draw(ctx, &self.assets.basements, param)?;
		
		self.player1.rect(ctx)?;
		bullet::draw(ctx, &self.bullets);
		enemy::draw(ctx, &self.enemies);

		// Draw doors. (VERY temporary code).
		if self.rooms[self.player1.current_room.0][self.player1.current_room.1].is_finished {
			// left
			if map::room_next(self.player1.current_room, &self.map)[0] {
				graphics::draw(ctx, &self.assets.left_door_open, param)?;
			}
			// top
			if map::room_next(self.player1.current_room, &self.map)[1] {
				graphics::draw(ctx, &self.assets.top_door_open, param)?;
			}
			// right
			if map::room_next(self.player1.current_room, &self.map)[2] {
				graphics::draw(ctx, &self.assets.right_door_open, param)?;
			}
			// bottom
			if map::room_next(self.player1.current_room, &self.map)[3] {
				graphics::draw(ctx, &self.assets.bot_door_open, param)?;
			}
		} else {
			// left
			if map::room_next(self.player1.current_room, &self.map)[0] {
				graphics::draw(ctx, &self.assets.left_door_closed, param)?;
			}
			// top
			if map::room_next(self.player1.current_room, &self.map)[1] {
				graphics::draw(ctx, &self.assets.top_door_closed, param)?;
			}
			// right
			if map::room_next(self.player1.current_room, &self.map)[2] {
				graphics::draw(ctx, &self.assets.right_door_closed, param)?;
			}
			// bottom
			if map::room_next(self.player1.current_room, &self.map)[3] {
				graphics::draw(ctx, &self.assets.bot_door_closed, param)?;
			}
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