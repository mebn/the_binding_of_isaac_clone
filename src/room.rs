use rand::Rng;
use crate::map;
use ggez::graphics;
use ggez::{Context};

use crate::mygame::{MyGame};

use cgmath::{Point2};

pub struct Room {
    pub room_number: (usize, usize),
    pub num_of_enemies: u32,
    pub is_finished: bool,
}

impl Room {
    pub fn init() -> Room {
        Room {
            room_number: map::PLAYER_SPAWN,
            is_finished: true, // so doors to new rooms open.
            num_of_enemies: 0,
        }
    }

    pub fn new(room_number: (usize, usize)) -> Room {
        let mut rng = rand::thread_rng();
        Room {
            room_number,
            is_finished: false,
            num_of_enemies: rng.gen_range(1, 10),
        }
    }
}

pub fn draw_background(ctx: &mut Context, mygame: &mut MyGame) {
    let dst: Point2<f32> = Point2::new(0.0, 0.0);
    let param = graphics::DrawParam::new().dest(dst);

    // Draw background.
    graphics::draw(ctx, &mygame.assets.basements, param).unwrap();
}

pub fn draw_doors(ctx: &mut Context, mygame: &mut MyGame) {
    let dst: Point2<f32> = Point2::new(0.0, 0.0);
    let param = graphics::DrawParam::new().dest(dst);

    if mygame.rooms[mygame.player1.current_room.0][mygame.player1.current_room.1].is_finished {
        // left
        if map::room_next(mygame.player1.current_room, &mygame.map)[0] {
            graphics::draw(ctx, &mygame.assets.left_door_open, param).unwrap();
        }
        // top
        if map::room_next(mygame.player1.current_room, &mygame.map)[1] {
            graphics::draw(ctx, &mygame.assets.top_door_open, param).unwrap();
        }
        // right
        if map::room_next(mygame.player1.current_room, &mygame.map)[2] {
            graphics::draw(ctx, &mygame.assets.right_door_open, param).unwrap();
        }
        // bottom
        if map::room_next(mygame.player1.current_room, &mygame.map)[3] {
            graphics::draw(ctx, &mygame.assets.bot_door_open, param).unwrap();
        }
    } else {
        // left
        if map::room_next(mygame.player1.current_room, &mygame.map)[0] {
            graphics::draw(ctx, &mygame.assets.left_door_closed, param).unwrap();
        }
        // top
        if map::room_next(mygame.player1.current_room, &mygame.map)[1] {
            graphics::draw(ctx, &mygame.assets.top_door_closed, param).unwrap();
        }
        // right
        if map::room_next(mygame.player1.current_room, &mygame.map)[2] {
            graphics::draw(ctx, &mygame.assets.right_door_closed, param).unwrap();
        }
        // bottom
        if map::room_next(mygame.player1.current_room, &mygame.map)[3] {
            graphics::draw(ctx, &mygame.assets.bot_door_closed, param).unwrap();
        }
    }
}