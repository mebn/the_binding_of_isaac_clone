use rand::Rng;

pub const MAP_SIZE: usize = 13;
pub const PLAYER_SPAWN: (usize, usize) = {
    let coord = (MAP_SIZE - 1) / 2;
    (coord, coord)
};

pub fn generate_map() -> Vec<Vec<bool>> {
    let mut map_vec: Vec<Vec<bool>> = Vec::new();
    let mut rng = rand::thread_rng();

    // creates a MAP_SIZE x MAP_SIZE vector.
    for _ in 0..MAP_SIZE {
        let mut col: Vec<bool> = Vec::new();

        for _ in 0..MAP_SIZE {
            let rand_val = rng.gen_range(0, 4);
            let room_or_not = rand_val != 0;

            col.push(room_or_not);
        }

        map_vec.push(col);
    }

    // make sure spawn room exists.
    map_vec[PLAYER_SPAWN.0][PLAYER_SPAWN.0] = true;

    map_vec
}

// check if it's possible to enter a room next to current room. 
// returns vec with true/false for doors [left, top, right, bottom]
pub fn room_next(current_location: (usize, usize), map: &Vec<Vec<bool>>) -> Vec<bool> {
    let mut possible_doors: Vec<bool> = vec![false; 4]; // [left, top, right, bot]
    let (row, col) = current_location;
    
    // left
    if col == 0 {
        possible_doors[0] = false;
    } else if map[row][col - 1] {
        possible_doors[0] = true;
    } else {
        possible_doors[0] = false;
    }

    // top
    if row == 0 {
        possible_doors[1] = false;
    } else if map[row - 1][col] {
        possible_doors[1] = true;
    } else {
        possible_doors[1] = false;
    }

    // right
    if col == MAP_SIZE - 1 {
        possible_doors[2] = false;
    } else if map[row][col + 1] {
        possible_doors[2] = true;
    } else {
        possible_doors[2] = false;
    }

    // bottom
    if row == MAP_SIZE - 1 {
        possible_doors[3] = false;
    } else if map[row + 1][col] {
        possible_doors[3] = true;
    } else {
        possible_doors[3] = false;
    }

    return possible_doors;
}