use std::collections::{VecDeque, BinaryHeap, HashMap, HashSet};
use std::io::{self, Write};
use std::i32;

use hex2d::Angle;

use game::{Command, Unit, ALL_COMMANDS};
use game::{Game, GamePosition};
use board::{Board, offset_to_cube, cube_to_offset};

fn xy(unit: &Unit) -> Vec<(i32, i32)> {
    let mut acc: Vec<(i32, i32)> = unit.iter().collect();
    let pivot = unit.position.coord;
    acc.push(cube_to_offset(&pivot));
    acc.sort();
    acc
}

pub fn route_because_it_works(source: &Unit, target: &Unit,
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

/// Find a sequence of commands which transform `source` to `target`.
pub fn route(source: &Unit, target: &Unit, board: &Board,
             phrases: &Vec<Vec<Command>>) -> Option<Vec<Command>> {
    let max = phrases.iter().map(|v| v.len()).max().unwrap_or(0);
    let penalty = 1000;
    let mut q: BinaryHeap<(i32, Unit)> = BinaryHeap::new();  // max-heap.
    q.push((0, source.clone()));
    let mut parents: HashMap<Unit, (Command, Unit)> = HashMap::new();
    let mut power: HashSet<Unit> = HashSet::new();
    let mut dist: HashMap<Unit, i32> = HashMap::new();
    dist.insert(source.clone(), 0);
    // XXX we use parent links instead of a separate hash set
    // for visited nodes.
    parents.insert(source.clone(), (ALL_COMMANDS[0], source.clone()));
    while let Some((d, tip)) = q.pop() {
        assert!(board.check_unit_position(&tip));
        assert!(parents.contains_key(&tip));
        if tip == *target {
            break
        } else if d > *dist.get(&tip).unwrap_or(&i32::max_value()) ||
            power.contains(&tip) {
            continue
        }

        let d = -d;

        'phrases: for phrase in phrases.iter() {
            let mut next = tip.clone();
            for c in phrase.iter() {
                next = next.apply(c);
                if parents.contains_key(&next) || !board.check_unit_position(&next) {
                    continue 'phrases
                }
            }

            power.insert(tip.clone());
            let score = (max - phrase.len() + 1) as i32;
            if d + score < *dist.get(&next).unwrap_or(&i32::max_value()) {
                q.push((-(d + score), next.clone()));
                dist.insert(next.clone(), d + score);
                let mut next = tip.clone();
                for c in phrase {
                    let next_next = next.apply(c);
                    dist.insert(next_next.clone(), 0);
                    parents.insert(next_next.clone(), (*c, next));
                    next = next_next;
                }
            }
        }

        for c in ALL_COMMANDS.iter() {
            let next = tip.apply(c);
            if board.check_unit_position(&next) {
                if d + penalty < *dist.get(&next).unwrap_or(&i32::max_value()) {
                    q.push((-(d + penalty), next.clone()));
                    dist.insert(next.clone(), d + penalty);
                    parents.insert(next.clone(), (*c, tip.clone()));
                }
            }
        }
    }

    if !parents.contains_key(target) {
        return None;  // no path found.
    }

    let mut seen: HashSet<Vec<(i32, i32)>> = HashSet::new();
    seen.insert(xy(&target));

    let mut path = Vec::new();
    let mut tip = target;
    while tip != source {
        assert!(parents.contains_key(&tip));
        let (c, ref next) = parents[tip];
        path.push(c);
        let xy = xy(&next);
        if seen.contains(&xy) {
            // got cycles? try again.
            return route_because_it_works(source, target, board);
        }

        seen.insert(xy);
        tip = next;
    }
    path.reverse();

    for c in ALL_COMMANDS.iter() {
        let locked = target.apply(c);
        if !board.check_unit_position(&locked) {
            assert!(!seen.contains(&xy(&locked)));
            path.push(*c);
            return Some(path)
        }
    }

    panic!("sai wat?");
}

fn reachable<'a>(source: &Unit<'a>, board: &Board) -> HashSet<Unit<'a>> {
    let mut q = VecDeque::new();
    q.push_back(source.clone());
    let mut seen: HashSet<Unit<'a>> = HashSet::new();
    seen.insert(source.clone());
    while let Some(tip) = q.pop_front() {
        assert!(board.check_unit_position(&tip));

        for cj in ALL_COMMANDS.iter() {
            let next = tip.apply(cj);
            if !seen.contains(&next) && board.check_unit_position(&next) {
                q.push_back(next.clone());
                seen.insert(next);
            }
        }
    }

    seen
}


fn can_be_locked<'a>(unit: &Unit<'a>, board: &Board) -> bool {
    for c in ALL_COMMANDS.iter() {
        let locked = unit.apply(c);
        if !board.check_unit_position(&locked) {
            return true
        }
    }
    return false;
}


pub fn candidates<'a>(unit: &Unit<'a>, board: &Board) -> Vec<Unit<'a>>{
    let mut result = Vec::new();
    let r = reachable(unit, board);
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
                if board.check_unit_position(&moved) && r.contains(&moved)
                    && can_be_locked(&moved, board) {
                    result.push(moved);
                }
            }
        }
    }
    result
}

pub fn best_position<'a>(unit: &Unit<'a>, next_unit: &Option<Unit<'a>>,
                         board: &Board) -> Vec<Unit<'a>> {
    let mut result = Vec::new();
    for moved in candidates(unit, board) {
        let board_with_moved = board.lock_unit(&moved).0;
        let score = match next_unit {
            &None => scoring_function(&board_with_moved),
            &Some(ref next) => {
                if board_with_moved.check_unit_position(&next) {
                    candidates(next, &board_with_moved).iter()
                        .map(|c| scoring_function(&board_with_moved.lock_unit(&c).0))
                        .max().unwrap_or(0)
                } else {
                     scoring_function(&board_with_moved)
                }

            }
        };
        result.push((moved, score));
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


pub fn play<'a>(g: &'a Game, phrases: &Vec<Vec<Command>>) -> (Vec<Command>, Vec<GamePosition<'a>>) {
    let mut cur_game_pos = GamePosition::start(g);
    let mut commands: Vec<Command> = Vec::new();
    let mut positions: Vec<GamePosition> = vec![cur_game_pos.clone()];
    let mut i = 0;
    'outer: while cur_game_pos.board.check_unit_position(&cur_game_pos.unit) {
        i += 1;
        // let mut stderr = io::stderr();
        // writeln!(&mut stderr, "{} out of {}", i, g.source.len()).unwrap();
        let best_positions = best_position(&cur_game_pos.unit,
                                           &cur_game_pos.next_unit(),
                                           &cur_game_pos.board);
        let mut moved = false;
        for target in best_positions {
            if let Some(new_commands) = route(&cur_game_pos.unit, &target,
                                              &cur_game_pos.board, phrases) {
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
