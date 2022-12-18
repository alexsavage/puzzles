use std::{collections::VecDeque, io::stdin};

fn main() {
    let mut x = usize::default();
    let mut y = usize::default();
    let mut height = Vec::new();
    let mut distance = Vec::new();
    let mut path: Vec<Vec<Option<(usize, usize)>>> = Vec::new();
    let mut queue = VecDeque::new();
    let mut end = (0, 0);

    for line in stdin().lines().flatten() {
        height.push(vec![]);
        distance.push(vec![]);
        path.push(vec![]);
        x = 0;
        for c in line.chars() {
            height[y].push(match c {
                'S' => 0,
                'E' => 25,
                c => (c as u8) - 97,
            });
            distance[y].push(match c {
                'S' => 0,
                _ => usize::MAX,
            });

            path[y].push(None);

            if c == 'S' {
                queue.push_back((x, y));
            }
            if c == 'E' {
                end = (x, y);
            }
            x += 1;
        }
        y += 1;
    }

    while let Some(current_point) = queue.pop_front() {
        // eprintln!("({},{})", current_point.0, current_point.1);
        let current_distance = distance[current_point.1][current_point.0] + 1;
        for next_point in neighbors(current_point, y, x) {
            let next_distance = distance[next_point.1][next_point.0];
            if can_move(current_point, next_point, &height) {
                if next_distance > current_distance {
                    distance[next_point.1][next_point.0] = current_distance;
                    path[next_point.1][next_point.0] = Some(current_point);
                    queue.push_front(next_point);
                }
            }
        }
    }

    println!("Part 1: {}", distance[end.1][end.0]);
}

fn neighbors(point: (usize, usize), max_y: usize, max_x: usize) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];
    if point.1 > 0 {
        neighbors.push((point.0, point.1 - 1));
    }
    if point.1 + 1 < max_y {
        neighbors.push((point.0, point.1 + 1));
    }
    if point.0 > 0 {
        neighbors.push((point.0 - 1, point.1));
    }
    if point.0 + 1 < max_x {
        neighbors.push((point.0 + 1, point.1));
    }
    neighbors
}

fn can_move(a: (usize, usize), b: (usize, usize), heights: &Vec<Vec<u8>>) -> bool {
    // eprintln!(
    //     "Checking from ({},{}) height {}, to ({},{}) height {}",
    //     a.0, a.1, heights[a.1][a.0], b.0, b.1, heights[b.0][b.1],
    // );
    let from_height = heights[a.1][a.0];
    let to_height = heights[b.1][b.0];

    match to_height.checked_sub(from_height) {
        Some(climb) => {
            if climb <= 1 {
                true
            } else {
                false
            }
        }
        None => true,
    }
}
