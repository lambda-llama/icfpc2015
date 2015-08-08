use std::rc::Rc;
use hex2d::{Coordinate, ToCoordinate};
use game::{Unit, Command, ALL_COMMANDS};


#[derive(RustcEncodable, Clone)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    cells: Rc<Vec<Vec<bool>>>
}

impl Board {
    pub fn new<I>(width: usize, height: usize, filled: I) -> Board
        where I: Iterator<Item=(i32, i32)>
    {
        let mut cells = vec![vec![false; width]; height];
        for (x, y) in filled {
            assert!(0 <= x && x <= width as i32);
            assert!(0 <= y && y <= height as i32);
            cells[y as usize][x as usize] = true;
        }
        Board {
            width: width,
            height: height,
            cells: Rc::new(cells)
        }
    }

    /// Returns `true` if a `unit` is within board boundaries and does
    /// not overlap any of the occupied cells.
    pub fn check_unit_position(&self, unit: &Unit) -> bool {
        unit.iter().all(|c| {
            let Coordinate { x, y } = cube_to_offset(&c);
            self.is_valid(x, y) && self.is_free(x, y)
        })
    }

    /// Return `true` if the command is locking the unit.
    // pub fn is_command_locking(&self, unit: &Unit, c: &Command) {
    //     unit.apply(c).iter().all(|c| {
    //         let Coordinate { x, y } = cube_to_offset(&c);
    //         self.is_valid(x, y) && self.is_free(x, y)
    //     })
    // }

    pub fn get_correct_commands(&self, unit: &Unit) -> Vec<&Command> {
        ALL_COMMANDS.iter().filter(|c| {
            self.check_unit_position(&unit.apply(c))
        }).collect()
    }

    pub fn is_free(&self, x: i32, y: i32) -> bool {
        assert!(self.is_valid(x, y));
        !self.cells[y as usize][x as usize]
    }

    pub fn place_new_unit(&self, unit: &Unit) -> Unit {
        let target_y = unit.pivot.y - unit.border_top();
        let target_x = unit.pivot.x  - unit.border_left() + (self.width as i32 - unit.width()) / 2;
        unit.move_to((target_x, target_y))
    }

    pub fn lock_unit(&self, unit: &Unit) -> Board {
        let mut cells = (*self.cells).clone();
        for c in unit.iter() {
            let Coordinate { x, y } = cube_to_offset(&c);
            assert!(self.is_free(x, y));
            cells[y as usize][x as usize] = true;
        }
        Board {
            cells: Rc::new(cells),
            ..*self
        }
    }

    fn is_valid(&self, x: i32, y: i32) -> bool {
        (0 <= x && x < self.width as i32) &&
        (0 <= y && y < self.height as i32)
    }
}

pub fn cube_to_offset<C>(c: &C) -> Coordinate where C: ToCoordinate + Copy {
    let c = c.to_coordinate();
    let z = c.x - c.y;
    return Coordinate {
        x: c.x + (z + (z & 1)) / 2,
        y: z
    }
}

pub fn offset_to_cube(c: &Coordinate) -> Coordinate {
    let x = c.x - (c.y + (c.y & 1)) / 2;
    let z = c.y;
    return Coordinate { x: x, y: -x - z }
}
