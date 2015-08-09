use std::hash::{Hash, Hasher};
use std::cmp::{Eq, PartialEq, Ord, PartialOrd, Ordering};

use hex2d::{Angle, Coordinate, Direction, ToCoordinate, Position, ToDirection};

use board::{Board, cube_to_offset};
use scoring::move_score;

pub struct Game {
    pub board: Board,
    pub source: Vec<Vec<Coordinate>>,
    pub seed: u64
}

#[derive(RustcEncodable)]
struct UnitState {
    pivot: (i32, i32),
    cells: Vec<(i32, i32)>
}

#[derive(RustcEncodable)]
struct GameState {
    pub board: Board,
    pub unit: UnitState,
    pub previous_move: String
}


#[derive(Clone)]
pub struct GamePosition<'a> {
    pub game: &'a Game,
    pub board: Board,
    pub unit: Unit<'a>,
    pub sum_unit_size: i32,
    pub next_source: usize,
    pub cleared_lines_prev: i32,
    pub score: i32,
    pub previous_move: Option<Command>
}

impl<'a> GamePosition<'a> {
    pub fn to_state(&self) -> GameState {
        let cells: Vec<(i32, i32)> = self.unit.iter().collect();
        GameState {
            board: self.board.clone(),
            unit: UnitState {
                pivot: cube_to_offset(&self.unit.position.to_coordinate()),
                cells: cells
            },
            previous_move: self.previous_move.map(|c| c.to_string()).unwrap_or("".to_string())
        }
    }

    pub fn start(g: &Game) -> GamePosition {
        GamePosition {
            game: g,
            board: g.board.clone(),
            unit: g.board.place_new_unit(&g.source[0]),
            sum_unit_size: 0,
            next_source: 1,
            cleared_lines_prev: 0,
            score: 0,
            previous_move: None
        }
    }

    pub fn lock_current_unit(&self, c: Command) -> Option<GamePosition<'a>> {
        if !(self.next_source < self.game.source.len()) {
            return None
        }

        let (board, cleared_lines) = self.board.lock_unit(&self.unit);
        let unit = self.board.place_new_unit(&self.game.source[self.next_source]);
        let sum_unit_size = self.sum_unit_size + self.unit.size();
        let new_score = self.score + move_score(self.unit.size(),
                                                cleared_lines,
                                                self.cleared_lines_prev);
        Some(GamePosition {
            game: self.game,
            board: board,
            unit: unit,
            sum_unit_size: sum_unit_size,
            next_source: self.next_source + 1,
            cleared_lines_prev: cleared_lines,
            score: new_score,
            previous_move: Some(c)
        })
    }

    pub fn step(&self, c: Command) -> Option<GamePosition<'a>> {
        let unit = self.unit.apply(&c);
        if self.board.check_unit_position(&unit) {
            Some(GamePosition {
                unit: unit,
                board: self.board.clone(),
                previous_move: Some(c),
                ..*self
            })
        } else {
            self.lock_current_unit(c)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Command {
    Move(Direction),
    Rotate(Angle)
}

impl ToString for Command {
    fn to_string(&self) -> String {
        match self {
            &Command::Move(dir) => {
                let dir_str = match dir {
                    Direction::YX => "West".to_string(),
                    Direction::XY => "East".to_string(),
                    Direction::ZY => "SE".to_string(),
                    Direction::ZX => "SW".to_string(),
                    _             => "UP".to_string()
                };
                format!("Move: {}", dir_str)
            }
            &Command::Rotate(ang) => {
                let ang_str = match ang {
                    Angle::Left  => "CCW".to_string(),
                    Angle::Right => "CW".to_string(),
                    _            => "Unexpected".to_string()
                };
                format!("Rotate: {}", ang_str)
            }
        }
    }
}

#[test]
fn to_string_test() {
    let s = ALL_COMMANDS[0].to_string();
    println!("{}", s);
}


pub static ALL_COMMANDS : [Command; 6] = [
    Command::Move(Direction::YX),  // West
    Command::Move(Direction::XY),  // East
    Command::Move(Direction::ZX),  // SW
    Command::Move(Direction::ZY),  // SE
    Command::Rotate(Angle::Left),  // CCW
    Command::Rotate(Angle::Right)  // CW
];

#[derive(Clone, Debug)]
pub struct Unit<'a> {
    cells: &'a Vec<Coordinate>,
    pub position: Position,
}

impl<'a> Hash for Unit<'a> {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.position.hash(state)
    }
}

impl<'a> PartialEq for Unit<'a> {
    fn eq(&self, other: &Unit) -> bool {
        self.position == other.position
    }
}

impl<'a> Eq for Unit<'a> {}

impl<'a> PartialOrd for Unit<'a> {
    fn partial_cmp(&self, other: &Unit) -> Option<Ordering> {
        self.position.partial_cmp(&other.position)
    }
}

impl<'a> Ord for Unit<'a> {
    fn cmp(&self, other: &Unit) -> Ordering {
        self.position.cmp(&other.position)
    }
}

impl<'a> Unit<'a> {
    pub fn new(cells: &'a Vec<Coordinate>) -> Unit<'a> {
        Unit {
            cells: cells,
            position: Position::new((0, 0).to_coordinate(), Direction::from_int(0))
        }
    }

        fn apply_to_coord(p: &Position, c: &Coordinate) -> Coordinate {
            let angle = p.to_direction() - Direction::from_int(0);
            let shift = p.to_coordinate();
            c.rotate_around_zero(angle) + shift
        }

    pub fn iter<'b>(&'b self) -> Box<Iterator<Item=(i32, i32)> + 'b> {
        let p = self.position;
        let it = self.cells.iter().map(move |&c| {
                cube_to_offset(&Unit::apply_to_coord(&p, &c))
        });
        Box::new(it)
    }

    pub fn border_top(&self) -> i32 {
        self.iter().map(|(_x, y)| y).min().unwrap()
    }

    pub fn border_left(&self) -> i32 {
        self.iter().map(|(x, _y)| x).min().unwrap()
    }

    pub fn border_right(&self) -> i32 {
        self.iter().map(|(x, _y)| x).max().unwrap()
    }

    pub fn width(&self) -> i32 {
        let result = self.border_right() - self.border_left() + 1;
        assert!(result > 0);
        result
    }

    pub fn size(&self) -> i32 {
        self.cells.len() as i32
    }

    pub fn apply(&self, c: &Command) -> Unit<'a> {
        match c {
            &Command::Move(d)   => {
                assert!(d == Direction::YX ||  // West
                        d == Direction::XY ||  // East
                        d == Direction::ZY ||  // SE
                        d == Direction::ZX);   // SW
                let position = self.position + d.to_coordinate();
                Unit { cells: self.cells, position: position}
            },
            &Command::Rotate(a) => {
                // Read as clockwise and counterclockwise.
                assert!(a == Angle::Right || a == Angle::Left);
                let position = self.position + a;
                Unit { cells: self.cells, position: position}
            }
        }
    }

    pub fn move_corner_to<C>(&self, to: C) -> Unit<'a> where C: ToCoordinate + Copy {
        let cell = Unit::apply_to_coord(&self.position, self.cells.first().unwrap());
        let diff = to.to_coordinate() - cell;
        Unit {
            cells: self.cells,
            position: self.position + diff
        }
    }

    pub fn move_to<C>(&self, target: C) -> Unit<'a>
        where C: ToCoordinate + Copy
    {
        let diff = target.to_coordinate() - self.position.to_coordinate();
        Unit {
            cells: self.cells,
            position: self.position + diff
        }
    }
}
