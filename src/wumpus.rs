use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum TileStatus {
    Breeze,
    Stench,
    Sparkling,
}

#[derive(Default, Clone, Debug)]
pub struct Tile {
    pub status: HashSet<TileStatus>,
}

#[derive(Clone)]
pub struct GameInfo {
    pub agent_position: (usize, usize),
    pub gold_position: (usize, usize),
    pub wumpus_position: (usize, usize),
    pub pits_position: HashSet<(usize, usize)>,
}

pub fn apply_status(map: &mut [Vec<Tile>], game_info: &GameInfo) {
    /* --------------------------------- Stench --------------------------------- */
    let (wumpus_x, wumpus_y) = game_info.wumpus_position;
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for (dx, dy) in directions.iter() {
        let smell_x = wumpus_x as isize + dx;
        let smell_y = wumpus_y as isize + dy;

        if smell_x >= 0
            && smell_x < map.len() as isize
            && smell_y >= 0
            && smell_y < map[0].len() as isize
        {
            map[smell_x as usize][smell_y as usize]
                .status
                .insert(TileStatus::Stench);
        }
    }

    /* --------------------------------- Breeze --------------------------------- */
    for (pit_x, pit_y) in &game_info.pits_position {
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for (dx, dy) in directions.iter() {
            let wind_x = *pit_x as isize + dx;
            let wind_y = *pit_y as isize + dy;

            if wind_x >= 0
                && wind_x < map.len() as isize
                && wind_y >= 0
                && wind_y < map[0].len() as isize
            {
                map[wind_x as usize][wind_y as usize]
                    .status
                    .insert(TileStatus::Breeze);
            }
        }
    }

    /* -------------------------------- Sparkling ------------------------------- */
    let (gold_x, gold_y) = game_info.gold_position;
    map[gold_x][gold_y].status.insert(TileStatus::Sparkling);
}
