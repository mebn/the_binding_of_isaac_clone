// ggez docs: https://docs.rs/ggez/0.5.1/ggez/



// PROBLEMS:
// resoureces not working correctly.


use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::input::keyboard;
use ggez::audio::{SoundSource, Source};
use ggez::conf;

use std::env;
use std::path;

struct MyGame {
	x_pos: f32,
	y_pos: f32
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources here: images, fonts, sounds, etc.
        MyGame {
			x_pos: 100.0,
			y_pos: 100.0
		}
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
		// Handle keypresses and movement.
		if keyboard::is_key_pressed(_ctx, event::KeyCode::D) { self.x_pos += 10.0; }
		if keyboard::is_key_pressed(_ctx, event::KeyCode::A) { self.x_pos -= 10.0; }
		if keyboard::is_key_pressed(_ctx, event::KeyCode::W) { self.y_pos -= 10.0; }
		if keyboard::is_key_pressed(_ctx, event::KeyCode::S) { self.y_pos += 10.0; }
		if keyboard::is_key_pressed(_ctx, event::KeyCode::F) {
			// Play sound.
			let mut space_sound: Source = Source::new(_ctx, "/coin.wav")?;
			space_sound.play()?;
		}
		Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
		
		// Draws a rectangle on the screen.
		let rect = graphics::Rect::new(self.x_pos, self.y_pos, 50.0, 50.0);
		let rect_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::WHITE)?;
		graphics::draw(ctx, &rect_mesh, graphics::DrawParam::default())?;

        graphics::present(ctx)
    }
}

fn main() {
	let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
	};
	
	let (mut ctx, mut event_loop) = ContextBuilder::new("game_0", "nilsen")
		.window_setup(conf::WindowSetup::default().title("My Game!"))
		.window_mode(conf::WindowMode::default().dimensions(600.0, 600.0))
		.add_resource_path(resource_dir)
		.build()
		.unwrap();

	let mut my_game = MyGame::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}