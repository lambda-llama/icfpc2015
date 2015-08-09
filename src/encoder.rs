use game::{Command};
use hex2d::{Angle, Direction};
use std::iter::{repeat};

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

fn get_symbols(c: &Command) -> [char; 6] {
    match c {
        &Command::Move(Direction::YX) => YX_SYMBOLS,
        &Command::Move(Direction::XY) => XY_SYMBOLS,
        &Command::Move(Direction::ZX) => ZX_SYMBOLS,
        &Command::Move(Direction::ZY) => ZY_SYMBOLS,
        &Command::Rotate(Angle::Left) => L_SYMBOLS,
        &Command::Rotate(Angle::Right) => R_SYMBOLS,
        _ => panic!("Wrong command")
    }
}

fn symbol_to_command(sym: char) -> Command {
    if YX_SYMBOLS.iter().any(|s| sym == *s) {
        Command::Move(Direction::YX)
    }
    else if XY_SYMBOLS.iter().any(|s| sym == *s) {
        Command::Move(Direction::XY)
    }
    else if ZX_SYMBOLS.iter().any(|s| sym == *s) {
        Command::Move(Direction::ZX)
    }
    else if ZY_SYMBOLS.iter().any(|s| sym == *s) {
        Command::Move(Direction::ZY)
    }
    else if L_SYMBOLS.iter().any(|s| sym == *s) {
        Command::Rotate(Angle::Left)
    }
    else if R_SYMBOLS.iter().any(|s| sym == *s) {
        Command::Rotate(Angle::Right)
    }
    else {
        panic!("No power!")
    }
}

fn phrase_to_commands(phrase: &String) -> Vec<Command> {
    phrase.chars().map(symbol_to_command).collect()
}

fn place_phrase(phrase: &String,
                commands: &Vec<Command>,
                result: &mut Vec<char>,
                used: &mut Vec<bool>) {
    let seq = phrase_to_commands(phrase);
    for i in 0..commands.len() - seq.len() {
        let mut flag = false;
        for j in 0..seq.len() {
            if used[i + j] || seq[j] != commands[i + j] {
                flag = true;
                break;
            }
        }
        if flag {
            continue;
        }
        for j in 0..seq.len() {
            result[i + j] = phrase.chars().nth(j).unwrap();
            used[i + j] = true;
        }
    }
}

pub fn encode(commands: &Vec<Command>, power_phrases: &Vec<String>) -> String {
    let mut sorted_phrases = power_phrases.clone();
    sorted_phrases.sort_by(|a, b| a.len().cmp(&b.len()));
    let mut result = repeat('*').take(commands.len()).collect::<Vec<_>>();
    let mut used = repeat(false).take(commands.len()).collect::<Vec<_>>();
    for p in &sorted_phrases {
        place_phrase(p, commands, &mut result, &mut used);
    };
    for i in 0..result.len() {
        if !used[i] {
            result[i] = get_symbols(&commands[i])[0];
        }
    };
    result.into_iter().collect()
}
