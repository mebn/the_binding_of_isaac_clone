use ggez::{Context};
use ggez::graphics;
use rand::Rng;

use crate::window;
use crate::player::{Player};
use crate::mygame::{MyGame};

use cgmath::{Point2};

pub struct Enemy {
	pub x_pos: f32,
    pub y_pos: f32,
    pub width: f32,
    pub height: f32,
    pub speed: f32,
    pub is_alive: bool
}

pub fn update_pos(enemy_vec: &mut Vec<Enemy>, player: &Player) {
    for enemy in enemy_vec {
        if enemy.x_pos >= player.x_pos { enemy.x_pos -= enemy.speed; }
        if enemy.x_pos <= player.x_pos { enemy.x_pos += enemy.speed; }
        if enemy.y_pos >= player.y_pos { enemy.y_pos -= enemy.speed; }
        if enemy.y_pos <= player.y_pos { enemy.y_pos += enemy.speed; }
    }
}

pub fn spawn_enemies(how_many: u32, player_x: f32, player_y: f32) -> Vec<Enemy> {
    const OFFSET: f32 = 200.0;
    let mut rng = rand::thread_rng();
    let mut temp_vec = Vec::new();
    let size = 50.0;

    for _ in 0..how_many {
        let temp = Enemy {
            x_pos: spawn_point(OFFSET, player_x, window::WIDTH, size),
            y_pos: spawn_point(OFFSET, player_y, window::HEIGHT, size),
            width: size,
            height: size,
            speed: rng.gen_range(1.0, 4.0),
            is_alive: true
        };

        temp_vec.push(temp);
    };

    temp_vec
}

pub fn draw(ctx: &mut Context, mygame: &mut MyGame) {
	for enemy in &mygame.enemies {
        let dst: Point2<f32> = Point2::new(enemy.x_pos, enemy.y_pos);
	
        let scale = {
            let wh = 30.0;
            let scale_f = enemy.width / wh;
            [scale_f, scale_f]
        };

        let param = graphics::DrawParam::new().dest(dst).scale(scale);
        graphics::draw(ctx, &mygame.assets.enemy, param).unwrap();
	};
}

pub fn remove_the_dead(enemies_vec: &mut Vec<Enemy>) {
	enemies_vec.retain(|b| b.is_alive);
}

fn spawn_point(offset: f32, player_coord: f32, window_dim: f32, box_dim: f32) -> f32 {
    let mut rng = rand::thread_rng();
    let mut coord: f32 = rng.gen_range(0.0, window_dim - box_dim);

    while (coord - player_coord).abs() < offset {
        coord = rng.gen_range(0.0, window_dim - box_dim);
    };

    coord
}

pub fn enemy_hit_player(enemy_vec: &mut Vec<Enemy>, player: &mut Player) {
    for enemy in enemy_vec {
        // Copied from https://developer.mozilla.org/en-US/docs/Games/Techniques/2D_collision_detection
        if enemy.x_pos < player.x_pos + player.width &&
            enemy.x_pos + enemy.width > player.x_pos &&
            enemy.y_pos < player.y_pos + player.height &&
            enemy.y_pos + enemy.height > player.y_pos {
                player.life -= 1;
                enemy.is_alive = false;
                println!("{}", player.life);
        }
    }
}