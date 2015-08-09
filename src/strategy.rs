use std::collections::{VecDeque, HashMap};
use std::io::{self, Write};

use hex2d::Angle;

use game::{Command, Unit, ALL_COMMANDS};
use game::{Game, GamePosition};
use board::{Board, offset_to_cube};

/// Find a sequence of commands which transform `source` to `target`.
pub fn route(source: &Unit, target: &Unit,
             board: &Board) -> Option<Vec<Command>> {
    let mut q = VecDeque::new();
    q.push_back(source.clone());
    let mut parents: HashMap<Unit, (Command, Unit)> = HashMap::new();
    // XXX we use parent links instead of a separate hash set
    // for visited nodes.
    parents.insert(source.clone(), (ALL_COMMANDS[0], source.clone()));
    while let Some(tip) = q.pop_front() {
        assert!(board.check_unit_position(&tip));
        assert!(parents.contains_key(&tip));
        if tip == *target {
            break;
        }

        for cj in ALL_COMMANDS.iter() {
            let next = tip.apply(cj);
            if !parents.contains_key(&next) && board.check_unit_position(&next) {
                q.push_back(next.clone());
                parents.insert(next, (*cj, tip.clone()));
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

    for c in ALL_COMMANDS.iter() {
        let locked = target.apply(c);
        if !board.check_unit_position(&locked) {
            path.push(*c);
            return Some(path)
        }
    }

    panic!("sai wat?");
}

pub fn best_position<'a>(unit: &Unit<'a>, board: &Board) -> Vec<Unit<'a>> {
    let mut result = Vec::new();
    let rots = [
        Command::Rotate(Angle::Left),
        Command::Rotate(Angle::Right)];
    for y in (-3..(board.height + 3) as i32).rev() {
        for x in (-3..(board.width + 3) as i32) {
            let c = offset_to_cube(&(x, y));
            let mut candidates = Vec::new();
            let moved = unit.move_corner_to(c);
            candidates.push(moved.clone());
            for rot in rots.iter() {
                candidates.push(moved.apply(&rot));
                candidates.push(moved.apply(&rot).apply(&rot));
            }
            for moved in candidates {
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
    let penalty: Vec<_> = (0..board.height as i64).rev().map(|i| -i).collect();
    let full_row_cost = 10000;
    // let hole_penalty = 0;
    return (board.n_full_rows() * full_row_cost) as i64 + board.total_sum(&penalty);
        // + (board.n_holes() as i64) * hole_penalty;
}

pub fn play<'a>(g: &'a Game) -> (Vec<Command>, Vec<GamePosition<'a>>) {
    let mut cur_game_pos = GamePosition::start(g);
    let mut commands: Vec<Command> = Vec::new();
    let mut positions: Vec<GamePosition> = vec![cur_game_pos.clone()];
    let mut i = 0;
    'outer: while cur_game_pos.board.check_unit_position(&cur_game_pos.unit) {
        i += 1;
        let mut stderr = io::stderr();
        writeln!(&mut stderr, "{} out of {}", i, g.source.len()).unwrap();
        let best_positions = best_position(&cur_game_pos.unit, &cur_game_pos.board);
        let mut moved = false;
        for target in best_positions {
            if let Some(new_commands) = route(&cur_game_pos.unit, &target,
                                              &cur_game_pos.board) {
                for &cmd in new_commands.iter() {
                    if let Some(new_pos) = cur_game_pos.step(cmd) {
                        cur_game_pos = new_pos;
                        positions.push(cur_game_pos.clone())
                    } else {
                        break 'outer;
                    }
                }
                commands.extend(new_commands);
                moved = true;
                break;
            }
        }
        assert!(moved);
    }

    return (commands, positions)
}
