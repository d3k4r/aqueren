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
    money: i32,
    shares: PlayerShares
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

fn emptyShares() -> PlayerShares {
    PlayerShares { luxor: 0, tower: 0, american: 0, festival: 0, worldwide: 0, continental: 0, imperial: 0 }
}

fn newPlayer() -> Player {
    Player { money: 0, shares: emptyShares() }
}

pub fn initialState() -> Game {
    let slots = (0..COLS)
        .map(|x| (0..ROWS).map(|x| Slot { hotel: None } ).collect() )
        .collect();
    let players = (0..PLAYERS).map(|x| newPlayer() ).collect();
    Game { board: Board { slots: slots }, players: players }
}
