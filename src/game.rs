use hex2d::{Angle, Coordinate, Direction, ToCoordinate};
use board::{Board, cube_to_offset, offset_to_cube};

pub struct Game {
    pub board: Board,
    pub source: Vec<Unit>
}

impl Game {
    pub fn play<'a>(&'a self, moves: &Vec<Command>) -> Vec<GamePosition<'a>> {
        let mut result: Vec<GamePosition<'a>>  = Vec::new();
        let start = GamePosition::start(self);
        result.push(start);
        for m in moves {
            let next = result.last().unwrap().step(m);
            result.push(next);
        }
        result
    }
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
}


pub struct GamePosition<'a> {
    pub game: &'a Game,
    pub board: Board,
    pub unit: Unit,
    pub next_source: usize
}

impl<'a> GamePosition<'a> {
    pub fn to_state(&self) -> GameState {
        let cells: Vec<(i32, i32)> = self.unit.iter()
            .map(|c| cube_to_offset(&c)).collect();
        GameState {
            board: self.board.clone(),
            unit: UnitState {
                pivot: cube_to_offset(&self.unit.pivot),
                cells: cells
            }
        }
    }

    pub fn start(g: &Game) -> GamePosition {
        GamePosition {
            game: g,
            board: g.board.clone(),
            unit: g.board.place_new_unit(&g.source[0]),
            next_source: 1
        }
    }

    pub fn lock_current_unit(&self) -> GamePosition<'a> {
        let (board, cleared_lines) = self.game.board.lock_unit(&self.unit);
        let unit = self.game.board.place_new_unit(&self.game.source[self.next_source]);
        GamePosition {
            board: board,
            unit: unit,
            next_source: self.next_source + 1,
            ..*self
        }
    }

    pub fn step(&self, c: &Command) -> GamePosition<'a> {
        let unit = self.unit.apply(c);
        if self.game.board.check_unit_position(&unit) {
            GamePosition {
                unit: unit,
                board: self.board.clone(),
                ..*self
            }
        } else {
            println!("LOCKED!");
            self.lock_current_unit()
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
                    Direction::ZY => "SW".to_string(),
                    Direction::ZX => "SE".to_string(),
                    _             => "UP".to_string()
                };
                format!("Move: {}", dir_str)
            }
            &Command::Rotate(ang) => {
                let ang_str = match ang {
                    Angle::Left  => "Left".to_string(),
                    Angle::Right => "Right".to_string(),
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
    Command::Move(Direction::YX),
    Command::Move(Direction::XY),
    Command::Move(Direction::ZX),
    Command::Move(Direction::ZY),
    Command::Rotate(Angle::Left),
    Command::Rotate(Angle::Right)
];

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Unit {
    pub cells: Vec<Coordinate>,
    pub pivot: Coordinate
}

impl Unit {
    pub fn new(pivot: Coordinate, cells: Vec<Coordinate>) -> Unit {
        Unit {
            pivot: pivot,
            cells: cells
        }
    }

    pub fn iter<'a>(&'a self) -> Box<Iterator<Item=(i32, i32)> + 'a> {
        Box::new(self.cells.iter().map(cube_to_offset))
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

    pub fn apply(&self, c: &Command) -> Unit {
        match c {
            &Command::Move(d)   => {
                assert!(d == Direction::YX ||  // West
                        d == Direction::XY ||  // East
                        d == Direction::ZY ||  // SW
                        d == Direction::ZX);   // SE
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

    pub fn move_to<C>(&self, new_pivot: C) -> Unit
        where C: ToCoordinate + Copy
    {
        Unit {
            cells: self.cells.iter()
                .map(|&c| c - self.pivot + new_pivot).collect(),
            pivot: new_pivot.to_coordinate()
        }
    }
}
