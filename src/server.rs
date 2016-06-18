extern crate hyper;
extern crate rustc_serialize;

use game;
use types::{Action, Game, Tile, TurnResult};

use std::sync::Mutex;
use std::io::Read;
use rustc_serialize::json;
use self::hyper::{Get, Post};
use self::hyper::header::ContentLength;
use self::hyper::server::{Handler, Request, Response, Server};
use self::hyper::uri::RequestUri::AbsolutePath;

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct PlaceTileCmd {
    pub tile: Tile
}

macro_rules! try_return(
    ($e:expr) => {{
        match $e {
            Ok(v) => v,
            Err(e) => { println!("Error: {}", e); return; }
        }
    }}
    );

struct GameHandler {
    actions: Mutex<Vec<Action>>,
    initial_game: Game
}

impl Handler for GameHandler {
    fn handle(&self, mut req: Request, mut res: Response) {
        let mut actions = self.actions.lock().unwrap();
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
                println!("/state");
                let result = game::compute_state(initial_game, &actions);
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
                println!("/action");
                let cmd: Result<PlaceTileCmd, json::DecoderError> = json::decode(&body);
                match cmd {
                    Ok(cmd) => {
                        println!("/action got command");
                        let result = game::compute_state(initial_game, &actions);
                        match result {
                            TurnResult::Success(game) => {
                                println!("/action got game");
                                let action = Action::PlaceTile { player: game.turn.clone(), tile: cmd.tile };
                                match game::play_turn(&game, &action) {
                                    TurnResult::Success(game_after) => {
                                        println!("/action ran action");
                                        actions.push(action);
                                        let encoded = json::encode(&game_after).unwrap();
                                        try_return!(res.send(encoded.as_bytes()));
                                    }
                                    TurnResult::Error(e) => {
                                        println!("/action error");
                                        try_return!(res.send(format!("{{\"error\": \"{}\"}}", e).as_bytes()));
                                    }
                                }
                            }
                            TurnResult::Error(error) => {
                                println!("/action some error ");
                                try_return!(res.send(error.as_bytes()));
                            }
                        }
                    },
                    Err(e) => {
                        println!("/action bad action");
                        try_return!(res.send("Bad action!".as_bytes()))
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

pub fn run_server() {
    let actions = Mutex::new(game::new_actions());
    let initial_game = game::new_game();
    let mut handler = GameHandler { actions: actions, initial_game: initial_game};
    let server = Server::http("localhost:3001").unwrap();
    println!("Starting server on localhost:3001");

    let _ = server.handle(handler);
}
