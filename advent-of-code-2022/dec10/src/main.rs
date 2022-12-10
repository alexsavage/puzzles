use regex::Regex;
use std::io::stdin;

struct Core {
    cycle: u32,
    x: i32,
    accum_1: i32,
    accum_2: i32,
}

enum Instruction {
    NoOp,
    AddX,
}

impl Core {
    fn new() -> Self {
        Self {
            cycle: 0,
            x: 1,
            accum_1: 0,
            accum_2: 0,
        }
    }
    fn start_cycle(&mut self) {
        self.cycle += 1;
        if self.x.abs_diff((self.cycle as i32 - 1) % 40) <=1 {
            print!("#");
        } else {
            print!(".");
        }

        if self.cycle % 40 == 0 {
            println!();
        }
        // eprintln!("Start cycle {}", self.cycle);
        if (self.cycle + 20) % 40 == 0 {
            // eprintln!("Cycle {}: Increment accum_1({}) by {} = {}", self.cycle, self.accum_1, self.x * self.cycle as i32, self.accum_1 + self.x * self.cycle as i32);
            self.accum_1 += self.x * self.cycle as i32;
        }
    }

    fn decode(&mut self, instruction: Instruction, operand: Option<i32>) {
        match instruction {
            Instruction::NoOp => {
                // eprintln!("Decode No-Op");
                self.start_cycle();
            }
            Instruction::AddX => {
                // eprintln!("Decode AddX: {}", operand.unwrap());
                self.start_cycle();
                self.start_cycle();
                match operand {
                    Some(value) => {
                        self.add_x(value);
                    }
                    None => {
                        panic!();
                    }
                }
            }
        }
    }

    fn add_x(&mut self, operand: i32) {
        // eprintln!("Add {} to X ({}) = {}", operand, self.x, self.x + operand);
        self.x += operand;
    }
}

fn main() {
    let asm_syntax = Regex::new(r"^(addx|noop) ?(-?\d+)?$").unwrap();
    let mut cpu = Core::new();

    for line in stdin().lines().flatten() {
        let capture = asm_syntax.captures(&line).unwrap();
        match &capture[1] {
            "noop" => {
                cpu.decode(Instruction::NoOp, None);
            }
            "addx" => {
                cpu.decode(Instruction::AddX, Some(capture[2].parse().unwrap()));
            }
            _ => {
                panic!();
            }
        }
    }

    println!("Part 1: {}", cpu.accum_1);
    println!("Part 2: {}", cpu.accum_2);
}
