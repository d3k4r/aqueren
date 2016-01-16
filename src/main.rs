extern crate hyper;

use hyper::server::{Server, Request, Response};

fn hello(req: Request, res: Response) {
    res.send(b"{greeting: \"Hey, what up.\"}").unwrap();
}

fn main() {
    println!("Starting server on localhost:3001");
	Server::http("localhost:3001").unwrap().handle(hello).unwrap();
}
