use rand::Rng;
use crate::map;

pub struct Room {
    pub id: (usize, usize),
    pub player_is_here: bool,
    pub doors: Vec<bool>, // [left, top, right, bot]
    pub neighbors: Vec<usize>,
    pub num_of_enemies: u32,
    pub is_finished: bool,
}

impl Room {
    pub fn init() -> Room {
        Room {
            id: map::PLAYER_SPAWN,
            player_is_here: true,
            doors: Vec::new(),
            neighbors: Vec::new(),
            is_finished: true, // so doors to new rooms open.
            num_of_enemies: 0,
        }
    }

    pub fn new(id: (usize, usize)) -> Room {
        let mut rng = rand::thread_rng();
        Room {
            id,
            player_is_here: false,
            doors: Vec::new(),
            neighbors: Vec::new(),
            is_finished: false,
            num_of_enemies: rng.gen_range(1, 10),
        }
    }
}