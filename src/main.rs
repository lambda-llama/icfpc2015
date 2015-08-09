extern crate hex2d;
extern crate rustc_serialize;
extern crate rand;
extern crate getopts;
extern crate simple_parallel;

mod formats;
mod board;
mod game;
mod scoring;
mod strategy;
mod encoder;

use getopts::Options;
use std::io::Read;
use std::fs;
use std::env;
use rustc_serialize::json;

// fn dirty_play<'a>(g: &'a game::Game, cmds: &Vec<game::Command>) -> Vec<game::GamePosition<'a>> {
//     let mut cur_game_pos = game::GamePosition::start(g);
//     let mut positions: Vec<game::GamePosition> = vec![cur_game_pos.clone()];
//     for &cmd in cmds.iter() {
//         if let Some(new_pos) =  cur_game_pos.step(cmd) {
//             cur_game_pos = new_pos;
//             positions.push(cur_game_pos.clone())
//         } else {
//             break
//         }
//     }
//     positions
// }

// let letters = "iiiiiiiimmiiiiiimimmiiiimimimmimimimimmimimimeemimeeeemimimimimiiiiiimmeemimimimimiimimimmeemimimimmeeeemimimimmiiiiiipmiimimimeeemmimimmemimimimiiiiiimeeemimimimimeeemimimimmiiiimemimimmiiiipimeeemimimmiiiippmeeeeemimimimiiiimmimimeemimimeeeemimimiiiipmeeemmimmiimimmmimimeemimimimmeeemimiiiiipmiiiimmeeemimimiiiipmmiipmmimmiippimemimeeeemimmiipppmeeeeemimimmiimipmeeeemimimiimmeeeeemimmeemimmeeeemimiiippmiippmiiimmiimimmmmmeeeemimmiippimmimimeemimimimmeemimimimmeemimimimiimimimeeemmimimmmiiiiipimeemimimimmiiiimimmiiiiiiiimiimimimimeeemmimimimmiiiiiimimmemimimimimmimimimeemimiiiiiiiimiiiimimimiimimimmimmimimimimmeeeemimimimimmmimimimimeemimimimimmmemimimmiiiiiiimiimimimmiiiiiimeeeeemimimimimmimimimmmmemimimmeeeemimimimmiimimimmiiiiiipmeeeeemimimimimmiiiiimmemimimimimmmmimimmeeeemimimimimeeemimimimmiimimimeeemmimimmiiiiiiimimiiiiiimimmiiiiiiiimmimimimimiiiimimimeemimimimimmeeemimimimimiiiiiiimiiiimimmemimimimmeemimimimeeemmimimmiiiiiimmiiiipmmiiimmmimimeemimimeeemmimmiiiippmiiiimiiippimiimimeemimimeeeemimimiiiipmeemimimiimiimimmimeeemimimmippipmmiimemimmipimeeeemimmeemimiippimeeeeemimimmmimmmeeeemimimiiipimmiipmemimmeeeemimimiipipimmipppimeeemimmpppmmpmeeeeemimmemm";
//     let cmds:Vec<_> = letters.chars() .map(encoder::symbols2cmd).collect();



fn main() {
    let args: Vec<String> = env::args().collect();

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
    let power_phrases = matches.opt_strs("p");
    let mut data = String::new();
    fs::File::open(path).unwrap().read_to_string(&mut data).unwrap();
    let board: formats::Board = json::decode(&data).unwrap();
    if matches.opt_present("d") {
        let game = board.games().into_iter().next().unwrap();
        let (_, positions) = strategy::play(&game);
        // let positions = dirty_play(&game, &cmds);
        let positions: Vec<_> = positions.iter().map(|c| c.to_state())
            .collect();
        println!("{}", json::encode(&positions).unwrap());
    } else {
        let mut solutions = Vec::new();
        for game in board.games() {
            let (commands, _positions) = strategy::play(&game);
            // for (i, p) in positions.iter().enumerate() {
            //     println!("turn: {} score: {}, sum_size: {}", i, p.score, p.sum_unit_size);
            // }
            solutions.push(formats::Solution {
                problemId: board.id,
                seed: game.seed,
                tag: "CW/CCW".to_string(),
                solution: encoder::encode(&commands, &power_phrases)
            });
        }
        println!("{}", json::encode(&solutions).unwrap());
    }
}
