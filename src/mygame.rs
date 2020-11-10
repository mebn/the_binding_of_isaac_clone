use ggez::{Context};
use crate::player;
use crate::player::{Player};
use crate::enemy::{Enemy};
use crate::room::{Room};
use crate::bullet::{Bullet};
use crate::assets;
use crate::assets::{Assets};
use crate::map;
use crate::window;


#[derive(PartialEq)]
pub enum GameState {
	GAME,
	GAMEOVER,
	GAMEMENU
}

pub struct MyGame {
    pub game_state: GameState,
	pub player1: Player,
	pub bullets: Vec<Bullet>,
	pub enemies: Vec<Enemy>,
	pub rooms: Vec<Vec<Room>>,
	pub map: Vec<Vec<bool>>,
    pub assets: Assets,
    pub score: u32
}

pub fn new_game(ctx: &mut Context, game_state: GameState) -> MyGame {
    let assets = assets::add_assets(ctx);

    // Rooms
    let mut rooms: Vec<Vec<Room>> = Vec::new();
    for row in 0..13 {
        let mut row_vec: Vec<Room> = Vec::new();
        for col in 0..13 {
            row_vec.push(Room::new((row, col), &assets));
        }

        rooms.push(row_vec);
    }
    let (row, col) = map::PLAYER_SPAWN;
    rooms[row][col] = Room::init();


    let player1 = Player {
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
        game_state,
        player1,
        bullets: Vec::new(),
        enemies: Vec::new(),
        rooms,
        map: map::generate_map(),
        assets,
        score: 0
    }
}