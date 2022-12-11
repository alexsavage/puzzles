use std::{collections::VecDeque, io::stdin};

#[derive(Debug, Clone, Copy)]
enum Operator {
    Addition,
    Multiplication,
}

#[derive(Debug, Clone, Copy)]
struct Operation {
    operator: Operator,
    constant: Option<u32>,
}

#[derive(Debug)]
struct MonkeyDescription {
    operation: Operation,
    target_divisor: u32,
    target_true: usize,
    target_false: usize,
}

impl MonkeyDescription {
    fn new(
        operation: Operation,
        target_divisor: u32,
        target_true: usize,
        target_false: usize,
    ) -> Self {
        Self {
            operation,
            target_divisor,
            target_true,
            target_false,
        }
    }
}

fn main() {
    let mut descriptions: Vec<MonkeyDescription> = vec![];
    let mut inspections: Vec<usize> = vec![];

    let mut line_number = 0;

    let mut items: Vec<VecDeque<u32>> = vec![];
    let mut operation: Operation = Operation {
        operator: Operator::Addition,
        constant: None,
    };
    let mut target_divisor = u32::default();
    let mut target_true = usize::default();

    for line in stdin().lines().flatten() {
        line_number += 1;

        if line_number == 2 {
            items.push(line[18..].split(", ").map(|c| c.parse().unwrap()).collect());
        } else if line_number == 3 {
            let operation_string = &mut line[23..].split(' ');
            operation = match operation_string.next() {
                Some("+") => Operation {
                    operator: Operator::Addition,
                    constant: Some(operation_string.next().unwrap().parse::<u32>().unwrap()),
                },
                Some("*") => Operation {
                    operator: Operator::Multiplication,
                    constant: if let Ok(constant) = operation_string.next().unwrap().parse() {
                        Some(constant)
                    } else {
                        None // square
                    },
                },
                Some(x) => {
                    eprintln!("Bad operator: '{}'", x);
                    panic!()
                }
                None => panic!(),
            };
        } else if line_number == 4 {
            target_divisor = line[21..].parse().unwrap();
        } else if line_number == 5 {
            target_true = line[29..].parse().unwrap();
        } else if line_number == 6 {
            let target_false = line[30..].parse().unwrap();
            descriptions.push(MonkeyDescription::new(
                operation,
                target_divisor,
                target_true,
                target_false,
            ));
            inspections.push(0);
        } else if line_number == 7 {
            line_number = 0;
        }
    }

    for _round in 0..20 {
        for i in 0..descriptions.len() {
            // dbg!(i);
            let monkey = &descriptions[i];
            // dbg!(monkey);
            // dbg!(&items[i]);
            while let Some(old) = items[i].pop_front() {
                // dbg!(old);
                let new = match monkey.operation {
                    Operation {
                        operator: Operator::Addition,
                        constant: Some(x),
                    } => old + x,
                    Operation {
                        operator: Operator::Multiplication,
                        constant: Some(x),
                    } => old * x,
                    Operation {
                        operator: Operator::Multiplication,
                        constant: None,
                    } => old.checked_mul(old).unwrap(),
                    _ => panic!(),
                } / 3;
                // dbg!(&items[i]);
                // dbg!(new);
                let j = match new % monkey.target_divisor {
                    0 => monkey.target_true,
                    _ => monkey.target_false,
                };
                inspections[i] += 1;
                items[j].push_back(new);
                // dbg!(j);
                // dbg!(&items[j]);
            }
        }
    }

    inspections.sort_unstable_by(|a, b| b.cmp(a));
    println!("Part 1: {}", inspections[0] * inspections[1]);
}
