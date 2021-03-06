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
        let board = board::Board::new(self.width, self.height,
                                      self.filled.iter().map(|c| (c.x, c.y)));


        let units: Vec<Vec<hex2d::Coordinate>> = self.units.iter().map(|u| {
            u.members.iter()
                    .map(|&c| hex2d::Coordinate::from(c) - hex2d::Coordinate::from(u.pivot))
                    .collect()
        }).collect();

        let source = get_source_seq(self.sourceLength, seed).iter()
            .map(|&i| units[i % units.len()].clone())
            .collect();

        game::Game {
            board: board,
            source: source,
            seed: seed
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
    pub problemId: u64,
    pub seed: u64,
    pub tag: String,
    pub solution: String,
}

impl From<Cell> for hex2d::Coordinate {
    fn from(c: Cell) -> hex2d::Coordinate {
        board::offset_to_cube(&(c.x, c.y))
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
