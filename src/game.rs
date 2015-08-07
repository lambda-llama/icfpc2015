use hex2d::{self, Angle, Coordinate, Direction};

#[derive(RustcEncodable, Clone)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<bool>>
}

pub struct Game {
    pub board: Board,
    pub source: Vec<Unit>
}

struct GamePosition {
    game: Game,
    unit: Unit
}

impl Board {
    fn check_unit_position(&self, unit: &Unit) -> bool {
        unimplemented!()
    }

    fn get_correct_commands(&self, unit: &Unit) -> Vec<Command> {
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

impl GamePosition {
    fn start(mut g: Game) -> GamePosition {
        let unit = g.source.remove(0);
        GamePosition {
            game: g,
            unit: unit
        }
    }

    fn step(&self, c: &Command) -> Option<GamePosition> {
        let unit = self.unit.apply(c);
        if self.game.board.check_unit_position(&unit) {
            if self.game.board.get_correct_commands(&unit).len() == 0 {
                unimplemented!();
            }
            else {
                unimplemented!();
            }
        }
        else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Unit {
    pub cells: Vec<hex2d::Coordinate>,
    pub pivot: hex2d::Coordinate
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
}
