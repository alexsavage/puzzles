use std::io::stdin;

fn priority(item: char) -> usize {
    match item {
        'a'..='z' => item as usize - 96,
        'A'..='Z' => item as usize - 64 + 26,
        _ => panic!(),
    }
}

fn find_overlap(line: &str) -> char {
    let mut seen: [bool; 52] = [false; 52];
    let chars = line.chars();
    let count = chars.count();

    for (index, item) in line.char_indices() {
        if index < count/2 {
            seen[priority(item)-1] = true;
        } else {
            if seen[priority(item)-1] {
                return item;
            }
        }
    }
    panic!("No overlap found.")
}

fn main() {
    let mut sum: usize = 0;
    for line in stdin().lines().flatten() {
        sum += priority(find_overlap(line.as_str()));
    }

    println!("Part 1: {}", sum);
}
