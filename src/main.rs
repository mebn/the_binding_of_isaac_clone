// ggez docs: https://docs.rs/ggez/0.5.1/ggez/
// Help from: https://github.com/AndrewJakubowicz/ggezFlappyCrabby

use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics;

mod mygame;
mod window;
mod player;
mod bullet;
mod enemy;
mod room;
mod map;
mod assets;
mod state;

use mygame::{MyGame, GameState};

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
		let assets = assets::add_assets(ctx);

		// Rooms
		let mut rooms: Vec<Vec<room::Room>> = Vec::new();
		for row in 0..13 {
			let mut row_vec: Vec<room::Room> = Vec::new();
			for col in 0..13 {
				row_vec.push(room::Room::new((row, col), &assets));
			}

			rooms.push(row_vec);
		}
		let (row, col) = map::PLAYER_SPAWN;
		rooms[row][col] = room::Room::init();


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
			game_state: GameState::GAME,
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
		if self.game_state == GameState::GAME {
			state::game::update(ctx, self);
		} else if self.game_state == GameState::GAMEOVER {

		} else if self.game_state == GameState::GAMEMENU {
			state::game_menu::update(ctx, self);
		}

		Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		graphics::clear(ctx, graphics::BLACK);

		if self.game_state == GameState::GAME {
			state::game::draw(ctx, self);
		} else if self.game_state == GameState::GAMEOVER {
			graphics::clear(ctx, graphics::BLACK);
		} else if self.game_state == GameState::GAMEMENU {
			state::game_menu::draw(ctx, self);
		}
		
        graphics::present(ctx)
	}
}


fn main() {
	let (mut ctx, mut event_loop) = window::build_window();
	let mut my_game = MyGame::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}