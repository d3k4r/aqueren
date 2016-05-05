extern crate rustc_serialize;

mod game;
mod types;

use game::*;
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
    let end_tiles =   [[0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]];
    let player_tiles = [[ (0,0), (0,1), (0,2), (0,3), (0,4), (0,5) ],
    [ (1,0), (1,1), (1,2), (1,3), (1,4), (1,5) ],
    [ (2,2), (2,1), (2,2), (2,3), (2,4), (2,5) ],
    [ (3,3), (3,1), (3,2), (3,3), (3,4), (3,5) ]];
    let game = new_game_with_tiles(start_tiles, player_tiles);
    let tile_to_place = Tile::new(0,2).unwrap();
    let action = Action::PlaceTile { player: PlayerId::One, tile: tile_to_place };
    match play_turn(&game, &action) {
        TurnResult::Success(game_after) => {
            assert_boards_equal(&tiles_to_board(&end_tiles), &game_after.board);
        }
        _ => {
            panic!("Placing a valid tile failed")
        }
    }
}

#[test]
fn placing_a_tile_fails_if_player_does_not_have_tile() {
    let start_tiles = [[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]];
    let player_tiles = [[ (0,0), (0,1), (0,2), (0,3), (0,4), (0,5) ],
    [ (1,0), (1,1), (1,2), (1,3), (1,4), (1,5) ],
    [ (2,2), (2,1), (2,2), (2,3), (2,4), (2,5) ],
    [ (3,3), (3,1), (3,2), (3,3), (3,4), (3,5) ]];
    let game = new_game_with_tiles(start_tiles, player_tiles);
    let tile_to_place = Tile::new(5,11).unwrap();
    let action = Action::PlaceTile { player: PlayerId::One, tile: tile_to_place };
    match play_turn(&game, &action) {
        TurnResult::Success(_) => {
            panic!("Placing a tile succeeded when player did not have tile")
        }
        _ => {}
    }
}

#[test]
fn placing_a_tile_fails_if_player_does_not_have_turn() {
    let start_tiles = [[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]];
    let player_tiles = [[ (0,0), (0,1), (0,2), (0,3), (0,4), (0,5) ],
    [ (1,0), (1,1), (1,2), (1,3), (1,4), (1,5) ],
    [ (2,2), (2,1), (2,2), (2,3), (2,4), (2,5) ],
    [ (3,3), (3,1), (3,2), (3,3), (3,4), (3,5) ]];
    let game = new_game_with_tiles(start_tiles, player_tiles);
    let tile_to_place = Tile::new(1,4).unwrap();
    let action = Action::PlaceTile { player: PlayerId::Two, tile: tile_to_place };
    match play_turn(&game, &action) {
        TurnResult::Success(_) => {
            panic!("Placing a tile succeeded when player did not have turn")
        }
        _ => {}
    }
}

#[test]
fn placing_a_tile_removes_tile_from_player() {
    let start_tiles = [[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]];
    let player_tiles = [[ (0,0), (0,1), (0,2), (0,3), (0,4), (0,5) ],
    [ (1,0), (1,1), (1,2), (1,3), (1,4), (1,5) ],
    [ (2,2), (2,1), (2,2), (2,3), (2,4), (2,5) ],
    [ (3,3), (3,1), (3,2), (3,3), (3,4), (3,5) ]];
    let game = new_game_with_tiles(start_tiles, player_tiles);
    let tile_to_place = Tile::new(0,2).unwrap();
    let action = Action::PlaceTile { player: PlayerId::One, tile: tile_to_place.clone() };
    match play_turn(&game, &action) {
        TurnResult::Success(game_after) => {
            let player = game_after.players.iter().find(|p| p.id == PlayerId::One).unwrap();
            let has_tile = player.tiles.iter().any(|t| *t == tile_to_place);
            assert!(!has_tile, "Placed tile was still on player")
        }
        _ => {
            panic!("Placing a valid tile failed")
        }
    }
}

#[test]
fn buying_stocks_reduces_player_money() {
    let start_tiles = [[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]];
    let player_tiles = [[ (0,0), (0,1), (0,2), (0,3), (0,4), (0,5) ],
    [ (1,0), (1,1), (1,2), (1,3), (1,4), (1,5) ],
    [ (2,2), (2,1), (2,2), (2,3), (2,4), (2,5) ],
    [ (3,3), (3,1), (3,2), (3,3), (3,4), (3,5) ]];
    let game = new_game_with_tiles(start_tiles, player_tiles);
    let action = Action::BuyStocks { player: PlayerId::One, hotel1: Some(Hotel::Luxor), hotel2: None, hotel3: None };
    match play_turn(&game, &action) {
        TurnResult::Success(game_after) => {
            let player = game_after.players.iter().find(|p| p.id == PlayerId::One).unwrap();
            let expected_money = 5800;
            let error_msg = format!("After buying stocks, expected player to have {:?} dollars but player had {:?} dollars", expected_money, player.money);
            assert!(player.money == expected_money, error_msg)
        }
        _ => {
            panic!("Failed buying stocks")
        }
    }
}

#[test]
fn buying_stocks_gives_player_bought_stocks() {
    let start_tiles = [[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]];
    let player_tiles = [[ (0,0), (0,1), (0,2), (0,3), (0,4), (0,5) ],
    [ (1,0), (1,1), (1,2), (1,3), (1,4), (1,5) ],
    [ (2,2), (2,1), (2,2), (2,3), (2,4), (2,5) ],
    [ (3,3), (3,1), (3,2), (3,3), (3,4), (3,5) ]];
    let game = new_game_with_tiles(start_tiles, player_tiles);
    let action = Action::BuyStocks {
        player: PlayerId::One, 
        hotel1: Some(Hotel::Luxor), 
        hotel2: Some(Hotel::Luxor), 
        hotel3: Some(Hotel::Imperial)
    };
    match play_turn(&game, &action) {
        TurnResult::Success(game_after) => {
            let player = game_after.players.iter().find(|p| p.id == PlayerId::One).unwrap();
            let expected_shares = PlayerShares {
                luxor: 2,
                tower: 0,
                american: 0,
                festival: 0,
                worldwide: 0,
                continental: 0,
                imperial: 1
            };
            let error_msg = format!("After buying stocks, expected player to have shares\n{:?} but player had shares\n{:?}", expected_shares, player.shares);
            assert!(player.shares == expected_shares, error_msg)
        }
        _ => {
            panic!("Failed buying stocks")
        }
    }
}

#[test]
fn player_can_draw_tile() {
}

#[test]
fn drawing_tile_ends_players_turn() {
}

fn new_game_with_tiles(start_tiles: BoardTiles, player_tiles: PlayerTiles) -> Game {
    let (starting_tiles, _) = board_tiles_to_tiles(&start_tiles);
    let players = player_tiles
        .iter()
        .enumerate()
        .map(|(i, tiles)| {
            let _tiles = tiles
                .iter()
                .map(|&(r,c)| Tile::new(r as u8, c as u8).unwrap() )
                .collect();
            new_player(PlayerId::new((i+1) as u8).unwrap(), _tiles)
        })
    .collect();
    let slots = initial_slots(starting_tiles);
    Game {
        board: Board { slots: slots },
        players: players, 
        turn: PlayerId::One, 
        turn_state: TurnState::Placing
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
