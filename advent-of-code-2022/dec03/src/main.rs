use std::io::stdin;

fn priority(item: char) -> usize {
    match item {
        'a'..='z' => item as usize - 96,      // 1..26
        'A'..='Z' => item as usize - 64 + 26, // 27..52
        _ => panic!(),
    }
}

fn find_overlap_1(line: &str) -> char {
    let mut seen: [bool; 52] = [false; 52];
    let chars = line.chars();
    let count = chars.count();

    for (index, item) in line.char_indices() {
        if index < count / 2 {
            seen[priority(item) - 1] = true;
        } else {
            if seen[priority(item) - 1] {
                return item;
            }
        }
    }
    panic!("No overlap found.")
}

fn find_overlap_2(line1: &str, line2: &str, line3: &str) -> char {
    let mut seen: [u8; 52] = [0; 52];

    for item in line1.chars() {
        let index = priority(item) - 1;
        if seen[index] == 0 {
            seen[index] = 1;
        }
    }

    for item in line2.chars() {
        let index = priority(item) - 1;
        if seen[index] == 1 {
            seen[index] = 2;
        }
    }

    for item in line3.chars() {
        let index = priority(item) - 1;
        if seen[index] == 2 {
            return item;
        }
    }

    panic!("No overlap found.")
}

enum State {
    One,
    Two,
    Three,
}
fn main() {
    let mut sum_1: usize = 0;
    let mut sum_2: usize = 0;
    let mut state: State = State::One;
    let mut line_1: String = Default::default();
    let mut line_2: String = Default::default();

    for line in stdin().lines().flatten() {
        sum_1 += priority(find_overlap_1(&line));
        match state {
            State::One => {
                line_1 = line;
                state = State::Two;
            }
            State::Two => {
                line_2 = line;
                state = State::Three;
            }
            State::Three => {
                sum_2 += priority(find_overlap_2(&line_1, &line_2, &line));
                state = State::One;
            }
        }
    }

    println!();
    println!("Part 1: {}", sum_1);
    println!("Part 2: {}", sum_2);
}
