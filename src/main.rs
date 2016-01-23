extern crate hyper;
extern crate rustc_serialize;


mod state;

use std::io::Read;
use rustc_serialize::json;
use hyper::{Get, Post};
use hyper::header::ContentLength;
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

fn handle_request(mut req: Request, mut res: Response) {
    let mut body: String  = "".to_string();
    req.read_to_string(&mut body);

    let path = match req.uri {
        AbsolutePath(ref path) => {
            path
        },
        _ => panic!("Unsupported URI format."),
    };

    match path.as_ref() {
        "/" => {
            match req.method {
                Get => {
                    try_return!(res.send(b"Hey, what up."));
                },
                _ => {
                    *res.status_mut() = hyper::BadRequest;
                    (*res.headers_mut()).set(ContentLength(0));
                }
            }
        },
        "/state" => {
            match req.method {
                Get => {
                    let game = state::initial_state();
                    let encoded = json::encode(&game).unwrap();
                    try_return!(res.send(encoded.as_bytes()));
                },
                _ => {
                    *res.status_mut() = hyper::BadRequest;
                    (*res.headers_mut()).set(ContentLength(0));
                }
            }
        },
        "/turn" => {
            match req.method {
                Post => {
                    try_return!(res.send(body.as_bytes()));
                },
                _ => {
                    *res.status_mut() = hyper::BadRequest;
                    (*res.headers_mut()).set(ContentLength(0));
                }
            }
        },
        _ => {
            *res.status_mut() = hyper::NotFound;
            (*res.headers_mut()).set(ContentLength(0));
        }
    }
}

fn main() {
    let server = Server::http("localhost:3001").unwrap();
    let _guard = server.handle(handle_request);
    println!("Starting server on localhost:3001");
}
