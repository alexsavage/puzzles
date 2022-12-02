use std::io::stdin;

struct Leaderboard {
    first: u32,
    second: u32,
    third: u32,
}

impl Leaderboard {
    fn track(&mut self, value: u32) {
        if self.first < value {
            self.third = self.second;
            self.second = self.first;
            self.first = value;
        } else if self.second < value {
            self.third = self.second;
            self.second = value;
        } else if self.third < value {
            self.third = value;
        }
    }
}

fn main() {
    let mut result = Leaderboard {
        first: 0,
        second: 0,
        third: 0,
    };

    let mut elf_sum = 0;

    for line in stdin().lines().flatten() {
        match line.as_str() {
            "" => {
                result.track(elf_sum);
                elf_sum = 0;
            }
            _ => {
                elf_sum += line.parse::<u32>().unwrap();
            }
        }
    }

    result.track(elf_sum);

    println!();
    println!("  Largest: {}", result.first);
    println!(
        "Top three: {}",
        result.first + result.second + result.third
    );
}
