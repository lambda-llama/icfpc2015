use hex2d::{self, Angle, Coordinate, Direction};

#[derive(RustcEncodable, Clone)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<bool>>
}

impl Board {
    pub fn check_unit_position(&self, unit: &Unit) -> bool {
        unimplemented!()
    }

    pub fn get_correct_commands(&self, unit: &Unit) -> Vec<Command> {
        unimplemented!()
    }

    fn place_unit(&self, unit: &Unit) -> Board {
        let mut clone = self.clone();
        for cell in unit.cells.iter() {
            let x = cell.x as usize;
            let y = cell.y as usize;
            assert!(!clone.cells[x][y]);
            clone.cells[x][y] = true
        }

        clone
    }
}

pub struct Game {
    pub board: Board,
    pub source: Vec<Unit>
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
            board : g.board.clone(),
            unit: g.source[0].clone(),
            next_source: 1
        }
    }

    fn next_unit(&self) -> Option<GamePosition> {
        let board = self.game.board.place_unit(&self.unit);
        if (self.next_source + 1 < self.game.source.len()) {
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

    fn step(&self, c: &Command) -> Option<GamePosition> {
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

pub enum Command {
    Move(hex2d::Direction),
    Rotate(hex2d::Angle)
}

static ALL_COMMANDS : [Command; 6] = [
    Command::Move(Direction::YX),
    Command::Move(Direction::XY),
    Command::Move(Direction::ZX),
    Command::Move(Direction::ZY),
    Command::Rotate(Angle::Left),
    Command::Rotate(Angle::Right)
];

#[derive(Clone)]
pub struct Unit {
    pub cells: Vec<hex2d::Coordinate>,
    pub pivot: hex2d::Coordinate
}

impl Unit {
    fn apply(&self, c: &Command) -> Unit {
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

    pub fn move_to(&self, new_pivot: hex2d::Coordinate) -> Unit {
        Unit {
            cells: self.cells.clone(),
            pivot: new_pivot
        }
    }
}
