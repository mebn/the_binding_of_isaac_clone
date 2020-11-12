use rand::Rng;
use crate::map;
use ggez::graphics;
use ggez::{Context};

use crate::mygame::MyGame;
use crate::assets::Assets;

use cgmath::{Point2};

pub struct Room {
    pub room_number: (usize, usize),
    pub num_of_enemies: u32,
    pub is_finished: bool,
    pub background_index: usize,
}

impl Room {
    pub fn init() -> Room {
        Room {
            room_number: map::PLAYER_SPAWN,
            is_finished: true, // so doors to new rooms open.
            num_of_enemies: 0,
            background_index: 0
        }
    }

    pub fn new(room_number: (usize, usize), assets: &Assets) -> Room {
        let mut rng = rand::thread_rng();
        let rand_num = rng.gen_range(0, assets.backgrounds.len());

        Room {
            room_number,
            is_finished: false,
            num_of_enemies: rng.gen_range(1, 14),
            background_index: rand_num
        }
    }
}

pub fn draw_background(ctx: &mut Context, mygame: &mut MyGame) {
    let dst: Point2<f32> = Point2::new(0.0, 0.0);
    let param = graphics::DrawParam::new().dest(dst);

    let background = {
        let (row, col) = mygame.player1.current_room;
        let bg_index = mygame.rooms[row][col].background_index;

        &mygame.assets.backgrounds[bg_index]
    };

    // Draw background.
    graphics::draw(ctx, background, param).unwrap();
}

pub fn draw_doors(ctx: &mut Context, mygame: &mut MyGame) {
    let dst: Point2<f32> = Point2::new(0.0, 0.0);
    let param = graphics::DrawParam::new().dest(dst);
    let (row, col) = mygame.player1.current_room;

    let doors_sprite = if mygame.rooms[row][col].is_finished {
        [&mygame.assets.left_door_open, &mygame.assets.top_door_open,
        &mygame.assets.right_door_open, &mygame.assets.bot_door_open]
    } else {
        [&mygame.assets.left_door_closed, &mygame.assets.top_door_closed,
        &mygame.assets.right_door_closed, &mygame.assets.bot_door_closed]
    };

    for i in 0..4 {
        // [left, top, right, bottom]
        if map::room_next(mygame.player1.current_room, &mygame.map)[i] {
            graphics::draw(ctx, doors_sprite[i], param).unwrap();
        }
    }
}