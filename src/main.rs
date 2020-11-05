// ggez docs: https://docs.rs/ggez/0.5.1/ggez/
// Help from: https://github.com/AndrewJakubowicz/ggezFlappyCrabby

use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::graphics::{Image};

use ggez::timer;

mod mygame;
mod window;
mod player;
mod bullet;
mod enemy;
mod room;
mod map;
mod assets;

mod game;

use mygame::{MyGame, Game_state};
use assets::{Assets};

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
		// Load/create resources here: images, fonts, sounds, etc.
		let assets = Assets {
			basements: Image::new(ctx, "/bg.png").unwrap(),
			left_door_open: Image::new(ctx, "/left_door_open.png").unwrap(),
			left_door_closed: Image::new(ctx, "/left_door_closed.png").unwrap(),
			top_door_open: Image::new(ctx, "/top_door_open.png").unwrap(),
			top_door_closed: Image::new(ctx, "/top_door_closed.png").unwrap(),
			right_door_open: Image::new(ctx, "/right_door_open.png").unwrap(),
			right_door_closed: Image::new(ctx, "/right_door_closed.png").unwrap(),
			bot_door_open: Image::new(ctx, "/bot_door_open.png").unwrap(),
			bot_door_closed: Image::new(ctx, "/bot_door_closed.png").unwrap(),
			head: Image::new(ctx, "/head.png").unwrap(),
			bullet: Image::new(ctx, "/bullet.png").unwrap(),
			enemy: Image::new(ctx, "/enemy.png").unwrap(),
		};

		// Rooms
		let mut rooms: Vec<Vec<room::Room>> = Vec::new();
		for row in 0..13 {
			let mut row_vec: Vec<room::Room> = Vec::new();
			for col in 0..13 {
				row_vec.push(room::Room::new((row, col)));
			}

			rooms.push(row_vec);
		}
		rooms[map::PLAYER_SPAWN.0][map::PLAYER_SPAWN.1] = room::Room::init();


		let player1 = player::Player {
			x_pos: window::WIDTH / 2.0,
			y_pos: window::HEIGHT / 2.0,
			width: 60.0,
			height: 60.0,
			speed: 7.0,
			reload_time: player::RELOAD_TIME,
			current_room: map::PLAYER_SPAWN,
			life: 6
		};

        MyGame {
			game_state: Game_state::GAME,
			player1,
			bullets: Vec::new(),
			enemies: Vec::new(),
			rooms,
			map: map::generate_map(),
			assets,
			score: 0
		}
	}
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
		if self.game_state == Game_state::GAME {
			game::update(ctx, self);
		}

		Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		graphics::clear(ctx, graphics::BLACK);

		if self.game_state == Game_state::GAME {
			game::draw(ctx, self);
		} else if self.game_state
		 == Game_state::GAMEOVER {
			graphics::clear(ctx, graphics::BLACK);
		 }
		
        graphics::present(ctx)
	}
}

fn main() {
	let (mut ctx, mut event_loop) = window::build_window();
	let mut my_game = MyGame::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}