use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::input::keyboard::{KeyCode, is_key_pressed};

use crate::window;

pub struct Player {
	pub x_pos: f32,
    pub y_pos: f32,
    pub width: f32,
    pub height: f32,
	pub speed: f32,
	pub fire_speed: f32,
	pub current_room: (usize, usize),
}

impl Player {
    // Draws a rectangle.
	pub fn rect(&self, ctx: &mut Context) -> GameResult {
		let rect = graphics::Rect::new(self.x_pos, self.y_pos, self.width, self.height);
		let rect_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::WHITE)?;
		graphics::draw(ctx, &rect_mesh, graphics::DrawParam::default())
    }
    
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

	pub fn what_door(&self) -> &str {
		let door_width = 95.0;

		// right door.
		if self.x_pos == window::WIDTH - self.width - window::INNER_WIDTH &&
		   self.y_pos > window::HEIGHT / 2.0 - door_width &&
		   self.y_pos < window::HEIGHT / 2.0 + door_width {
			   return "right";
		}

		// left door.
		if self.x_pos == window::INNER_WIDTH &&
		   self.y_pos > window::HEIGHT / 2.0 - door_width &&
		   self.y_pos < window::HEIGHT / 2.0 + door_width {
			   return "left";
		}

		// top door.
		if self.y_pos == window::INNER_WIDTH &&
		   self.x_pos > window::WIDTH / 2.0 - door_width &&
		   self.x_pos < window::WIDTH / 2.0 + door_width {
			return "top";
		}

		// bottom door.
		if self.y_pos == window::HEIGHT - self.height - window::INNER_WIDTH &&
		   self.x_pos > window::WIDTH / 2.0 - door_width &&
		   self.x_pos < window::WIDTH / 2.0 + door_width {
			return "bottom";
		}

		return "none";
	}
}