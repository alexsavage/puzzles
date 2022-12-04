use regex::Regex;
use std::{io::stdin, ops::RangeInclusive};

fn contains(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    if a.start() <= b.start() {
        a.end() >= b.end() // &&
    } else {
        b.end() >= a.end() // && b.start() < a.start()
    }
}

fn overlaps(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    a.start() <= b.end() && a.end() >= b.start()
}

fn main() {
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    let mut count1: u32 = 0;
    let mut count2: u32 = 0;

    for line in stdin().lines().flatten() {
        let captures = re.captures(&line).unwrap();
        let a: RangeInclusive<u32> = captures[1].parse().unwrap()..=captures[2].parse().unwrap();
        let b: RangeInclusive<u32> = captures[3].parse().unwrap()..=captures[4].parse().unwrap();

        if contains(&a, &b) || contains(&b, &a) {
            count1 += 1;
        }

        if overlaps(&a, &b) {
            count2 += 1;
        }
    }

    println!("Part 1: {}", count1);
    println!("Part 2: {}", count2);
}
