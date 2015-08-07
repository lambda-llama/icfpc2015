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

fn extract(x: u64) -> usize {
    ((x % (1 << 31)) >> 16)  as usize
}

pub fn get_source_seq(length: usize, seed: u64) -> Vec<usize> {
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

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Unit {
    pub members: Vec<Cell>,
    pub pivot: Cell
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Cell {
    pub x: usize,
    pub y: usize
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
#[allow(non_snake_case)]
pub struct Solution {
    problemId: u64,
    seed: u64,
    tag: String,
    solution: Vec<Command>
}

type Command = char;
