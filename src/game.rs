use hex2d::{self, Angle, Coordinate, Position};

struct Board {
    width: usize,
    height: usize,
    cells: Vec<Vec<bool>>
}

#[derive(Clone)]
struct UnitShape {
    cells: Vec<hex2d::Coordinate>,
    pivot: hex2d::Coordinate
}

#[derive(Clone)]
struct Unit {
    shape: UnitShape,
    position: hex2d::Position, 
}
