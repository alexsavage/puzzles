use sprs::TriMat;
use std::{cmp::Ordering, io::stdin};

#[derive(Clone, Copy)]
enum Move {
    Up,
    UpLeft,
    Left,
    DownLeft,
    Down,
    DownRight,
    Right,
    UpRight,
}

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn shift(&self, direction: Move) -> Point {
        match direction {
            Move::Up => Point {
                x: self.x,
                y: self.y - 1,
            },
            Move::Down => Point {
                x: self.x,
                y: self.y + 1,
            },
            Move::Left => Point {
                x: self.x - 1,
                y: self.y,
            },
            Move::Right => Point {
                x: self.x + 1,
                y: self.y,
            },
            Move::UpLeft => Point {
                x: self.x - 1,
                y: self.y - 1,
            },
            Move::DownLeft => Point {
                x: self.x - 1,
                y: self.y + 1,
            },
            Move::DownRight => Point {
                x: self.x + 1,
                y: self.y + 1,
            },
            Move::UpRight => Point {
                x: self.x + 1,
                y: self.y - 1,
            },
        }
    }

    fn chase(&self, other: &Point) -> Option<Move> {
        if self.x.abs_diff(other.x) < 2 && self.y.abs_diff(other.y) < 2 {
            None
        } else {
            match other.x.cmp(&self.x) {
                Ordering::Less => match other.y.cmp(&self.y) {
                    Ordering::Less => Some(Move::UpLeft),
                    Ordering::Equal => Some(Move::Left),
                    Ordering::Greater => Some(Move::DownLeft),
                },
                Ordering::Equal => match other.y.cmp(&self.y) {
                    Ordering::Less => Some(Move::Up),
                    Ordering::Equal => None,
                    Ordering::Greater => Some(Move::Down),
                },
                Ordering::Greater => match other.y.cmp(&self.y) {
                    Ordering::Less => Some(Move::UpRight),
                    Ordering::Equal => Some(Move::Right),
                    Ordering::Greater => Some(Move::DownRight),
                },
            }
        }
    }
}

fn main() {
    let mut mat = TriMat::new((usize::MAX, usize::MAX));
    let mut head = Point { x: usize::MAX/2, y: usize::MAX/2 };
    let mut tail: Point = Point { x: usize::MAX/2, y: usize::MAX/2 };
    mat.add_triplet(tail.x, tail.y, 1);
    for line in stdin().lines().flatten() {
        let dir = match line.chars().next().unwrap() {
            'U' => Move::Up,
            'D' => Move::Down,
            'L' => Move::Left,
            'R' => Move::Right,
            _ => panic!(),
        };

        let steps: u32 = line[2..].parse().unwrap();

        for _ in 0..steps {
            head = head.shift(dir);
            if let Some(chase) = tail.chase(&head) {
                tail = tail.shift(chase);
                if mat.find_locations(tail.y, tail.x).len() == 0 {
                    mat.add_triplet(tail.y, tail.x, 1);
                }
            }
        }
    }

    println!("Part 1: {}", mat.nnz());
}
