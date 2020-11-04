// ggez docs: https://docs.rs/ggez/0.5.1/ggez/
// Help from: https://github.com/AndrewJakubowicz/ggezFlappyCrabby

use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::input::keyboard::{KeyCode, is_key_pressed};
use ggez::input::keyboard;
// use ggez::audio::{Source};

use ggez::timer;

mod window;
mod player;
mod bullet;
mod enemy;
mod room;
mod map;

// struct Assets {
// 	shooting: Source
// }

// This struct handles all the
// players and objects in the game.
struct MyGame {
	player1: player::Player,
	bullets: Vec<bullet::Bullet>,
	enemies: Vec<enemy::Enemy>,
	// rooms: Vec<room::Room>,
	rooms: Vec<Vec<room::Room>>,
	map: Vec<Vec<bool>>
	// assets: Assets
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
		// Load/create resources here: images, fonts, sounds, etc.
		// let assets = Assets {
		// 	shooting: Source::new(ctx, "/pew.wav").unwrap()
		// };

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
			fire_speed: 0.5, // half a sec.
			current_room: map::PLAYER_SPAWN,
		};

        MyGame {
			player1,
			bullets: Vec::new(),
			enemies: Vec::new(),
			rooms,
			map: map::generate_map()
			// assets
		}
	}

	fn do_stuff(&mut self, direction: u32, ctx: &mut Context) {
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

			for enemy in &self.enemies {
				enemy.draw(ctx).unwrap();
			}
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
				
				self.do_stuff(0, ctx);
			}

			if possible_rooms[1] && self.player1.what_door() == "top" {
				self.player1.y_pos = window::HEIGHT - window::INNER_WIDTH - self.player1.height * 2.0;
				
				self.do_stuff(1, ctx);
			}
			
			if possible_rooms[2] && self.player1.what_door() == "right" {
				self.player1.x_pos = window::INNER_WIDTH + self.player1.width;
				
				self.do_stuff(2, ctx);
			}

			if possible_rooms[3] && self.player1.what_door() == "bottom" {
				self.player1.y_pos = 0.0 + window::INNER_WIDTH + self.player1.height;
				
				self.do_stuff(3, ctx);
			}
		}

		// if self.player1.what_door() &&
		//    self.rooms[self.player1.current_room].is_finished {
		// 	// move player
		// 	self.player1.x_pos = 100.0;
		// 	self.player1.y_pos = 100.0;

		// 	self.rooms[self.player1.current_room].player_is_here = false;
		// 	self.player1.current_room += 1;

		// 	self.rooms.push(room::Room::new(self.player1.current_room));

		// 	self.enemies = enemy::spawn_enemies(
		// 		self.rooms[self.player1.current_room].num_of_enemies,
		// 		window::WIDTH,
		// 		window::HEIGHT,
		// 		self.player1.x_pos,
		// 		self.player1.y_pos);
		// }

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
		self.player1.rect(ctx)?;

		// Draw bullets.
		for bull in &self.bullets {
			bull.draw(ctx)?;
		}

		// Draw enemies.
		for enemy in &self.enemies {
			enemy.draw(ctx)?;
		}

		// Draw doors.
		// left
		if map::room_next(self.player1.current_room, &self.map)[0] {
			let rect = graphics::Rect::new(0.0, window::HEIGHT / 2.0, 20.0, 100.0);
			let rect_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::WHITE)?;
			graphics::draw(ctx, &rect_mesh, graphics::DrawParam::default())?;
		}
		// top
		if map::room_next(self.player1.current_room, &self.map)[1] {
			let rect = graphics::Rect::new(window::WIDTH / 2.0, 0.0, 100.0, 20.0);
			let rect_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::WHITE)?;
			graphics::draw(ctx, &rect_mesh, graphics::DrawParam::default())?;
		}
		// right
		if map::room_next(self.player1.current_room, &self.map)[2] {
			let rect = graphics::Rect::new(window::WIDTH - 20.0, window::HEIGHT / 2.0, 20.0, 100.0);
			let rect_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::WHITE)?;
			graphics::draw(ctx, &rect_mesh, graphics::DrawParam::default())?;
		}
		// bottom
		if map::room_next(self.player1.current_room, &self.map)[3] {
			let rect = graphics::Rect::new(window::WIDTH / 2.0, window::HEIGHT - 20.0, 100.0, 20.0);
			let rect_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::WHITE)?;
			graphics::draw(ctx, &rect_mesh, graphics::DrawParam::default())?;
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