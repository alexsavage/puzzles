use std::io::stdin;

const SCORE_ROCK: i32 = 1;
const SCORE_PAPER: i32 = 2;
const SCORE_SCISSORS: i32 = 3;

const SCORE_LOSE: i32 = 0;
const SCORE_DRAW: i32 = 3;
const SCORE_WIN: i32 = 6;

enum Move {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

fn score_round(player: char, opponent: char) -> i32 {
    let player_move = match player {
        'X' => Move::Rock,
        'Y' => Move::Paper,
        'Z' => Move::Scissors,
        _ => panic!()
    };

    let opponent_move = match opponent {
        'A' => Move::Rock,
        'B' => Move::Paper,
        'C' => Move::Scissors,
        _ => panic!()
    };

    let mut result = match player_move {
        Move::Rock => SCORE_ROCK,
        Move::Paper => SCORE_PAPER,
        Move::Scissors => SCORE_SCISSORS,
    };
    
    result += match (player_move, opponent_move) {
        (Move::Rock, Move::Paper) => SCORE_LOSE,
        (Move::Rock, Move::Scissors) => SCORE_WIN,
        (Move::Paper, Move::Rock) => SCORE_WIN,
        (Move::Paper, Move::Scissors) => SCORE_LOSE,
        (Move::Scissors, Move::Rock) => SCORE_LOSE,
        (Move::Scissors, Move::Paper) => SCORE_WIN,
        _ => SCORE_DRAW
    };

    result
}

fn score_round_b(rig: char, opponent: char) -> i32 {
    let opponent_move = match opponent {
        'A' => Move::Rock,
        'B' => Move::Paper,
        'C' => Move::Scissors,
        _ => panic!(),
    };

    let rig_outcome = match rig {
        'X' => Outcome::Lose,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => panic!(),
    };

    let mut result = match rig_outcome {
        Outcome::Win => SCORE_WIN,
        Outcome::Lose => SCORE_LOSE,
        Outcome::Draw => SCORE_DRAW,
    };

    let player_move = match (opponent_move, rig_outcome) {
        (Move::Rock, Outcome::Lose) => Move::Scissors,
        (Move::Rock, Outcome::Win) => Move::Paper,
        (Move::Paper, Outcome::Lose) => Move::Rock,
        (Move::Paper, Outcome::Win) => Move::Scissors,
        (Move::Scissors, Outcome::Lose) => Move::Paper,
        (Move::Scissors, Outcome::Win) => Move::Rock,
        (draw, _) => draw, // match to draw
    };

    result += match player_move {
        Move::Rock => SCORE_ROCK,
        Move::Paper => SCORE_PAPER,
        Move::Scissors => SCORE_SCISSORS,
    };

    result
}

fn main() {
    let mut score_a = 0;
    let mut score_b = 0;
    for line in stdin().lines().flatten() {
        let a = line.chars().nth(0).unwrap();
        let b = line.chars().nth(2).unwrap();
        score_a += score_round(b, a);
        score_b += score_round_b(b, a);
    }

    println!("Part A score: {}", score_a);
    println!("Part B score: {}", score_b);
}
