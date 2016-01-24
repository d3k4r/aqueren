extern crate rand;

const ROWS: i8 = 9;
const COLS: i8 = 12;
const TILES: i8 = 108;
const PLAYERS: i8 = 4;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Game {
    players: Vec<Player>,
    board: Board,
    turn: PlayerId,
    merge_decision: Option<PlayerId>
}

#[derive(RustcDecodable, RustcEncodable)]
struct Player {
    id: PlayerId,
    money: i32,
    shares: PlayerShares,
    tiles: Vec<Tile>
}

type PlayerId = i8;

#[derive(RustcDecodable, RustcEncodable)]
struct PlayerShares {
    luxor: i8,
    tower: i8,
    american: i8,
    festival: i8,
    worldwide: i8,
    continental: i8,
    imperial: i8
}

#[derive(RustcDecodable, RustcEncodable, Clone, Debug)]
struct Tile { row: i8, col: i8 }

#[derive(RustcDecodable, RustcEncodable)]
struct Board {
    slots: Vec<Slot>
}

#[derive(RustcDecodable, RustcEncodable)]
struct Slot {
    row: i8,
    col: i8,
    has_tile: bool,
    hotel: Option<Hotel>
}

enum Action {
    PlaceTile,
    HandleMergeStocks,
    BuyStocks,
    DrawTile,
    EndGame
}

pub struct PlaceTile {
    player: PlayerId,
    tile: Tile
}

pub struct HandleMergeStocks {
    hold: i8,
    sell: i8,
    trade: i8
}

type BuyStocks = PlayerShares;

#[derive(RustcDecodable, RustcEncodable)]
enum Hotel { Luxor, Tower, American, Festival, Worldwide, Continental, Imperial }

fn empty_shares() -> PlayerShares {
    PlayerShares { luxor: 0, tower: 0, american: 0, festival: 0, worldwide: 0, continental: 0, imperial: 0 }
}

fn new_player(id: PlayerId, tiles: Vec<Tile>) -> Player {
    Player { id: id, money: 6000, shares: empty_shares(), tiles: tiles }
}

fn choose_tiles(tiles: Vec<Tile>, count: usize) -> (Vec<Tile>, Vec<Tile>) {
    let mut remaining_tiles = tiles;
    let mut random_tiles = Vec::new();
    for _ in 0..count {
        let random_index = (rand::random::<i8>() as usize) % remaining_tiles.len();
        random_tiles.push(remaining_tiles.remove(random_index));
    }
    (random_tiles, remaining_tiles)
}

pub fn initial_state() -> Game {
    let mut slots: Vec<Slot> = (0..COLS)
        .flat_map(|col| {
            let rows = 0..ROWS;
            let res: Vec<Slot> = rows.map(|row| Slot { row: row, col: col, hotel: None, has_tile: false } ).collect();
            res
        }).collect();
    let tiles = (0..TILES).map(|i| Tile { row: i / COLS, col: i % ROWS }).collect();
    let (starting_tiles, remaining_tiles) = choose_tiles(tiles, PLAYERS as usize);
    let (tiles1, remaining_tiles1) = choose_tiles(remaining_tiles, 6 as usize);
    let (tiles2, remaining_tiles2) = choose_tiles(remaining_tiles1, 6 as usize);
    let (tiles3, remaining_tiles3) = choose_tiles(remaining_tiles2, 6 as usize);
    let (tiles4, _) = choose_tiles(remaining_tiles3, 6 as usize);
    let players = vec![
        new_player(1, tiles1), 
        new_player(2, tiles2),
        new_player(3, tiles3),
        new_player(4, tiles4)];
    for tile in starting_tiles {
        let found_slot = slots.iter_mut().find(|slot| slot.row == tile.row && slot.col == tile.col);
        match found_slot {
            Some(slot) => {
                slot.has_tile = true;
            }
            None => panic!("No slot found!")
        }
    }
    Game {
        board: Board { slots: slots },
        players: players, 
        turn: 1, 
        merge_decision: None
    }
}

#[test]
fn players_start_with_six_tiles() {
    let state = initial_state();
    for player in state.players {
        assert_eq!(player.tiles.len(), 6);
    }
}

#[test]
fn players_start_with_6000_in_cash() {
    let state = initial_state();
    for player in state.players {
        assert_eq!(player.money, 6000);
    }
}

#[test]
fn players_start_with_zero_shares() {
    let state = initial_state();
    for player in state.players {
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
    let state = initial_state();
    let board_tiles = state.board.slots.iter().filter(|s| s.has_tile).count();
    assert_eq!(board_tiles, 4)
}
