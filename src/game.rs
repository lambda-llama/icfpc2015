type Pos = (usize, usize);

struct Board {
    width: usize,
    height: usize,
    cells: Vec<Vec<bool>>
}

#[derive(Clone)]
struct UnitShape {
    cells: Vec<Pos>,
    pivot: Pos
}


enum Move {
    Translation(Pos),
    Rotation(i8)
}

#[derive(Clone)]
struct Unit {
    shape: UnitShape,
    position: Pos,               // pivot positon
    rotation: u8                 // 0..5
}

impl Unit {
    pub fn moved(self, m: &Move) -> Unit {
        match m {
            &Move::Translation((dx, dy)) => Unit {
                position: (self.position.0 + dx, self.position.1 + dy),
                ..self
            },
            &Move::Rotation(amount) => Unit {
                rotation: (((self.rotation as i32) + amount as i32) % 6) as u8,
                ..self
            }
        }
    }
}
