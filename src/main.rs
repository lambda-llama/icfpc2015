extern crate hex2d;
extern crate rustc_serialize;
extern crate rand;

mod formats;
mod board;
mod game;
mod scoring;
mod strategy;

use hex2d::{Angle, Direction};

use rustc_serialize::json;
use std::io::Read;
use std::fs;

fn fetch_game(i: u64) -> formats::Board {
    let path = format!("./problems/problem_{}.json", i);
    let mut data = String::new();
    fs::File::open(path).unwrap().read_to_string(&mut data).unwrap();
    json::decode(&data).unwrap()
}

fn main() {
    let board = fetch_game(0);
    let game = board.games().into_iter().next().unwrap();
    let positions: Vec<_> = strategy::process_game(&game).iter().map(|c| c.to_state()).collect();
    println!("{}", json::encode(&positions).unwrap());
}
