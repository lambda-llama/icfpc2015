type Pos = (usize, usize)

struct Board {
    width: usize,
    height: usize,
    cells: Vec<Vec<bool>>>
}

struct UnitShape {
    cells: Vec<Pos>,
    pivot: Pos
}

struct Unit {
    shape: UnitType,
    position: Pos,               // pivot positon
    rotation: u8                 // 0..5
}

enum Move {
    East, West, SouthEast, SouthWest;
}

enum Turn {
    Clockwise,
    CounterClockwise
}
