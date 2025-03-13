use wumpus::GameInfo;
use wumpus::Tile;
use wumpus::TileStatus;
use wumpus::apply_status;

mod wumpus;

fn main() {
    /* ---------------------------------- Setup --------------------------------- */
    let mut map: Vec<Vec<Tile>> = vec![vec![Tile::default(); 4]; 4];
    let game_info = GameInfo {
        agent_position: (0, 0),
        wumpus_position: (1, 1),
        gold_position: (1, 2),
        pits_position: [(2, 1), (1, 3)].iter().cloned().collect(),
    };
    apply_status(&mut map, &game_info);

    /* --------------------------------- Display -------------------------------- */
    display_map(&map, &game_info);
}

/* ----------------------------------- CLI ---------------------------------- */

fn display_map(map: &[Vec<Tile>], game_info: &GameInfo) {
    let width = map[0].len();

    let separator = "-".repeat(width * 14).to_string();

    println!("{separator}");

    for (i, row) in map.iter().enumerate() {
        // displays: Breeze, Stench, Sparkling
        let mut first_line = String::new();
        // displays: Wumpus, Pit, Gold
        let mut second_line = String::new();

        for (j, tile) in row.iter().enumerate() {
            let mut status_str = String::new();

            /* --------------------------------- Status --------------------------------- */
            for status in &tile.status {
                match status {
                    TileStatus::Breeze => status_str.push_str("B "),
                    TileStatus::Stench => status_str.push_str("S "),
                    TileStatus::Sparkling => status_str.push_str("Sk "),
                }
            }

            first_line.push_str(&format!("| {:<11} ", status_str));

            /* -------------------------------- Entities -------------------------------- */
            let mut entity_str = String::new();
            if game_info.pits_position.contains(&(i, j)) {
                entity_str.push_str("P ");
            }
            if game_info.wumpus_position == (i, j) {
                entity_str.push_str("W ");
            }
            if game_info.gold_position == (i, j) {
                entity_str.push_str("G ");
            }
            if game_info.agent_position == (i, j) {
                entity_str.push_str("A ");
            }

            second_line.push_str(&format!("| {:<11} ", entity_str));
        }

        first_line.push('|');
        second_line.push('|');

        println!("{}", first_line);
        println!("{}", second_line);

        println!("{separator}");
    }
}
