extern crate hex2d;
extern crate rustc_serialize;

mod formats;
mod board;
mod game;
// mod scoring;
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
    let game = fetch_game(0).games().into_iter().next().unwrap();
    // let mut moves: Vec<game::Command> = Vec::new();
    // moves.push(game::Command::Move(Direction::ZX));
    let p = strategy::best_position(&game.source[0], &game.board).unwrap();
    println!("source = {:?}", game.source[0]);
    println!("target = {:?}", p);
    let moves = strategy::route(&game.source[0], &p, &game.board);
    println!("{:?}", moves);
    let states: Vec<_> = game.play(&moves).iter().map(|p| p.to_state())
        .collect();
    println!("{}", json::encode(&states).unwrap());
}
