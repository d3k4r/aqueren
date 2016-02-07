extern crate hyper;
extern crate rustc_serialize;

mod game;

use std::io::Read;
use rustc_serialize::json;
use hyper::{Get, Post};
use hyper::header::ContentLength;
use hyper::server::{Handler, Server, Request, Response};
use hyper::uri::RequestUri::AbsolutePath;
use game::{Action, Game};

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
    starting_board: Game
}

impl Handler for GameHandler {
    fn handle(&self, mut req: Request, mut res: Response) {
        let actions = &self.actions;
        let starting_board = &self.starting_board;
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
                let state = game::compute_state(starting_board, actions);
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
    let starting_board = game::new_game();
    let server = Server::http("localhost:3001").unwrap();

    println!("Starting server on localhost:3001");

    let _ = server.handle(GameHandler { actions: actions, starting_board: starting_board} );
}
