use hex2d::{Angle, Coordinate, Direction, ToCoordinate};
use board::Board;

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


struct GamePosition<'a> {
    game: &'a Game,
    pub board: Board,
    unit: Unit,
    next_source: usize
}

impl<'a> GamePosition<'a> {
    pub fn to_state(&self) -> GameState {
        let pivot = (self.unit.pivot.x, self.unit.pivot.y);
        let cells: Vec<(i32, i32)> = self.unit.iter().collect();
        GameState {
            board: self.board.clone(),
            unit: UnitState {
                pivot: pivot,
                cells: cells
            }
        }
    }

    fn start(g: &Game) -> GamePosition {
        GamePosition {
            game: g,
            board: g.board.clone(),
            unit: g.board.place_new_unit(&g.source[0]),
            next_source: 1
        }
    }

    fn lock_current_unit(&self) -> GamePosition<'a> {
        let board = self.game.board.lock_unit(&self.unit);
        let unit = self.game.board.place_new_unit(&self.game.source[self.next_source]);
        GamePosition {
            board: board,
            unit: unit,
            next_source: self.next_source + 1,
            ..*self
        }
    }

    fn step(&self, c: &Command) -> GamePosition<'a> {
        let unit = self.unit.apply(c);
        if self.game.board.check_unit_position(&unit) {
            GamePosition {
                unit: unit,
                board: self.board.clone(),
                ..*self
            }
        } else {
            self.lock_current_unit()
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Command {
    Move(Direction),
    Rotate(Angle)
}

pub static ALL_COMMANDS : [Command; 6] = [
    Command::Move(Direction::YX),
    Command::Move(Direction::XY),
    Command::Move(Direction::XZ),
    Command::Move(Direction::YZ),
    Command::Rotate(Angle::Left),
    Command::Rotate(Angle::Right)
];

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Unit {
    cells: Vec<Coordinate>,
    pub pivot: Coordinate
}

impl Unit {
    pub fn new(pivot: Coordinate, cells: Vec<Coordinate>) -> Unit {
        Unit {
            pivot: pivot,
            cells: cells
        }
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

    pub fn iter<'a>(&'a self) -> Box<Iterator<Item=(i32, i32)> + 'a> {
        Box::new(self.cells.iter().map(|c| (c.x, c.y)))
    }

    pub fn apply(&self, c: &Command) -> Unit {
        match c {
            &Command::Move(d)   => {
                assert!(d == Direction::YX ||  // West
                        d == Direction::XY ||  // East
                        d == Direction::XZ ||  // SW
                        d == Direction::YZ);   // SE
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
