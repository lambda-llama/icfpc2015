use game::{Unit, Command, ALL_COMMANDS};


#[derive(RustcEncodable, Clone)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<bool>>
}

impl Board {
    pub fn check_unit_position(&self, unit: &Unit) -> bool {
        for cell in &unit.cells {
            if self.cells[cell.x as usize][cell.y as usize] ||
                cell.x < 0 || cell.x >= self.width as i32    ||
                cell.y < 0 || cell.y >= self.height as i32 {
                return false
            }
        }
        true
    }

    pub fn get_correct_commands(&self, unit: &Unit) -> Vec<&Command> {
        ALL_COMMANDS.iter().filter(|c| {
            self.check_unit_position(&unit.apply(c))
        }).collect()
    }

    pub fn place_unit(&self, unit: &Unit) -> Board {
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
