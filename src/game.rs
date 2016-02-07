extern crate rand;

use types::*;

pub fn new_game() -> Game {
    let all_tiles = (0..TILES).map(|i| {
        let row = i / COLS;
        let col = i % ROWS;
        if let Some(tile) = Tile::new(row, col) {
            tile
        } else {
            panic!("Attempted to create invalid tile ({},{})", row, col)
        }
        }).collect();
    let (starting_tiles, remaining_tiles) = choose_tiles(all_tiles, PLAYERS);
    let players = new_players(remaining_tiles);
    let slots = initial_slots(starting_tiles);
    Game {
        board: Board { slots: slots },
        players: players, 
        turn: 1, 
        merge_decision: None
    }
}

fn choose_tiles(tiles: Vec<Tile>, count: u8) -> (Vec<Tile>, Vec<Tile>) {
    let mut remaining_tiles = tiles;
    let mut random_tiles = Vec::new();
    for _ in 0..count {
        let random_index = rand::random::<usize>() % remaining_tiles.len();
        random_tiles.push(remaining_tiles.remove(random_index));
    }
    (random_tiles, remaining_tiles)
}

fn new_players(tiles: Vec<Tile>) -> Vec<Player>{
    let init_players: Vec<Player> = Vec::new();
    let (players, _) = (0..PLAYERS)
        .fold( (init_players, tiles), | (mut v, remaining), i | {
            let (player_tiles, new_remaining) = choose_tiles(remaining, 6);
            let player = new_player(i, player_tiles);
            v.push(player);
            (v, new_remaining)
        });
    players
}

pub fn new_player(id: PlayerId, tiles: Vec<Tile>) -> Player {
    Player { id: id, money: 6000, shares: empty_shares(), tiles: tiles }
}

fn empty_shares() -> PlayerShares {
    PlayerShares { luxor: 0, tower: 0, american: 0, festival: 0, worldwide: 0, continental: 0, imperial: 0 }
}

pub fn initial_slots(starting_tiles: Vec<Tile>) -> Vec<Slot> {
    (0..ROWS).flat_map(|row| -> Vec<Slot> {
        (0..COLS).map(|col| {
            Slot { 
                row: row, 
                col: col, 
                hotel: None, 
                has_tile: has_tile_on_slot(&starting_tiles, row, col)
            } 
        }).collect()
    }).collect()
}

fn has_tile_on_slot(tiles: & Vec<Tile>, row: u8, col: u8) -> bool {
    tiles.iter().any(|t| t.row() == row && t.col() == col)
}

pub fn play_turn(game: &Game, action: Action) -> Game {
    match action {
        Action::PlaceTile { player: player, tile: tile } => {
            place_tile(game, player, &tile)
        }
        _ => panic!(format!("I don't know how to play a turn with action {:?}", action))
    }
}

fn place_tile(game: &Game, player: u8, tile: &Tile) -> Game {
    Game {
        board: place_tile_on_board(&game.board, &tile),
        players: game.players.clone(),
        turn: game.turn,
        merge_decision: game.merge_decision
    }
}

