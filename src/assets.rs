use ggez::{Context};
use ggez::graphics::{Image};

pub struct Assets {
	// background
	pub backgrounds: Vec<Image>,

	pub head: Image,
	pub enemy: Image,
	pub bullet: Image,
	// doors
	pub left_door_open: Image,
	pub left_door_closed: Image,
	pub right_door_open: Image,
	pub right_door_closed: Image,
	pub top_door_open: Image,
	pub top_door_closed: Image,
	pub bot_door_open: Image,
    pub bot_door_closed: Image,
}

pub fn add_assets(ctx: &mut Context) -> Assets {
	let backgrounds = vec!(
		// Image::new(ctx, "/backgrounds/bg_init.png").unwrap(),
		Image::new(ctx, "/backgrounds/bg1.png").unwrap(),
		Image::new(ctx, "/backgrounds/bg2.png").unwrap(),
		Image::new(ctx, "/backgrounds/bg3.png").unwrap(),
		Image::new(ctx, "/backgrounds/bg4.png").unwrap(),
	);

	Assets {
		backgrounds,
		head: Image::new(ctx, "/head.png").unwrap(),
		bullet: Image::new(ctx, "/bullet.png").unwrap(),
		enemy: Image::new(ctx, "/enemy.png").unwrap(),
		// doors
		left_door_open: Image::new(ctx, "/doors/left_open.png").unwrap(),
		left_door_closed: Image::new(ctx, "/doors/left_closed.png").unwrap(),
		top_door_open: Image::new(ctx, "/doors/top_open.png").unwrap(),
		top_door_closed: Image::new(ctx, "/doors/top_closed.png").unwrap(),
		right_door_open: Image::new(ctx, "/doors/right_open.png").unwrap(),
		right_door_closed: Image::new(ctx, "/doors/right_closed.png").unwrap(),
		bot_door_open: Image::new(ctx, "/doors/bot_open.png").unwrap(),
		bot_door_closed: Image::new(ctx, "/doors/bot_closed.png").unwrap(),
	}
}
