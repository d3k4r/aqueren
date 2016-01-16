extern crate hyper;
extern crate rustc_serialize;

use rustc_serialize::json;

mod state;

use hyper::server::{Server, Request, Response};

fn hello(req: Request, res: Response) {
	let game = state::initialState();
	let encoded = json::encode(&game).unwrap();

	res.send(encoded.as_bytes()).unwrap();
}

fn main() {
    println!("Starting server on localhost:3001");
	Server::http("localhost:3001").unwrap().handle(hello).unwrap();
}