fn place_tile_on_board(board: &Board, tile: &Tile) -> Board {
    let slots = board.slots
        .iter()
        .map(|s| {
            if s.row == tile.row() && s.col == tile.col() {
                Slot { row: s.row, col: s.col, has_tile: true, hotel: s.hotel.clone() }
            } else {
                s.clone()
            }
        })
        .collect();
    Board { slots: slots }
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::*;

    type BoardTiles = [[i32; COLS as usize]; ROWS as usize];
    type PlayerTiles = [[(i32,i32); 6]; PLAYERS as usize];

    #[test]
    fn players_start_with_six_tiles() {
        let game = new_game();
        for player in game.players {
            assert_eq!(player.tiles.len(), 6);
        }
    }

    #[test]
    fn players_start_with_6000_in_cash() {
        let game = new_game();
        for player in game.players {
            assert_eq!(player.money, 6000);
        }
    }

    #[test]
    fn players_start_with_zero_shares() {
        let game = new_game();
        for player in game.players {
            assert_eq!(player.shares.luxor, 0);
            assert_eq!(player.shares.tower, 0);
            assert_eq!(player.shares.american, 0);
            assert_eq!(player.shares.festival, 0);
            assert_eq!(player.shares.worldwide, 0);
            assert_eq!(player.shares.continental, 0);
            assert_eq!(player.shares.imperial, 0);
        }
    }

    #[test]
    fn game_starts_with_four_placed_tiles() {
        let game = new_game();
        let board_tiles = game.board.slots.iter().filter(|s| s.has_tile).count();
        assert_eq!(board_tiles, 4)
    }

    #[test]
    fn placing_a_tile_adds_tile_to_board() {
        let start_tiles = [[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]];
        let end_tiles =   [[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]];
        let player_tiles = [[ (0,0), (0,1), (0,2), (0,3), (0,4), (0,5) ],
                            [ (1,0), (1,1), (1,2), (1,3), (1,4), (1,5) ],
                            [ (2,2), (2,1), (2,2), (2,3), (2,4), (2,5) ],
                            [ (3,3), (3,1), (3,2), (3,3), (3,4), (3,5) ]];
        let game = new_game_with_tiles(start_tiles, player_tiles);
        let tile_to_place = Tile::new(5,11).unwrap();
        let action = Action::PlaceTile { player: 1, tile: tile_to_place };
        let game_after = play_turn(&game, action);
        assert_boards_equal(&tiles_to_board(&end_tiles), &game_after.board);
    }

    fn new_game_with_tiles(start_tiles: BoardTiles, player_tiles: PlayerTiles) -> Game {
        let (starting_tiles, remaining) = board_tiles_to_tiles(&start_tiles);
        let players = player_tiles
            .iter()
            .enumerate()
            .map(|(i, tiles)| {
                let _tiles = tiles
                    .iter()
                    .map(|&(r,c)| Tile::new(r as u8, c as u8).unwrap() )
                    .collect();
                new_player(i as u8, _tiles)
            })
            .collect();
        let slots = initial_slots(starting_tiles);
        Game {
            board: Board { slots: slots },
            players: players, 
            turn: 1, 
            merge_decision: None
        }
    }

    fn board_tiles_to_tiles(tiles: &BoardTiles) -> (Vec<Tile>, Vec<Tile>) {
        let mut chosen = Vec::new();
        let mut others = Vec::new();
        for row in 0..tiles.len() {
            for col in 0..tiles[0].len() {
                let tile = Tile::new(row as u8, col as u8).unwrap();
                if tiles[row][col] == 1 {
                    chosen.push(tile)
                } else {
                    others.push(tile)
                }
            }
        }
        (chosen, others)
    }

    fn assert_boards_equal(expected: &Board, actual: &Board) {
        let are_equal = expected.slots
            .iter()
            .zip(actual.slots.iter())
            .all(|(left, right)| *left == *right );
        let error_msg = format!("\n\nBoard did not have expected tiles.\n\nExpected tiles:\n{}\n\nActual tiles:\n{}\n\n", print_board(&expected), print_board(&actual));
        assert!(are_equal, error_msg);
    }

    fn tiles_to_board(tiles: &BoardTiles) -> Board {
        let board_tiles = tiles
            .iter()
            .enumerate()
            .flat_map(|(row, row_tiles)| {
                let slots: Vec<Slot> = row_tiles
                    .iter()
                    .enumerate()
                    .map(|(col, val): (usize, &i32)| {
                        let has_tile = *val == 1;
                        Slot { row: row as u8, col: col as u8, has_tile: has_tile, hotel: None }
                    })
                    .collect();
                slots
            }).collect();
        Board { slots: board_tiles }
    }

    fn row_to_char<'a>(row: u8) -> &'a str {
        let mapping = ["A", "B", "C", "D", "E", "F", "G", "H", "I"];
        mapping[row as usize]
    }

    fn print_tile(slot: &Slot) -> String{
        format!("{}{}", row_to_char(slot.row), slot.col + 1)
    }

    fn print_board(board: &Board) -> String {
        let mut string = String::new();
        string.push_str("   1  2  3  4  5  6  7  8  9  10 11 12\n");
        for row in board.slots.chunks(COLS as usize) {
            let row_char = row_to_char(row[0].row);
            string.push_str(&format!("{}  ", row_char));
            for slot in row {
                string.push_str(&format!("{:<2} ", if slot.has_tile { '\u{25FC}' } else { '\u{25FB}' }));
            }
            string.push_str(&format!("{}\n", row_char));
        }
        string.push_str("   1  2  3  4  5  6  7  8  9  10 11 12");
        string
    }
}
