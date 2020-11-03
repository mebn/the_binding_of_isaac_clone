use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::audio::{Source};
use ggez::input::keyboard::{KeyCode, is_key_pressed};

pub struct Player {
	pub x_pos: f32,
    pub y_pos: f32,
    pub width: f32,
    pub height: f32,
	pub speed: f32,
	pub fire_speed: f32,
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
	
	pub fn no_wall_hax(&mut self, width: f32, height: f32) {
		if self.x_pos > width - self.width { self.x_pos = width - self.width; }
		if self.x_pos < 0.0 { self.x_pos = 0.0; }
		if self.y_pos > height - self.height { self.y_pos = height - self.height; }
		if self.y_pos < 0.0 { self.y_pos = 0.0; }
	}
}