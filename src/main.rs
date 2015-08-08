extern crate hex2d;
extern crate rustc_serialize;
extern crate rand;
extern crate getopts;

mod formats;
mod board;
mod game;
mod scoring;
mod strategy;

use getopts::Options;
use std::io::Read;
use std::fs;
use std::env;
use rustc_serialize::json;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.reqopt("f", "", "File containing JSON encoded input", "FILENAME");
    opts.optopt("t", "", "Time limit, in seconds, to produce output", "NUMBER");
    opts.optopt("m", "", "Memory limit, in megabytes, to produce output", "NUMBER");
    opts.optopt("c", "", "Number of processor cores available", "NUMBER");
    opts.optmulti("p", "", "Phrase of power", "STRING");
    opts.optflag("d", "", "Toggle debug mode");
    opts.optflag("h", "help", "Print help");
    let matches = match opts.parse(&args[1..]) {
        Ok(m)  => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let path = matches.opt_str("f").unwrap();
    let mut data = String::new();
    fs::File::open(path).unwrap().read_to_string(&mut data).unwrap();
    let board: formats::Board = json::decode(&data).unwrap();
    if matches.opt_present("d") {
        let game = board.games().into_iter().next().unwrap();
        let (_, positions) = strategy::process_game(&game);
        let positions: Vec<_> = positions.iter().map(|c| c.to_state()).collect();
        println!("{}", json::encode(&positions).unwrap());
    } else {
        let mut solutions = Vec::new();
        for game in board.games() {
            let (commands, positions) = strategy::process_game(&game);
            for (i, p) in positions.iter().enumerate() {
                println!("turn: {} score: {}, sum_size: {}", i, p.score, p.sum_unit_size);
            }
            solutions.push(formats::Solution {
                problemId: board.id,
                seed: game.seed,
                tag: "CW/CCW".to_string(),
                solution: encoder::encode(commands)
            });
        }
        println!("{}", json::encode(&solutions).unwrap());
    }
}
