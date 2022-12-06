use regex::Regex;
use std::io::stdin;

enum InputState {
    Cargo,
    Skipping,
    Moves,
}

fn main() {
    let move_exp = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();

    let mut ship1: Vec<String> = vec![String::default(); 9];
    let mut ship2: Vec<String> = vec![String::default(); 9];

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
                            ship1[i].insert(0, cargo_item);
                            ship2[i].insert(0, cargo_item);
                        }
                    }
                }
            }
            InputState::Skipping => {
                state = InputState::Moves;
            }
            InputState::Moves => {
                let moves = move_exp.captures(&line).unwrap();
                let mut count = moves[1].parse::<usize>().unwrap();
                let from = moves[2].parse::<usize>().unwrap() - 1;
                let to = moves[3].parse::<usize>().unwrap() - 1;

                // Part 2 first because it doesn't modify count
                let mut pile = ship2[from].clone();
                let range = pile.len() - count..;
                ship2[to].push_str(pile.drain(range).as_str());
                ship2[from] = pile;

                while count > 0 {
                    let x = ship1[from].pop().unwrap();
                    ship1[to].push(x);
                    count -= 1;
                }
            }
        }
    }

    let mut result1 = String::with_capacity(ship1.len());
    for pile in &ship1 {
        if let Some(base) = pile.chars().nth_back(0) {
            result1.push(base);
        }
    }
    println!("Result 1: {}", result1);

    let mut result2 = String::with_capacity(ship2.len());
    for pile in &ship2 {
        if let Some(base) = pile.chars().nth_back(0) {
            result2.push(base);
        }
    }
    println!("Result 2: {}", result2);
}
