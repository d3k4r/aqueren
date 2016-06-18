extern crate hyper;
extern crate rustc_serialize;

use game;
use types::{Action, Game, Tile};

use std::sync::Mutex;
use std::io::Read;
use rustc_serialize::json;
use rustc_serialize::Encodable;
use self::hyper::{Get, Post};
use self::hyper::header::ContentLength;
use self::hyper::method::Method;
use self::hyper::server::{Handler, Request, Response, Server};
use self::hyper::uri::RequestUri::AbsolutePath;

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct PlaceTileCmd {
    pub tile: Tile
}

struct GameHandler {
    actions: Mutex<Vec<Action>>,
    initial_game: Game
}

impl Handler for GameHandler {
    fn handle(&self, req: Request, mut res: Response) {
        let mut actions = self.actions.lock().unwrap();
        let game = game::compute_state(&self.initial_game, &actions).unwrap();
        let (method, path, body) = parse_request(req);
        println!("{} {}", method, path);
        match (method, path.as_ref()) {
            (Get, "/state") => send_json(&game, res),
            (Post, "/action") => {
                match handle_action(&game, body) {
                    Ok((game_after, action)) => {
                        actions.push(action);
                        send_json(&game_after, res)
                    },
                    Err(e) => send_error(e, res)
                }
            }
            _ => {
                *res.status_mut() = hyper::BadRequest;
                (*res.headers_mut()).set(ContentLength(0));
            }
        }
    }
}

fn parse_request(mut req: Request) -> (Method, String, String) {
    let mut body: String = "".to_string();
    let _ = req.read_to_string(&mut body);
    let path = match req.uri {
        AbsolutePath(ref p) => { p },
        _ => panic!("Unsupported URI format."),
    };
    (req.method, path.clone(), body)
}

fn send_json<T: Encodable>(object: &T, res: Response) {
    let encoded = json::encode(object).unwrap();
    match res.send(encoded.as_bytes()) {
        Ok(_) => {},
        Err(e) => { println!("Error sending: {}", e) }
    }
}

fn send_error(error_msg: String, res: Response) {
    match res.send(error_msg.as_bytes()) {
        Ok(_) => {},
        Err(e) => { println!("Error sending: {}", e) }
    }
}

fn handle_action(game: &Game, body: String) -> Result<(Game, Action), String> {
    match parse_action(&game, body) {
        Ok(action) => game::play_turn(&game, &action).map(|game_after| (game_after, action)),
        Err(e) => Err(e)
    }
}

fn parse_action(game_before: &Game, json: String) -> Result<Action, String> {
    let cmd: Result<PlaceTileCmd, String> = json::decode(&json).map_err(|e| e.to_string());
    cmd.map(|c| Action::PlaceTile { player: game_before.turn.clone(), tile: c.tile })
}

pub fn run_server() {
    let actions = Mutex::new(game::new_actions());
    let initial_game = game::new_game();
    let handler = GameHandler { actions: actions, initial_game: initial_game};
    let server = Server::http("localhost:3001").unwrap();
    println!("Starting server on localhost:3001");
    let _ = server.handle(handler);
}
