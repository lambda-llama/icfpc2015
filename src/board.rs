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
