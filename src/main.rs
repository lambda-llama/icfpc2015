extern crate hex2d;
extern crate rustc_serialize;

mod board;
mod game;

use rustc_serialize::json;
use std::io::Read;
use std::fs;

use board::Board;

fn main() {
    let mut problem = String::new();
    fs::File::open("./../problems/problem_0.json").unwrap().read_to_string(&mut problem).unwrap();
    let board: Board = json::decode(&problem).unwrap();
    println!("{:?}", board);
}
