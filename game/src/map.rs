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
            let rand_val = rng.gen_range(0, 10);
            let room_or_not = rand_val != 0;

            col.push(room_or_not);
        }

        map_vec.push(col);
    }

    // make sure spawn room exists.
    map_vec[PLAYER_SPAWN.0][PLAYER_SPAWN.0] = true;

    // prints the map
    for row in &map_vec {
        println!("{:?}", row);
    };

    map_vec
}

pub fn room_next(current_location: (usize, usize), map: &Vec<Vec<bool>>) -> Vec<bool> {
    let mut possible_doors: Vec<bool> = vec!(false, false, false, false); // [left, top, right, bot]
    
    // left
    if current_location.1 == 0 {
        possible_doors[0] = false;
    } else if map[current_location.0][current_location.1 - 1] {
        possible_doors[0] = true;
    } else {
        possible_doors[0] = false;
    }

    // top
    if current_location.0 == 0 {
        possible_doors[1] = false;
    } else if map[current_location.0 - 1][current_location.1] {
        possible_doors[1] = true;
    } else {
        possible_doors[1] = false;
    }

    // right
    if current_location.1 == MAP_SIZE - 1 {
        possible_doors[2] = false;
    } else if map[current_location.0][current_location.1 + 1] {
        possible_doors[2] = true;
    } else {
        possible_doors[2] = false;
    }

    // bottom
    if current_location.0 == MAP_SIZE - 1 {
        possible_doors[3] = false;
    } else if map[current_location.0 + 1][current_location.1] {
        possible_doors[3] = true;
    } else {
        possible_doors[3] = false;
    }

    return possible_doors;
}