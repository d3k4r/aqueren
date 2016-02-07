extern crate hyper;
extern crate rustc_serialize;

mod game;
mod types;

use types::{Action, Game};

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

impl Handler for GameHandler {
    fn handle(&self, mut req: Request, mut res: Response) {
        let actions = &self.actions;
        let initial_game = &self.initial_game;
        let mut body: String  = "".to_string();
        req.read_to_string(&mut body);

        let path = match req.uri {
            AbsolutePath(ref path) => {
                path
            },
            _ => panic!("Unsupported URI format."),
        };

        match (req.method, path.as_ref()) {
            (Get, "/state") => {
                let state = game::compute_state(initial_game, actions);
                let encoded = json::encode(&state).unwrap();
                try_return!(res.send(encoded.as_bytes()));
            },
            (Post, "/turn") => {
                try_return!(res.send(body.as_bytes()));
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
