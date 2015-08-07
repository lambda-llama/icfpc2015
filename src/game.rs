use hex2d::{Angle, Coordinate, Direction};
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
            if let Some(next) = result.last().unwrap().step(m)  {
                result.push(next);
            } else {
                break;
            }
        }
        result
    }
}

struct GamePosition<'a> {
    game: &'a Game,
    board: Board,
    unit: Unit,
    next_source: usize
}

impl<'a> GamePosition<'a> {
    fn start(g: &Game) -> GamePosition {
        GamePosition {
            game: g,
            board: g.board.clone(),
            unit: g.source[0].clone(),
            next_source: 1
        }
    }

    fn next_unit(&self) -> Option<GamePosition<'a>> {
        let board = self.game.board.place_unit(&self.unit);
        if self.next_source + 1 < self.game.source.len() {
            Some(GamePosition {
                board: board,
                unit: self.game.source[self.next_source].clone(),
                next_source: self.next_source + 1,
                ..*self
            })
        }
        else {
            None
        }
    }

    fn step(&self, c: &Command) -> Option<GamePosition<'a>> {
        let unit = self.unit.apply(c);
        if self.game.board.check_unit_position(&unit) {
            if self.game.board.get_correct_commands(&unit).len() == 0 {
                self.next_unit()
            }
            else {
                Some(GamePosition {
                    unit: unit,
                    board: self.board.clone(),
                    ..*self
                })
            }
        }
        else {
            None
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
    Command::Move(Direction::ZX),
    Command::Move(Direction::ZY),
    Command::Rotate(Angle::Left),
    Command::Rotate(Angle::Right)
];

#[derive(Clone, PartialEq, Eq, Hash)]
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

    pub fn iter<'a>(&'a self) -> Box<Iterator<Item=(i32, i32)> + 'a> {
        Box::new(self.cells.iter().map(|c| (c.x, c.y)))
    }

    pub fn apply(&self, c: &Command) -> Unit {
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

    pub fn move_to(&self, new_pivot: Coordinate) -> Unit {
        Unit {
            cells: self.cells.iter()
                .map(|c| c - self.pivot + new_pivot).collect(),
            pivot: new_pivot
        }
    }
}
