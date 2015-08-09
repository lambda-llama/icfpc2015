use std::rc::Rc;
use hex2d::{Coordinate, ToCoordinate};
use game::Unit;
use rand;

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
            assert!(0 <= x && x < width as i32);
            assert!(0 <= y && y < height as i32);
            cells[y as usize][x as usize] = true;
        }
        Board {
            width: width,
            height: height,
            cells: Rc::new(cells)
        }
    }

    pub fn n_clear_top_rows(&self) -> usize {
        self.cells.iter()
            .take_while(|row| row.iter().all(|c| !c))
            .count()
    }

    pub fn total_sum(&self, penalty_per_row: &Vec<i64>) -> i64 {
        assert!(penalty_per_row.len() == self.height);
        self.cells.iter()
            .zip(penalty_per_row)
            .map(|(row, penalty)| (row.iter().filter(|&&x| x).count() as i64) * penalty)
            .fold(0, |a, b| a + b) // sum is ustable tt
    }

    pub fn n_full_rows(&self) -> usize {
        self.cells.iter()
            .filter(|&l| Board::check_line_filled(&l))
            .count()
    }

    pub fn n_holes(&self) -> usize {
        return (0..self.width).flat_map(|x| {
            (0..self.height).map(move |y| (x, y))
        }).filter(|&(x, y)| self.is_hole(x as i32, y as i32)).count()
    }

    pub fn is_hole(&self, x: i32, y: i32) -> bool {
        if !self.is_free(x as i32, y as i32) {
            return false
        }
        let mut free_neighbours = 0;
        let t = offset_to_cube(&(x, y));
        for n in t.neighbors().iter() {
            let (x, y) = cube_to_offset(n);
            if self.is_valid(x, y) && self.is_free(x, y) {
                free_neighbours += 1;
            }
        }

        return free_neighbours <= 1
    }

    /// Returns `true` if a `unit` is within board boundaries and does
    /// not overlap any of the occupied cells.
    pub fn check_unit_position(&self, unit: &Unit) -> bool {
        unit.iter().all(|(x, y)| {
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

    fn check_line_filled(line: &Vec<bool>) -> bool {
        line.iter().all(|&c| c)
    }

    pub fn clear_filled_lines(&self, cells: &Vec<Vec<bool>>) -> (Board, i32) {
        let mut old_cells : Vec<Vec<bool>> = Vec::new();

        for line in cells.iter() {
            if !Board::check_line_filled(&line) {
                old_cells.push(line.clone())
            }
        }
        let lines_cleared = self.height - old_cells.len();

        let mut new_cells = vec![vec![false; self.width]; lines_cleared];
        new_cells.extend(old_cells);

        let board = Board {
            cells: Rc::new(new_cells),
            ..*self
        };
        (board, lines_cleared as i32)
    }

    pub fn is_free(&self, x: i32, y: i32) -> bool {
        assert!(self.is_valid(x, y));
        !self.cells[y as usize][x as usize]
    }

    pub fn place_new_unit<'a>(&self, cells: &'a Vec<Coordinate>) -> Unit<'a> {
        let unit = Unit::new(cells);
        let (x, y) = cube_to_offset(&unit.position.to_coordinate());
        let target_y = y - unit.border_top();
        let unit = unit.move_to(offset_to_cube(&(0, target_y)));

        let (x, y) = cube_to_offset(&unit.position.to_coordinate());
        let target_y = y - unit.border_top();
        let target_x = x - unit.border_left() + (self.width as i32 - unit.width()) / 2;
        let to = offset_to_cube(&(target_x, target_y));
        unit.move_to(to)
    }

    pub fn lock_unit(&self, unit: &Unit) -> (Board, i32) {
        let mut cells = (*self.cells).clone();
        for (x, y) in unit.iter() {
            assert!(self.is_free(x, y));
            cells[y as usize][x as usize] = true;
        }
        self.clear_filled_lines(&cells)
    }

    fn is_valid(&self, x: i32, y: i32) -> bool {
        (0 <= x && x < self.width as i32) &&
        (0 <= y && y < self.height as i32)
    }
}

pub fn cube_to_offset<C>(c: &C) -> (i32, i32) where C: ToCoordinate + Copy {
    let c = c.to_coordinate();
    let z = c.z();
    let col = c.x + (z - (z & 1)) / 2;
    let row = z;
    return (col, row)
}

pub fn offset_to_cube<C>(c: &C) -> Coordinate where C: ToCoordinate + Copy{
    let c = c.to_coordinate();
    let x = c.x - (c.y - (c.y & 1)) / 2;
    let z = c.y;
    return Coordinate { x: x, y: -x -z }
}

#[test]
fn offset_cube_id() {
    assert!(cube_to_offset(&offset_to_cube(&(0, 0))) == (0, 0));
    assert!(cube_to_offset(&offset_to_cube(&(8, 8))) == (8, 8));

    for _ in 0..500 {
        let x = rand::random::<u8>() as i32;
        let y = rand::random::<u8>() as i32;
        assert!(cube_to_offset(&offset_to_cube(&(x, y))) == (x, y));
    }
}
