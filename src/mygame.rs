use crate::player::{Player};
use crate::enemy::{Enemy};
use crate::room::{Room};
use crate::bullet::{Bullet};
use crate::assets::{Assets};

#[derive(PartialEq)]
pub enum Game_state {
	GAME,
	GAMEOVER,
	GAMEMENU
}

pub struct MyGame {
    pub game_state: Game_state,
	pub player1: Player,
	pub bullets: Vec<Bullet>,
	pub enemies: Vec<Enemy>,
	pub rooms: Vec<Vec<Room>>,
	pub map: Vec<Vec<bool>>,
    pub assets: Assets,
    pub score: u32
}