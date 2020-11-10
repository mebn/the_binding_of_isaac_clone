use ggez::{Context, graphics, input};

use crate::{mygame, player, bullet, enemy, room, window, map};
use mygame::{MyGame, GameState};
use cgmath::{Point2};

// these coord are taken from photoshop
const X_START_MENU: f32 = 446.0;
const Y_START_MENU: f32 = 264.0;
const W_START_MENU: f32 = 191.0;
const H_START_MENU: f32 = 35.0;

const X_TRY: f32 = 450.0;
const Y_TRY: f32 = 343.0;
const W_TRY: f32 = 170.0;
const H_TRY: f32 = 36.0;


pub fn update(ctx: &mut Context, mygame: &mut MyGame) {
    if input::mouse::button_pressed(ctx, input::mouse::MouseButton::Left) {
        let point = input::mouse::position(ctx);

        // user clicked "START MENU"
        if point.x > X_START_MENU && point.x < X_START_MENU + W_START_MENU &&
           point.y > Y_START_MENU && point.y < Y_START_MENU + H_START_MENU {
            mygame.game_state = GameState::GAMEMENU;
        }
        
        // user clicked TRY AGAIN
        if point.x > X_TRY && point.x < X_TRY + W_TRY &&
           point.y > Y_TRY && point.y < Y_TRY + H_TRY {
            mygame.game_state = GameState::GAME;
        }

    }
}

pub fn draw(ctx: &mut Context, mygame: &mut MyGame) {
    graphics::clear(ctx, graphics::BLACK);

    let dst: Point2<f32> = Point2::new(0.0, 0.0);
	let param = graphics::DrawParam::new().dest(dst);
    graphics::draw(ctx, &mygame.assets.game_over, param).unwrap();
    
    let rect = graphics::Rect::new(X_START_MENU, Y_START_MENU, W_START_MENU, H_START_MENU);
    let rect_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::WHITE).unwrap();
    graphics::draw(ctx, &rect_mesh, graphics::DrawParam::default()).unwrap();
}

