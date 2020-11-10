use ggez::{Context, graphics, input};

use crate::{mygame, player, bullet, enemy, room, window};
use mygame::{MyGame, GameState};
use cgmath::{Point2};

// these coord are taken from photoshop
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
	
	// let scale = {
	// 	let wh = 30.0;
	// 	let scale_f = mygame.player1.width / wh;
	// 	[scale_f, scale_f]
	// };

	let param = graphics::DrawParam::new().dest(dst);
	graphics::draw(ctx, &mygame.assets.game_menu, param).unwrap();
    
    // let rect = graphics::Rect::new(X, Y, W, H);
    // let rect_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::WHITE).unwrap();
    // graphics::draw(ctx, &rect_mesh, graphics::DrawParam::default()).unwrap();
}


// fn draw_scores(ctx: &mut Context) {
//     let fps_display = graphics::Text::new(format!("PLAY GAME!"));

//     graphics::draw(
//         ctx,
//         &fps_display,
//         (Point2::new(X + W / 2.0, Y + H / 2.0), graphics::WHITE),
//     ).unwrap();
// }
