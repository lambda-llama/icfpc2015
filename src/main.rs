extern crate rustc_serialize;
mod board;

use rustc_serialize::json;

use board::Board;

fn main() {
    let b = Board {id: 92};
    let encoded = json::encode(&b).unwrap();
    println!("{}", encoded);
}
