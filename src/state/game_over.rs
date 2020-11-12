use ggez::{Context, graphics, input};

use crate::mygame::{MyGame, GameState};
use cgmath::{Point2};

use std::fs::File;
use std::io::prelude::*;

// these coord are taken from photoshop
const X_START_MENU: f32 = 447.0;
const Y_START_MENU: f32 = 297.0;
const W_START_MENU: f32 = 190.0;
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
        
        // user clicked "TRY AGAIN"
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

    draw_score(ctx, mygame);
}

fn draw_score(ctx: &mut Context, mygame: &mut MyGame) {
    let mut file = File::open("src/scores").expect("Can't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let scores: Vec<&str> = contents.split(',').collect();

    let font = mygame.assets.font;
    let mut text = graphics::Text::new(format!("{}", scores[scores.len() - 1]));

    let dst: Point2<f32> = Point2::new(540.0, 205.0);
    let param = graphics::DrawParam::new()
        .dest(dst)
        .color(graphics::Color::new(71.0/255.0, 46.0/255.0, 40.0/255.0, 1.0));

    graphics::draw(ctx, text.set_font(font, graphics::Scale{x: 23.0, y: 23.0}), param).unwrap();
}
