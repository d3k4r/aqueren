extern crate rand;

const ROWS: u8 = 9;
const COLS: u8 = 12;
const TILES: u8 = 108;
const PLAYERS: u8 = 4;

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

type PlayerId = u8;

#[derive(RustcDecodable, RustcEncodable)]
struct PlayerShares {
    luxor: u8,
    tower: u8,
    american: u8,
    festival: u8,
    worldwide: u8,
    continental: u8,
    imperial: u8
}

#[derive(RustcDecodable, RustcEncodable, Clone, Debug)]
struct Tile { row: u8, col: u8 }

#[derive(RustcDecodable, RustcEncodable)]
struct Board {
    slots: Vec<Slot>
}

#[derive(RustcDecodable, RustcEncodable)]
struct Slot {
    row: u8,
    col: u8,
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
    hold: u8,
    sell: u8,
    trade: u8
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

fn has_tile_on_slot(tiles: & Vec<Tile>, row: u8, col: u8) -> bool {
    tiles.iter().any(|t| t.row == row && t.col == col)
}

fn initial_slots(starting_tiles: Vec<Tile>) -> Vec<Slot> {
    (0..COLS).flat_map(|col| -> Vec<Slot> {
        (0..ROWS).map(|row| {
            Slot { 
                row: row, 
                col: col, 
                hotel: None, 
                has_tile: has_tile_on_slot(&starting_tiles, row, col)
            } 
        }).collect()
    }).collect()
}

pub fn initial_state() -> Game {
    let all_tiles = (0..TILES).map(|i| Tile { row: i / COLS, col: i % ROWS }).collect();
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
