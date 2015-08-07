use std::ops::Add;

use hex2d::{self, Angle, Coordinate, Direction, Position, ToCoordinate};

struct Board {
    width: usize,
    height: usize,
    cells: Vec<Vec<bool>>
}

struct Game {
    inial_board: Board,
    source: Vec<Unit>
}

struct GamePosition {
    game: Game,
    unit: Unit
}

impl GamePosition {
    fn start(mut g: Game) -> GamePosition {
        let unit = g.source.remove(0);
        GamePosition {
            game: g,
            unit: unit
        }
    }

    fn step(&self, c: Command) -> GamePosition {
        unimplemented!();
    }
}

#[derive(Clone)]
struct Unit {
    cells: Vec<hex2d::Coordinate>,
    pivot: hex2d::Coordinate
}

enum Command {
    Move(hex2d::Direction),
    Rotate(hex2d::Angle)
}

impl Unit {
    fn apply(&self, c: &Command) -> Unit {
        match c {
            &Command::Move(d)   => {
                assert!(d == Direction::YX ||  // West
                        d == Direction::XY ||  // East
                        d == Direction::ZX ||  // SW
                        d == Direction::ZY);   // SE
                let cells = self.cells.iter().map(|&c| c + d).collect();
                let pivot = self.pivot + d;
                Unit { cells: cells, pivot: pivot }
            },
            &Command::Rotate(a) => {
                // Read as clockwise and counterclockwise.
                assert!(a == Angle::Right || a == Angle::Left);
                let cells = self.cells.iter()
                    .map(|c| c.rotate_around(self.pivot, a)).collect();
                Unit { cells: cells, ..*self }
            }
        }
    }
}
