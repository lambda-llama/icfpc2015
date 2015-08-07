use std::rc::Rc;
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
        unit.iter().all(|(x, y)| self.is_valid(x, y) && self.is_free(x, y))
    }

    pub fn get_correct_commands(&self, unit: &Unit) -> Vec<&Command> {
        ALL_COMMANDS.iter().filter(|c| {
            self.check_unit_position(&unit.apply(c))
        }).collect()
    }

    pub fn is_free(&self, x: i32, y: i32) -> bool {
        assert!(self.is_valid(x, y));
        !self.cells[y as usize][x as usize]
    }

    pub fn place_unit(&self, unit: &Unit) -> Board {
        let mut cells = (*self.cells).clone();
        for (x, y) in unit.iter() {
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
