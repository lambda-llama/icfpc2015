extern crate hex2d;
extern crate rustc_serialize;

mod formats;
mod game;
mod scoring;
mod strategy;

use rustc_serialize::json;
use std::io::Read;
use std::fs;

fn fetch_game(i: u64) -> formats::Board {
    let path = format!("./problems/problem_{}.json", i);
    println!("{}", path);
    let mut data = String::new();
    fs::File::open(path).unwrap().read_to_string(&mut data).unwrap();
    json::decode(&data).unwrap()
}

fn main() {
    let games = fetch_game(0).games();
    let problem = games.into_iter().next().unwrap().board;
    println!("{}", json::encode(&problem).unwrap());
}
