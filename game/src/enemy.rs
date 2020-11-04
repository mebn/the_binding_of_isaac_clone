use ggez::{Context};
use ggez::graphics;
// use ggez::audio::{Source};
use rand::Rng;

use crate::window;

pub struct Enemy {
	pub x_pos: f32,
    pub y_pos: f32,
    pub width: f32,
    pub height: f32,
    pub speed: f32,
    pub is_alive: bool
}

impl Enemy {
    pub fn update_pos(&mut self, player_x: f32, player_y: f32) {
        if self.x_pos >= player_x { self.x_pos -= self.speed; }
        if self.x_pos <= player_x { self.x_pos += self.speed; }
        if self.y_pos >= player_y { self.y_pos -= self.speed; }
        if self.y_pos <= player_y { self.y_pos += self.speed; }
    }
}

pub fn spawn_enemies(how_many: u32, player_x: f32, player_y: f32) -> Vec<Enemy> {
    const OFFSET: f32 = 200.0;
    let mut rng = rand::thread_rng();
    let mut temp_vec = Vec::new();
    let size = 40.0;

    for _ in 0..how_many {
        let temp = Enemy {
            x_pos: not_close_to_player(OFFSET, player_x, window::WIDTH, size),
            y_pos: not_close_to_player(OFFSET, player_y, window::HEIGHT, size),
            width: size,
            height: size,
            speed: rng.gen_range(0.0, 3.0) + 1.0,
            is_alive: true
        };

        temp_vec.push(temp);
    };

    temp_vec
}

pub fn draw(ctx: &mut Context, enemy_vec: &Vec<Enemy>) {
	for bullet in enemy_vec {
		let rect = graphics::Rect::new(bullet.x_pos, bullet.y_pos, bullet.width, bullet.height);
		let rect_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::Color::new(1.0, 0.0, 0.0, 1.0)).unwrap();
		graphics::draw(ctx, &rect_mesh, graphics::DrawParam::default()).unwrap();
	};
}

pub fn remove_the_dead(enemies_vec: &mut Vec<Enemy>) {
	enemies_vec.retain(|b| b.is_alive);
}

fn not_close_to_player(offset: f32, player_coord: f32, window_dim: f32, box_dim: f32) -> f32 {
    let mut rng = rand::thread_rng();
    let mut coord: f32 = rng.gen_range(0.0, window_dim - box_dim);

    while (coord - player_coord).abs() < offset {
        coord = rng.gen_range(0.0, window_dim - box_dim);
    };

    coord
}