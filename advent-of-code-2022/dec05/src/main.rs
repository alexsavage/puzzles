use regex::Regex;
use std::io::stdin;

enum InputState {
    Cargo,
    Skipping,
    Moves,
}

fn main() {
    let move_exp = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();

    let mut ship: Vec<String> = vec![String::default();9];

    let mut state = InputState::Cargo;

    for line in stdin().lines().flatten() {
        match state {
            InputState::Cargo => {
                if line.chars().nth(1).unwrap_or_default().is_ascii_digit() {
                    state = InputState::Skipping;
                } else {
                    for i in 0..(line.len() / 4) + 1 {
                        let cargo_item = line.chars().nth(i * 4 + 1).unwrap_or_default();
                        if cargo_item.is_ascii_uppercase() {
                            ship[i].insert(0, cargo_item);
                        }
                    }
                }
            }
            InputState::Skipping => {
                state = InputState::Moves;
            }
            InputState::Moves => {
                let moves = move_exp.captures(&line).unwrap();
                let count = moves[1].parse::<usize>().unwrap();
                let from = moves[2].parse::<usize>().unwrap() - 1;
                let to = moves[3].parse::<usize>().unwrap() - 1;

                let mut pile = ship[from].clone();
                let range = pile.len()-count..;
                ship[to].push_str(pile.drain(range).as_str());
                ship[from] = pile;
            }
        }
    }
    let mut result = String::with_capacity(ship.len());
    for pile in &ship {
        if let Some(base) = pile.chars().nth_back(0) {
            result.push(base);
        }
    }
    println!("Result: {}", result);
}
