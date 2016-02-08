extern crate hyper;
extern crate rustc_serialize;

mod game;
mod types;

use types::{Action, Game, TurnResult};

use std::io::Read;
use rustc_serialize::json;
use hyper::{Get, Post};
use hyper::header::ContentLength;
use hyper::server::{Handler, Request, Response, Server};
use hyper::uri::RequestUri::AbsolutePath;

macro_rules! try_return(
    ($e:expr) => {{
        match $e {
            Ok(v) => v,
            Err(e) => { println!("Error: {}", e); return; }
        }
    }}
    );

struct GameHandler {
    actions: Vec<Action>,
    initial_game: Game
}

#[derive(RustcDecodable, Debug)]
struct ActionTile {
    player: u8,
    tile: types::Tile
}

impl Handler for GameHandler {
    fn handle(&self, mut req: Request, mut res: Response) {
        let actions = &self.actions;
        let initial_game = &self.initial_game;
        let mut body: String  = "".to_string();
        let _ = req.read_to_string(&mut body);

        let path = match req.uri {
            AbsolutePath(ref path) => {
                path
            },
            _ => panic!("Unsupported URI format."),
        };

        match (req.method, path.as_ref()) {
            (Get, "/state") => {
                let result = game::compute_state(initial_game, actions);
                match result {
                    TurnResult::Success(game) => {
                        let encoded = json::encode(&game).unwrap();
                        try_return!(res.send(encoded.as_bytes()));
                    }
                    TurnResult::Error(error) => {
                        try_return!(res.send(error.as_bytes()));
                    }
                }
            },
            (Post, "/action") => {
                let action_tile: ActionTile = json::decode(&body).unwrap();
                let player = types::PlayerId::new(action_tile.player).unwrap();
                let tile = types::Tile::new(action_tile.tile.row,action_tile.tile.col).unwrap();
                let action = Action::PlaceTile { player: player, tile: tile};
                let result = game::compute_state(initial_game, actions);
                match result {
                    TurnResult::Success(game) => {
                        match game::play_turn(&game, &action) {
                            TurnResult::Success(game_after) => {
                                let encoded = json::encode(&game_after).unwrap();
                                try_return!(res.send(encoded.as_bytes()));
                            }
                            _ => {
                                try_return!(res.send(b"{\"error\": \"Invalid tile\"}"));
                            }
                        }
                    }
                    TurnResult::Error(error) => {
                        try_return!(res.send(error.as_bytes()));
                    }
                }
            },
            _ => {
                *res.status_mut() = hyper::BadRequest;
                (*res.headers_mut()).set(ContentLength(0));
            }
        }
    }
}

fn main() {
    let actions = game::new_actions();
    let initial_game = game::new_game();
    let server = Server::http("localhost:3001").unwrap();
    println!("Starting server on localhost:3001");

    let _ = server.handle(GameHandler { actions: actions, initial_game: initial_game} );
}
