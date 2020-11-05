use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::input::keyboard::{KeyCode, is_key_pressed};

use crate::window;
use crate::mygame::{MyGame};
use crate::map;
use crate::enemy;

use cgmath::{Point2};

pub const RELOAD_TIME: f32 = 0.4;

#[derive(PartialEq)]
pub enum Door {
	LEFT = 0,
	TOP = 1,
	RIGHT = 2,
	BOTTOM = 3,
	NONE
}

pub struct Player {
	pub x_pos: f32,
    pub y_pos: f32,
    pub width: f32,
    pub height: f32,
	pub speed: f32,
	pub reload_time: f32,
	pub current_room: (usize, usize),
	pub life: i32
}

// Draws a rectangle.
pub fn draw(ctx: &mut Context, mygame: &mut MyGame) {
	let dst: Point2<f32> = Point2::new(mygame.player1.x_pos, mygame.player1.y_pos);
	
	let scale = {
		let wh = 30.0;
		let scale_f = mygame.player1.width / wh;
		[scale_f, scale_f]
	};

	let param = graphics::DrawParam::new().dest(dst).scale(scale);
	graphics::draw(ctx, &mygame.assets.head, param).unwrap();
}

impl Player {
    // Binds keypressess to movement.
    pub fn movement(&mut self, ctx: &Context) {
        if is_key_pressed(ctx, KeyCode::D) {
			self.x_pos += self.speed;
		}
		if is_key_pressed(ctx, KeyCode::A) {
			self.x_pos -= self.speed;
		}
		if is_key_pressed(ctx, KeyCode::W) {
			self.y_pos -= self.speed;
		}
		if is_key_pressed(ctx, KeyCode::S) {
			self.y_pos += self.speed;
		}
	}
	
	pub fn no_wall_hax(&mut self) {
		if self.x_pos > window::WIDTH - self.width - window::INNER_WIDTH {
			self.x_pos = window::WIDTH - self.width - window::INNER_WIDTH;
		}
		if self.x_pos < window::INNER_WIDTH {
			self.x_pos = window::INNER_WIDTH;
		}
		if self.y_pos > window::HEIGHT - self.height - window::INNER_WIDTH {
			self.y_pos = window::HEIGHT - self.height - window::INNER_WIDTH;
		}
		if self.y_pos < window::INNER_WIDTH {
			self.y_pos = window::INNER_WIDTH;
		}
	}

	pub fn is_at_door(&self) -> Door {
		let door_width = 95.0;

		// left door.
		if self.x_pos == window::INNER_WIDTH &&
		self.y_pos > window::HEIGHT / 2.0 - door_width &&
		self.y_pos < window::HEIGHT / 2.0 + door_width {
			return Door::LEFT;
	 }

		// top door.
		if self.y_pos == window::INNER_WIDTH &&
		   self.x_pos > window::WIDTH / 2.0 - door_width &&
		   self.x_pos < window::WIDTH / 2.0 + door_width {
			return Door::TOP;
		}

		// right door.
		if self.x_pos == window::WIDTH - self.width - window::INNER_WIDTH &&
		   self.y_pos > window::HEIGHT / 2.0 - door_width &&
		   self.y_pos < window::HEIGHT / 2.0 + door_width {
			   return Door::RIGHT;
		}

		// bottom door.
		if self.y_pos == window::HEIGHT - self.height - window::INNER_WIDTH &&
		   self.x_pos > window::WIDTH / 2.0 - door_width &&
		   self.x_pos < window::WIDTH / 2.0 + door_width {
			return Door::BOTTOM;
		}

		Door::NONE
	}

	pub fn ready_to_fire(&self) -> bool {
		self.reload_time < 0.0
	}

	pub fn player_still_alive(&self) -> bool {
		self.life <= 0
	}
}

fn handle_player_door_movement(mygame: &mut MyGame) {
	mygame.bullets = Vec::new();

	// check if player has already been in the room
	if !mygame.rooms[mygame.player1.current_room.0][mygame.player1.current_room.1].is_finished {
		// spawn enemies
		mygame.enemies = enemy::spawn_enemies(
			mygame.rooms[mygame.player1.current_room.0][mygame.player1.current_room.0].num_of_enemies,
			mygame.player1.x_pos,
			mygame.player1.y_pos);
	}
}

fn go_through_door(player: &mut Player, door: Door) {
	let dist_from_door = 10.0;

	if door == Door::LEFT {
		player.x_pos = window::WIDTH - window::INNER_WIDTH - player.width - dist_from_door;
		player.current_room = (player.current_room.0, player.current_room.1 - 1);
	} else if door == Door::TOP {
		player.y_pos = window::HEIGHT - window::INNER_WIDTH - player.height - dist_from_door;
		player.current_room = (player.current_room.0 - 1, player.current_room.1);
	} else if door == Door::RIGHT {
		player.x_pos = window::INNER_WIDTH + dist_from_door;
		player.current_room = (player.current_room.0, player.current_room.1 + 1);
	} else if door == Door::BOTTOM {
		player.y_pos = 0.0 + window::INNER_WIDTH + dist_from_door;
		player.current_room = (player.current_room.0 + 1, player.current_room.1);
	}
}

pub fn enter_new_room(mygame: &mut MyGame) {
	if mygame.rooms[mygame.player1.current_room.0][mygame.player1.current_room.1].is_finished {
		let possible_rooms = map::room_next(mygame.player1.current_room, &mygame.map);

		if possible_rooms[0] && mygame.player1.is_at_door() == Door::LEFT {
			go_through_door(&mut mygame.player1, Door::LEFT);
			handle_player_door_movement(mygame);
		} else if possible_rooms[1] && mygame.player1.is_at_door() == Door::TOP {
			go_through_door(&mut mygame.player1, Door::TOP);
			handle_player_door_movement(mygame);
		} else if possible_rooms[2] && mygame.player1.is_at_door() == Door::RIGHT {
			go_through_door(&mut mygame.player1, Door::RIGHT);
			handle_player_door_movement(mygame);
		} else if possible_rooms[3] && mygame.player1.is_at_door() == Door::BOTTOM {
			go_through_door(&mut mygame.player1, Door::BOTTOM);
			handle_player_door_movement(mygame);
		}
	}
}