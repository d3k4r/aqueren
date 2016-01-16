extern crate hyper;

use hyper::server::{Server, Request, Response};

fn hello(req: Request, res: Response) {
    println!("Hey, what up.");
}

fn main() {
	Server::http("localhost:3001").unwrap().handle(hello).unwrap();
}
