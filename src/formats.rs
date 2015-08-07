use hex2d;

use game;
use board;


#[derive(Debug, RustcDecodable, RustcEncodable)]
#[allow(non_snake_case)]
pub struct Board {
    pub id: u64,
    pub units: Vec<Unit>,
    pub width: usize,
    pub height: usize,
    pub filled: Vec<Cell>,
    pub sourceLength: usize,
    pub sourceSeeds: Vec<u64>
}

impl Board {
    pub fn games(&self) -> Vec<game::Game> {
        self.sourceSeeds.iter().map(|&s| self.game_for_seed(s)).collect()
    }

    pub fn game_for_seed(&self, seed: u64) -> game::Game {
        let mut cells = vec![vec![false; self.width]; self.height];
        for c in self.filled.iter() {
            cells[c.y as usize][c.x as usize] = true;
        }

        let board = board::Board {
            width: self.width,
            height: self.height,
            cells: cells
        };

        let units: Vec<game::Unit> = self.units.iter().map(|u| {
            game::Unit {
                cells: u.members.iter()
                    .map(|&c| hex2d::Coordinate::from(c))
                    .collect(),
                pivot: hex2d::Coordinate::from(u.pivot)
            }
        }).collect();

        let source = get_source_seq(self.sourceLength, seed).iter()
            .map(|&i| units[i % units.len()].clone())
            .collect();

        game::Game {
            board: board,
            source: source
        }
    }

}


#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Unit {
    pub members: Vec<Cell>,
    pub pivot: Cell
}

#[derive(Debug, RustcDecodable, RustcEncodable, Clone, Copy)]
pub struct Cell {
    pub x: i32,
    pub y: i32
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
#[allow(non_snake_case)]
pub struct Solution {
    problemId: u64,
    seed: u64,
    tag: String,
    solution: String,
}

impl From<Cell> for hex2d::Coordinate {
    fn from(c: Cell) -> hex2d::Coordinate {
        hex2d::Coordinate {x: c.x, y: c.y}
    }
}

pub fn get_source_seq(length: usize, seed: u64) -> Vec<usize> {
    fn extract(x: u64) -> usize {
        let m = 1 << 31;
        return ((x % m) >> 16)  as usize;
    }

    let modulus: u64 = 1 << 32;
    let multiplier: u64 = 1103515245;
    let increment: u64 = 12345;
    let mut result = Vec::new();
    let mut c = seed;
    for _ in 0..length {
        result.push(extract(c));
        c = (c * multiplier + increment) % modulus;
    }
    result
}
