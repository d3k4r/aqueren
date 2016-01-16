const ROWS: i8 = 12;
const COLS: i8 = 9;
const PLAYERS: i8 = 4;

struct Game {
  players: Vec<Player>,
  board: Board
}

struct Player {
  money: i32
}

struct Board {
  slots: Vec<Vec<Slot>>
}

struct Slot {
  hotel: Option<Hotel>
}

enum Hotel { Luxor, Tower, American, Festival, Worldwide, Continental, Imperial }

fn initialState() -> Game {
  let slots = (0..COLS)
    .map(|x| (0..ROWS).map(|x| Slot { hotel: None } ).collect() )
    .collect();
  let players = (0..PLAYERS).map(|x| Player { money: 0 } ).collect();
  Game { board: Board { slots: slots }, players: players }
}
