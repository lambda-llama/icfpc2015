use game::{Command};
use hex2d::{Angle, Direction};

static YX_SYMBOLS: [char; 6] = [
    'p', '\'', '!', '.', '0', '3'
];

static XY_SYMBOLS: [char; 6] = [
    'b', 'c', 'e', 'f', 'y', '2'
];

static ZX_SYMBOLS: [char; 6] = [  // SW.
    'a', 'g', 'h', 'i', 'j', '4'
];

static ZY_SYMBOLS: [char; 6] = [  // SE.
    'l', 'm', 'n', 'o', ' ', '5'
];

static L_SYMBOLS: [char; 6] = [
    'k', 's', 't', 'u', 'w', 'x'
];

static R_SYMBOLS: [char; 6] = [
    'd', 'q', 'r', 'v', 'z', '1'
];

fn get_symbols(c: Command) -> [char; 6] {
    match c {
        Command::Move(Direction::YX) => YX_SYMBOLS,
        Command::Move(Direction::XY) => XY_SYMBOLS,
        Command::Move(Direction::ZX) => ZX_SYMBOLS,
        Command::Move(Direction::ZY) => ZY_SYMBOLS,
        Command::Rotate(Angle::Left) => L_SYMBOLS,
        Command::Rotate(Angle::Right) => R_SYMBOLS,
        _ => panic!()
    }
}

pub fn encode(commands: Vec<Command>) -> String {
    let mut res = Vec::new();
    for c in commands {
        res.push(get_symbols(c)[0]);
    };
    res.into_iter().collect()
}
