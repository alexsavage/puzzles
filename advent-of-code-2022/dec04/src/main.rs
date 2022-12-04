use regex::Regex;
use std::io::stdin;

struct Plot {
    start: u32,
    end: u32,
}

impl Plot {
    fn len(&self) -> u32 {
        self.end - self.start
    }

    fn contains(&self, other: &Plot) -> bool {
        self.len() >= other.len() && self.start <= other.start && self.end >= other.end
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

        if a.overlaps(&b) {
            count2 += 1;
        }

        let (lhs, rhs) = match a.len().cmp(&b.len()) {
            std::cmp::Ordering::Less => (b, a),
            std::cmp::Ordering::Equal => (b, a),
            std::cmp::Ordering::Greater => (a, b),
        };

        if lhs.contains(&rhs) {
            count1 += 1;
        }
    }

    println!("Part 1: {}", count1);
    println!("Part 2: {}", count2);
}
