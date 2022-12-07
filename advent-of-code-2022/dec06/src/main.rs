use std::{collections::HashSet, io::stdin};

fn find_unique(line: &String, window: usize) {
    let mut offset: usize = 0;

    while let Some(substr) = line.get(offset..offset + window) {
        let mut hash_set = HashSet::<char>::new();
        for character in substr.chars() {
            hash_set.insert(character);
        }

        if hash_set.len() == window {
            println!("Unique sequence of {} at {}", window, offset + 4);
            break;
        }
        offset += 1;
    }
}

fn main() {
    for line in stdin().lines().flatten() {
        find_unique(&line, 4); // Part 1
        find_unique(&line, 14); // Part 2
    }
}
