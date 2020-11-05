use ggez::graphics;
use ggez::input::keyboard::{KeyCode, is_key_pressed};
use ggez::{Context};

use crate::window;
use crate::player::{Player, RELOAD_TIME};

use crate::mygame::{MyGame};
use cgmath::{Point2};

const BULLET_WH: f32 =  40.0;

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
			width: BULLET_WH,
			height: BULLET_WH,
			speed: 20.0,
			direction,
			did_hit: false,
		}
	}
}

pub fn draw(ctx: &mut Context, mygame: &mut MyGame) {
	for bullet in &mygame.bullets {
		let dst: Point2<f32> = Point2::new(bullet.x_pos, bullet.y_pos);
	
		let scale = {
			let wh = 30.0;
			let scale_f = BULLET_WH / wh;
			[scale_f, scale_f]
		};

		let param = graphics::DrawParam::new().dest(dst).scale(scale);
		graphics::draw(ctx, &mygame.assets.bullet, param).unwrap();
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

fn fire_a_bullet(bullet_vec: &mut Vec<Bullet>, player: &mut Player, direction: KeyCode) {
	let x = player.x_pos + (player.width - BULLET_WH) / 2.0;
	let y = player.y_pos + (player.height - BULLET_WH) / 2.0;
	
	let new_bullet = Bullet::new(x, y, direction);
	bullet_vec.push(new_bullet);
	player.reload_time = RELOAD_TIME;
}

pub fn fire_new_bullet_on_keypress(ctx: &Context, bullet_vec: &mut Vec<Bullet>, player: &mut Player) {
	if is_key_pressed(ctx, KeyCode::Right) && player.ready_to_fire() {
		fire_a_bullet(bullet_vec, player, KeyCode::Right);
	}else if is_key_pressed(ctx, KeyCode::Left) && player.ready_to_fire() {
		fire_a_bullet(bullet_vec, player, KeyCode::Left);
	}else if is_key_pressed(ctx, KeyCode::Up) && player.ready_to_fire() {
		fire_a_bullet(bullet_vec, player, KeyCode::Up);
	}else if is_key_pressed(ctx, KeyCode::Down) && player.ready_to_fire() {
		fire_a_bullet(bullet_vec, player, KeyCode::Down);
	}
}