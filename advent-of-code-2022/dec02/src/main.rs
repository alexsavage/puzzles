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

fn main() {
    let mut score = 0;
    for line in stdin().lines().flatten() {
        let opponent = line.chars().nth(0).unwrap();
        let player = line.chars().nth(2).unwrap();
        score += score_round(player,opponent);
    }

    println!("Total score: {}", score);
}
