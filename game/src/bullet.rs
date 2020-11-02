use ggez::graphics;
use ggez::input::keyboard::{KeyCode};
use ggez::{Context, GameResult};

pub struct Bullet {
	pub x_pos: f32,
	pub y_pos: f32,
	pub width: f32,
	pub height: f32,
	pub speed: f32,
	pub direction: KeyCode,
	pub did_hit: bool,
}

impl Bullet {
	pub fn new(x_pos: f32, y_pos: f32, direction: KeyCode) -> Bullet {
		Bullet {
			x_pos,
			y_pos,
			width: if direction == KeyCode::Up || direction == KeyCode::Down { 15.0 } else { 30.0 },
			height: if direction == KeyCode::Up || direction == KeyCode::Down { 30.0 } else { 15.0 },
			speed: 30.0,
			direction,
			did_hit: false,
		}
	}

	pub fn draw(&self, ctx: &mut Context) -> GameResult {
		let rect = graphics::Rect::new(self.x_pos, self.y_pos, self.width, self.height);
		let rect_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::WHITE)?;
		graphics::draw(ctx, &rect_mesh, graphics::DrawParam::default())
	}
}

pub fn move_bullet(bullet_vec: &mut Vec<Bullet>) {
	for bull in bullet_vec {
		match bull.direction {
			KeyCode::Right => bull.x_pos += bull.speed,
			KeyCode::Left => bull.x_pos -= bull.speed,
			KeyCode::Up => bull.y_pos -= bull.speed,
			KeyCode::Down => bull.y_pos += bull.speed,
			_ => ()
		}
	}
}

pub fn remove_old_bullets(bullet_vec: &mut Vec<Bullet>, w: f32, h: f32) {
	// Removes Bullet from vec if
	// the condition is not met.
	bullet_vec.retain(|b| b.x_pos < w);
	bullet_vec.retain(|b| b.x_pos > 0.0 - b.width);
	bullet_vec.retain(|b| b.y_pos < h);
	bullet_vec.retain(|b| b.y_pos > 0.0 - b.height);
}