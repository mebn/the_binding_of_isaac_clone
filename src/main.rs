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

use mygame::{MyGame, GameState, new_game};

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
		new_game(ctx, GameState::GAMEMENU)
	}
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
		if self.game_state == GameState::GAME {
			state::game::update(ctx, self);
		} else if self.game_state == GameState::GAMEOVER {
			state::game_over::update(ctx, self);
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
			state::game_over::draw(ctx, self);
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