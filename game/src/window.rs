use ggez::{Context, ContextBuilder};
use ggez::conf;
use ggez:: event;

use std::env;
use std::path;

pub fn build_window() -> (Context, event::EventsLoop) {
    // Handle resource path.
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
	};
    
    // Settings about the window.
	let cb = ContextBuilder::new("game_0", "nilsen")
		.window_setup(conf::WindowSetup::default().title("My Game!"))
		.window_mode(conf::WindowMode::default().dimensions(600.0, 600.0))
        .add_resource_path(resource_dir)
        .build()
        .unwrap();
        
    cb
}