use std::collections::VecDeque;

use hex2d::Coordinate;

use game::{Board, Command, Unit};


fn place(unit: &Unit, board: &Board) -> Option<Coordinate> {
    unimplemented!()
}

fn route(unit: &Unit, c: &Coordinate) -> Vec<Command> {
    unimplemented!()
}

fn best_position(unit: &Unit, board: &Board) -> Option<Unit> {
    for y in 0..board.height {
        for x in 0..board.width {
            let moved_unit = unimplemented!();
            if board.check_unit_position(&moved_unit) {
                return Some(moved_unit)
            }
        }
    }
    None
}
