use std::collections::{VecDeque, HashSet, HashMap};

use hex2d::{Coordinate};

use game::{Command, Unit, ALL_COMMANDS};
use board::Board;

fn route(source: &Unit, target: &Unit, board: &Board) -> Vec<Command> {
    let mut q = VecDeque::new();
    q.push_back(source.clone());
    let mut parents: HashMap<Unit, (Command, Unit)> = HashMap::new();
    let mut seen: HashSet<Unit> = HashSet::new();
    while (!q.is_empty()) {
        let tip = q.pop_front().unwrap();
        if tip == *target {
            break;
        }

        for cj in ALL_COMMANDS.iter() {
            let next = tip.apply(cj);
            if !seen.contains(&next) && board.check_unit_position(&next) {
                q.push_back(next.clone());
                parents.insert(next, (*cj, tip.clone()));
            }
        }

        seen.insert(tip);
    }

    let mut path = Vec::new();
    let mut tip = target;
    while tip != source && parents.contains_key(&tip) {
        let (c, ref next) = parents[tip];
        path.push(c);
        tip = next;
    }

    path
}

fn best_position(unit: &Unit, board: &Board) -> Option<Unit> {
    for y in 0..board.height {
        for x in 0..board.width {
            let moved_unit = unit.move_to(Coordinate  {x: x as i32, y: y as i32});
            if board.check_unit_position(&moved_unit) {
                return Some(moved_unit)
            }
        }
    }
    None
}
