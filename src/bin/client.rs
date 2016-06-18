extern crate aqueren;
extern crate hyper;
extern crate rustc_serialize;

use aqueren::server::{PlaceTileCmd};
use aqueren::types::{Board, COLS, Game, Player, PlayerShares, Slot, Tile};
use hyper::client::Client;
use hyper::client::response::Response;
use rustc_serialize::json;
use rustc_serialize::json::DecoderError;
use std::collections::HashMap;
use std::env;
use std::iter::FromIterator;
use std::io;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Write;

fn dump_state(server_url: &str) {
    match get_state(server_url) {
        Ok(game) => {
            println!("{}", print_game(&game));
        },
        Err(e) => println!("{}", e)
    }
}

fn place_tile(server_url: &str, tile: Tile) {
    let action = PlaceTileCmd { tile: tile };
    let encodedAction = json::encode(&action).unwrap();
    let url = format!("{}/action", server_url);
    let client = Client::new();
    let result: Result<Response, hyper::error::Error> = client
        .post(&url)
        .body(encodedAction.as_bytes())
        .send();
    let response = result
        .map_err(|_| "Could not get state, is the server running and do you have the correct server URL?".to_string())
        .and_then(parse_body)
        .and_then(parse_state);
    match response {
        Ok(game) => println!("{}", print_game(&game)),
        Err(e) => println!("{:?}", e)
    }
}

fn get_state(server_url: &str) -> Result<Game, String> {
    let url = format!("{}/state", server_url);
    let client = Client::new();
    let result: Result<Response, hyper::error::Error> = client.get(&url).send();
    result
        .map_err(|_| "Could not get state, is the server running and do you have the correct server URL?".to_string())
        .and_then(parse_body)
        .and_then(parse_state)
} 

fn parse_body(mut response: Response) -> Result<String, String> {
    let mut buf = String::new();
    match response.read_to_string(&mut buf) {
        Ok(_) => Ok(buf),
        Err(_) => Err("Could not parse body".to_string())
    }
}

fn parse_state(mut state_json: String) -> Result<Game, String> {
    json::decode(&state_json).map_err(|_| "Could not parse state".to_string())
}

fn print_game(game: &Game) -> String {
    let mut string = String::new();
    string.push_str("Game status\n");
    string.push_str("---------------------\n");
    string.push_str("\n");
    string.push_str(&print_players(&game.players));
    string.push_str("\n");
    string.push_str(&print_board(&game.board));
    string.push_str("\n");
    string.push_str(&format!("Turn: Player {:?} ({:?})\n", game.turn, game.turn_state));
    string
}

fn row_to_char<'a>(row: u8) -> &'a str {
    let mapping = ["A", "B", "C", "D", "E", "F", "G", "H", "I"];
    mapping[row as usize]
}

fn char_to_row(row: char) -> Option<u8> {
    let list = [
        ('A', 0), ('a', 0),
        ('B', 1), ('b', 1),
        ('C', 2), ('c', 2),
        ('D', 3), ('d', 3),
        ('E', 4), ('e', 4),
        ('F', 5), ('f', 5),
        ('G', 6), ('g', 6),
        ('H', 7), ('h', 7),
        ('I', 8), ('i', 8)];
    let mut mapping = HashMap::new();
    for &(c, i) in list.iter() {
        mapping.insert(c, i);
    }
    mapping.get(&row).map(|v| v.clone())
}

fn str_to_col(col_str: &str) -> Option<u8> {
    match col_str.parse::<u8>() {
        Ok(i) if i > 0 && i <= COLS => Some(i - 1),
        _ => None
    }
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
    string.push_str("   1  2  3  4  5  6  7  8  9  10 11 12\n");
    string
}

fn print_players(players: &Vec<Player>) -> String {
    let players_str: Vec<String> = players.iter().map(print_player).collect();
    players_str.as_slice().join("\n")
}

fn print_player(player: &Player) -> String {
    let mut string = String::new();
    string.push_str(&format!("Player {:?}: \n", player.id));
    string.push_str(&format!("  Money: {:?}\n", player.money));
    string.push_str(&format!("  Shares: {}\n", print_shares(&player.shares)));
    string.push_str(&format!("  Tiles: {}\n", print_tiles(&player.tiles)));
    string
}

fn print_shares(shares: &PlayerShares) -> String {
    format!("LUX: {}, TOW: {}, AMER: {}, FEST: {}, WW: {}, CONT: {}, IMP: {}", shares.luxor, shares.tower, shares.american, shares.festival, shares.worldwide, shares.continental, shares.imperial)
}

fn print_tiles(tiles: &Vec<Tile>) -> String {
    let mut tiles_str: Vec<String> = tiles.iter().map(print_tile).collect();
    let mut slice = tiles_str.as_mut_slice();
    slice.sort();
    slice.join(", ")
    
}

fn print_tile(slot: &Tile) -> String {
    format!("{}{}", row_to_char(slot.row), slot.col + 1)
}

fn start_repl(server_url: &str) {
    loop {
        print!("$ ");
        io::stdout().flush();
        match read_command() {
            Ok(cmd) => run_command(server_url, cmd),
            Err(e) => println!("{}", e)
        }
    }
}

fn run_command(server_url: &str, command: Cmd) {
    match command {
        Cmd::Dump => dump_state(server_url),
        Cmd::Place { tile } => place_tile(server_url, tile)
    }
}

enum Cmd {
    Dump,
    Place { tile: Tile }
}

fn read_command() -> Result<Cmd, String> {
    match read_input() {
        Ok(string) => parse_command(&string),
        Err(e) => Err(e)
    }
}

fn parse_command(string: &str) -> Result<Cmd, String> {
    let mut parts = string.split(" ");
    match parts.nth(0) {
        Some("dump") => Ok(Cmd::Dump),
        Some("place") => parse_place(string),
        _ => Err(format!("'{}' is not a command, try 'dump'", string))
    }
}

fn parse_place(string: &str) -> Result<Cmd, String> {
    let example = "Usage example: place B1";
    let mut parts = string.split(" ");
    match parts.nth(1) {
        Some(tile_str) => {
            match parse_tile(tile_str) {
                Some(tile) => Ok(Cmd::Place { tile: tile }),
                None => Err(format!("Couldn't parse tile '{}'\n{}", tile_str, example))
            }
        }
        None => Err(format!("Did you forget a tile?\n{}", example))
    }
}

fn parse_tile(string: &str) -> Option<Tile> {
    let mut chars = string.chars();
    match (chars.nth(0), &string[1..string.len()]) {
        (Some(row_char), col_str) => {
            match (char_to_row(row_char), str_to_col(col_str)) {
                (Some(row), Some(col)) => Some(Tile { row: row, col: col }),
                _ => None
            }
        },
        _ => None
    }
}

fn read_input() -> Result<String, String> {
    let mut input = String::new();
    let read = io::stdin().read_line(&mut input);
    match read {
        Ok(_) => input.trim().parse::<String>().map_err(|e| e.to_string()),
        Err(e) => Err(e.to_string())
    }
}

fn main() {
    let server_url = env::args().nth(1).unwrap_or("http://localhost:3001".to_string());
    println!("Starting client, connecting to {}", server_url);
    match get_state(&server_url) {
        Ok(game) => {
            println!("\n{}", print_game(&game));
        },
        Err(e) => panic!("{}", e)
    }
    start_repl(&server_url);
}
