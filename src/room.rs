use rand::Rng;
use crate::map;

pub struct Room {
    pub room_number: (usize, usize),
    pub num_of_enemies: u32,
    pub is_finished: bool,
}

impl Room {
    pub fn init() -> Room {
        Room {
            room_number: map::PLAYER_SPAWN,
            is_finished: true, // so doors to new rooms open.
            num_of_enemies: 0,
        }
    }

    pub fn new(room_number: (usize, usize)) -> Room {
        let mut rng = rand::thread_rng();
        Room {
            room_number,
            is_finished: false,
            num_of_enemies: rng.gen_range(1, 10),
        }
    }
}