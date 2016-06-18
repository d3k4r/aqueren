pub const ROWS: u8 = 9;
pub const COLS: u8 = 12;
pub const TILES: u8 = 108;
pub const PLAYERS: u8 = 4;

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct Game {
    pub players: Vec<Player>,
    pub board: Board,
    pub turn: PlayerId,
    pub turn_state: TurnState
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct Player {
    pub id: PlayerId,
    pub money: i32,
    pub shares: PlayerShares,
    pub tiles: Vec<Tile>
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone, PartialEq)]
pub enum PlayerId { One, Two, Three, Four }

impl PlayerId {
    pub fn new(id: u8) -> Option<PlayerId>{
        match id {
            1 => { Some(PlayerId::One) }
            2 => { Some(PlayerId::Two) }
            3 => { Some(PlayerId::Three) }
            4 => { Some(PlayerId::Four) }
            _ => { None }
        }
    }
}

#[derive(RustcDecodable, RustcEncodable, Clone, Debug, PartialEq)]
pub struct PlayerShares {
    pub luxor: u8,
    pub tower: u8,
    pub american: u8,
    pub festival: u8,
    pub worldwide: u8,
    pub continental: u8,
    pub imperial: u8
}

#[derive(RustcDecodable, RustcEncodable, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tile { pub row: u8, pub col: u8 }

impl Tile {
    pub fn new(row: u8, col: u8) -> Option<Tile> {
        if row >= ROWS || col >= COLS {
            None
        } else {
            Some(Tile {row: row, col: col})
        }
    }
    pub fn row(&self) -> u8 {
        self.row
    }
    pub fn col(&self) -> u8 {
        self.col
    }
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct Board {
    pub slots: Vec<Slot>
}

#[derive(RustcDecodable, RustcEncodable, Clone, PartialEq)]
pub struct Slot {
    pub row: u8,
    pub col: u8,
    pub has_tile: bool,
    pub hotel: Option<Hotel>
}

#[derive(RustcDecodable, RustcEncodable, Clone, Debug, PartialEq)]
pub enum Hotel { Luxor, Tower, American, Festival, Worldwide, Continental, Imperial }

#[derive(RustcDecodable, RustcEncodable, Clone, Debug, PartialEq)]
pub enum TurnState {
    Placing,
    BuyingOrDrawing,
    Drawing,
    CreatingChain,
    Merging
}

#[derive(Debug)]
pub enum Action {
    PlaceTile { player: PlayerId, tile: Tile },
    HandleMergeStocks { hold: u8, sell: u8, trade: u8 },
    BuyStocks { player: PlayerId, hotel1: Option<Hotel>, hotel2: Option<Hotel>, hotel3: Option<Hotel> },
    DrawTile,
    EndGame
}

pub enum TurnResult {
    Success(Game),
    Error(String)
}
