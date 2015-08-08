use std::collections::{VecDeque, HashSet, HashMap};

use hex2d::Coordinate;

use game::{Command, Unit, ALL_COMMANDS};
use board::{Board, offset_to_cube};

/// Find a sequence of commands which transform `source` to `target`.
pub fn route(source: &Unit, target: &Unit, board: &Board) -> Vec<Command> {
    let mut q = VecDeque::new();
    q.push_back(source.clone());
    let mut parents: HashMap<Unit, (Command, Unit)> = HashMap::new();
    let mut seen: HashSet<Unit> = HashSet::new();
    seen.insert(source.clone());
    while let Some(tip) = q.pop_front() {
        assert!(board.check_unit_position(&tip));
        assert!(seen.contains(&tip));
        if tip == *target {
            break;
        }

        for cj in ALL_COMMANDS.iter() {
            let next = tip.apply(cj);
            if !seen.contains(&next) && board.check_unit_position(&next) {
                q.push_back(next.clone());
                parents.insert(next.clone(), (*cj, tip.clone()));
                seen.insert(next);
            }
        }
    }

    let mut path = Vec::new();
    let mut tip = target;
    while tip != source && parents.contains_key(&tip) {
        let (c, ref next) = parents[tip];
        path.push(c);
        tip = next;
    }
    path.reverse();
    path
}

pub fn best_position(unit: &Unit, board: &Board) -> Option<Unit> {
    for y in (0..board.height).rev() {
        for x in 0..board.width {
            let c = offset_to_cube(&(x as i32, y as i32));
            let moved = unit.move_to(c);
            if board.check_unit_position(&moved) {
                return Some(moved)
            }
        }
    }
    None
}
