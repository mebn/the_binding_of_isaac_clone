use ggez::{Context, graphics, input};

use crate::mygame::{MyGame, GameState};
use cgmath::{Point2};

use std::fs::File;
use std::io::prelude::*;

// these coord are taken from photoshop
// TODO: make these coords responsive. Only works with 1080x720 now.
const X: f32 = 463.0;
const Y: f32 = 417.0;
const W: f32 = 155.0;
const H: f32 = 60.0;


pub fn update(ctx: &mut Context, mygame: &mut MyGame) {
    if input::mouse::button_pressed(ctx, input::mouse::MouseButton::Left) {
        let point = input::mouse::position(ctx);

        if point.x > X && point.x < X + W && point.y > Y && point.y < Y + H {
            println!("{:?}", point);
            mygame.game_state = GameState::GAME;
        }

    }
}

pub fn draw(ctx: &mut Context, mygame: &mut MyGame) {
    graphics::clear(ctx, graphics::BLACK);

    let dst: Point2<f32> = Point2::new(0.0, 0.0);
	let param = graphics::DrawParam::new().dest(dst);
    graphics::draw(ctx, &mygame.assets.game_menu, param).unwrap();
    
    draw_high_scores(ctx, mygame);
}

fn draw_high_scores(ctx: &mut Context, mygame: &mut MyGame) {
    // Gets the high scores from scores file and handles it.
    let mut file = File::open("src/scores").expect("Can't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let scores: Vec<&str> = contents.split(',').collect();
    let mut scores: Vec<i32> = scores.into_iter().map(|x| x.trim().parse().unwrap()).collect();
    scores.sort_by(|a, b| b.cmp(a));

    let mut text = {
        let mut text = String::new();
        let how_many = if scores.len() < 5 {scores.len()} else {5}; // Only display top 5.
        for i in 0..how_many {
            text = format!("{}{}\n", text, scores[i]); // appends scores[i] to text
        }
        graphics::Text::new(text)
    };

    // Handle high scores text
    let font = mygame.assets.font;
    let dst: Point2<f32> = Point2::new(448.0, 72.0);
    let param = graphics::DrawParam::new()
        .dest(dst)
        .color(graphics::Color::new(71.0/255.0, 46.0/255.0, 40.0/255.0, 1.0));

    graphics::draw(ctx, text.set_font(font, graphics::Scale{x: 23.0, y: 23.0}), param).unwrap();
}
