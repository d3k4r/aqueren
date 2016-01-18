extern crate rand;

const ROWS: i8 = 9;
const COLS: i8 = 12;
const TILES: i8 = 108;
const PLAYERS: i8 = 4;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Game {
  players: Vec<Player>,
  board: Board
}

#[derive(RustcDecodable, RustcEncodable)]
struct Player {
  money: i32,
  shares: PlayerShares,
  tiles: Vec<Tile>
}

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
  slots: Vec<Vec<Slot>>
}

#[derive(RustcDecodable, RustcEncodable)]
struct Slot {
  hotel: Option<Hotel>
}

#[derive(RustcDecodable, RustcEncodable)]
enum Hotel { Luxor, Tower, American, Festival, Worldwide, Continental, Imperial }

fn empty_shares() -> PlayerShares {
  PlayerShares { luxor: 0, tower: 0, american: 0, festival: 0, worldwide: 0, continental: 0, imperial: 0 }
}

fn new_player(tiles: Vec<Tile>) -> Player {
  Player { money: 6000, shares: empty_shares(), tiles: tiles }
}

fn choose_tiles(tiles: Vec<Tile>, count: usize) -> Vec<Tile> {
  let mut remaining_tiles = tiles.clone();
  let mut random_tiles = Vec::new();
  for _ in 0..count {
    let random_index = (rand::random::<i8>() as usize) % remaining_tiles.len();
    random_tiles.push(remaining_tiles.remove(random_index));
  }
  random_tiles
}

pub fn initial_state() -> Game {
  let slots = (0..COLS)
    .map(|_| (0..ROWS).map(|_| Slot { hotel: None } ).collect() )
    .collect();
  let tiles = (0..TILES).map(|i| Tile { row: i / COLS, col: i % ROWS }).collect();
  let mut chosen_tiles = choose_tiles(tiles, 6*PLAYERS as usize);
  let tiles1 = chosen_tiles.split_off(18);
  let tiles2 = chosen_tiles.split_off(12);
  let tiles3 = chosen_tiles.split_off(6);
  let tiles4 = chosen_tiles;
  let players = vec![
    new_player(tiles1), 
    new_player(tiles2),
    new_player(tiles3),
    new_player(tiles4)];
  Game { board: Board { slots: slots }, players: players }
}
