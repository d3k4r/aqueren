extern crate hyper;
extern crate rustc_serialize;

mod state;

use rustc_serialize::json;
use hyper::Get;
use hyper::server::{Server, Request, Response};
use hyper::uri::RequestUri::AbsolutePath;

macro_rules! try_return(
    ($e:expr) => {{
        match $e {
            Ok(v) => v,
            Err(e) => { println!("Error: {}", e); return; }
        }
    }}
    );

fn handle_request(req: Request, mut res: Response) {
    match req.uri {
        AbsolutePath(ref path) => match (&req.method, &path[..]) {
            (&Get, "/") => {
                try_return!(res.send(b"Hey, what up."));
                return;
            },
            (&Get, "/state") => {
                let game = state::initialState();
                let encoded = json::encode(&game).unwrap();
                try_return!(res.send(encoded.as_bytes()));
                return;
            }
            _ => {
                *res.status_mut() = hyper::NotFound;
                return;
            }
        },
        _ => {
            return;
        }
    };
}

fn main() {
    let server = Server::http("localhost:3001").unwrap();
    let _guard = server.handle(handle_request);
    println!("Starting server on localhost:3001");
}
