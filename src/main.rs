extern crate hex2d;
extern crate rustc_serialize;

mod formats;
mod board;
mod game;
mod scoring;
mod strategy;

use hex2d::Direction;

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
    let mut states = Vec::new();
    for game in board.games() {
        for unit in game.source.iter() {
            let p = strategy::best_position(&unit, &game.board).unwrap();
            // let moves = strategy::route(&unit, &p, &game.board);
    // Command::Move(Direction::YX),
    // Command::Move(Direction::XY),
    // Command::Move(Direction::XZ),
    // Command::Move(Direction::YZ),
    // Command::Rotate(Angle::Left),
    // Command::Rotate(Angle::Right)
            let moves = vec![game::Command::Move(Direction::XY)];
            if moves.is_empty() {
                break;
            }

            states.extend(game.play(&moves).iter().map(|p| p.to_state()));
            break;
        }
        break;
    }
    println!("{}", json::encode(&states).unwrap());
}
