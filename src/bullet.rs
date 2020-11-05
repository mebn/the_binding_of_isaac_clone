use ggez::graphics;
use ggez::input::keyboard::{KeyCode};
use ggez::{Context};
// use ggez::audio::{Source, SoundSource};

use crate::window;

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
			width: 20.0,
			height: 20.0,
			speed: 30.0,
			direction,
			did_hit: false,
		}
	}
}

pub fn draw(ctx: &mut Context, bullet_vec: &Vec<Bullet>) {
	for bullet in bullet_vec {
		let rect = graphics::Rect::new(bullet.x_pos, bullet.y_pos, bullet.width, bullet.height);
		let rect_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::WHITE).unwrap();
		graphics::draw(ctx, &rect_mesh, graphics::DrawParam::default()).unwrap();
	};
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

pub fn remove_old_bullets(bullet_vec: &mut Vec<Bullet>) {
	// Removes Bullet from vec if the condition is not met.
	bullet_vec.retain(|b| b.x_pos < window::WIDTH);
	bullet_vec.retain(|b| b.x_pos > 0.0 - b.width);
	bullet_vec.retain(|b| b.y_pos < window::HEIGHT);
	bullet_vec.retain(|b| b.y_pos > 0.0 - b.height);
	bullet_vec.retain(|b| !b.did_hit);
}