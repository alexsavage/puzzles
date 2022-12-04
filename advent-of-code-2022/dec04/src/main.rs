use regex::Regex;
use std::io::stdin;

struct Plot {
    start: u32,
    end: u32,
}

impl Plot {
    fn contains(&self, other: &Plot) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Plot) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}

fn main() {
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    let mut count1: u32 = 0;
    let mut count2: u32 = 0;

    for line in stdin().lines().flatten() {
        let captures = re.captures(&line).unwrap();
        let a = Plot {
            start: captures[1].parse::<u32>().unwrap(),
            end: captures[2].parse::<u32>().unwrap(),
        };
        let b = Plot {
            start: captures[3].parse::<u32>().unwrap(),
            end: captures[4].parse::<u32>().unwrap(),
        };

        if a.contains(&b) || b.contains(&a) {
            count1 += 1;
        }

        if a.overlaps(&b) {
            count2 += 1;
        }
    }

    println!("Part 1: {}", count1);
    println!("Part 2: {}", count2);
}
