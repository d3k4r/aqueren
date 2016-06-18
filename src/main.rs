extern crate hyper;
extern crate rustc_serialize;

mod game;
mod server;
mod types;

use server::{run_server};

fn main() {
    run_server()
}
