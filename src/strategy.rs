use std::collections::{VecDeque, HashSet, HashMap};

use hex2d::{Coordinate, Direction, Angle};

use game::{Command, Unit, ALL_COMMANDS};
use game::{Game, GamePosition};
use board::{Board, offset_to_cube};

/// Find a sequence of commands which transform `source` to `target`.
pub fn route(source: &Unit, target: &Unit,
             board: &Board) -> Option<Vec<Command>> {
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

    if !parents.contains_key(target) {
        return None;  // no path found.
    }

    let mut path = Vec::new();
    let mut tip = target;
    while tip != source {
        assert!(parents.contains_key(&tip));
        let (c, ref next) = parents[tip];
        path.push(c);
        tip = next;
    }
    path.reverse();
    path.push(Command::Move(Direction::ZY));  // lock.
    Some(path)
}

pub fn best_position(unit: &Unit, board: &Board) -> Vec<Unit> {
    let mut result = Vec::new();
    let rots = [
        Command::Rotate(Angle::Left),
        Command::Rotate(Angle::Right)];
    for y in (0..board.height).rev() {
        for x in 0..board.width {
            let c = offset_to_cube(&(x as i32, y as i32));
            for rot in rots.iter() {
                let moved = unit.move_corner_to(c).apply(&rot);
                if board.check_unit_position(&moved) {
                    let score = scoring_function(&board.lock_unit(&moved).0);
                    result.push((moved, score));
                }
            }
        }
    }
    result.sort_by(|&(_, s1), &(_, s2)| s2.cmp(&s1));
    result.into_iter().map(|(u, _)| u).collect()
}

pub fn scoring_function(board: &Board) -> i64 {
    let row_cost = 100;
    let hole_penalty = 0;
    return (board.n_full_rows() * row_cost + board.n_clear_top_rows()) as i64 +
        (board.n_holes() as i64) * hole_penalty;
}

pub fn process_game(g: &Game) -> (Vec<Command>, Vec<GamePosition>) {
    let mut cur_game_pos = GamePosition::start(g);
    let mut commands: Vec<Command> = Vec::new();
    let mut positions: Vec<GamePosition> = vec![cur_game_pos.clone()];
    'outer: loop {
        if !cur_game_pos.board.check_unit_position(&cur_game_pos.unit) {
            break;
        }
        let best_positions = best_position(&cur_game_pos.unit, &cur_game_pos.board);
        for target in best_positions {
            if let Some(new_commands) = route(&cur_game_pos.unit, &target, &cur_game_pos.board) {
                for &cmd in new_commands.iter() {
                    cur_game_pos = cur_game_pos.step(cmd);
                    positions.push(cur_game_pos.clone())
                }
                commands.extend(new_commands);
                break;
            }
        }
    }

    return (commands, positions)
}
