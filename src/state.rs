const ROWS: i8 = 12;
const COLS: i8 = 9;
const PLAYERS: i8 = 4;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Game {
  players: Vec<Player>,
  board: Board
}

#[derive(RustcDecodable, RustcEncodable)]
struct Player {
  money: i32
}

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

pub fn initialState() -> Game {
  let slots = (0..COLS)
    .map(|x| (0..ROWS).map(|x| Slot { hotel: None } ).collect() )
    .collect();
  let players = (0..PLAYERS).map(|x| Player { money: 0 } ).collect();
  Game { board: Board { slots: slots }, players: players }
}
