use std::io::stdin;

fn visible(grid: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();
    let a = grid[y][x];

    let mut visible = true;
    for y2 in 0..y {
        if a <= grid[y2][x] {
            visible = false;
            break;
        }
    }
    if visible {
        return true;
    }

    visible = true;
    for y2 in y + 1..rows {
        if a <= grid[y2][x] {
            visible = false;
            break;
        }
    }
    if visible {
        return true;
    }

    visible = true;
    for x2 in 0..x {
        if a <= grid[y][x2] {
            visible = false;
            break;
        }
    }
    if visible {
        return true;
    }

    visible = true;
    for x2 in x + 1..cols {
        if a <= grid[y][x2] {
            visible = false;
            break;
        }
    }
    if visible {
        return true;
    }

    visible
}

fn scenic_score(grid: &Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    let a = grid[y][x];

    let mut score_up = 0;

    // eprintln!("Evaluating [{},{}]={}",x,y,a);

    if y > 0 {
        for y2 in (0..y).rev() {
            score_up += 1;
            // eprintln!("Up 1");
            if a <= grid[y2][x] {
                break;
            }
        }
    }

    let mut score_down = 0;

    for y2 in y + 1..grid.len() {
        score_down += 1;
        if a <= grid[y2][x] {
            break;
        }
    }

    let mut score_left = 0;

    if x > 0 {
        for x2 in (0..x).rev() {
            score_left += 1;
            if a <= grid[y][x2] {
                break;
            }
        }
    }

    let mut score_right = 0;
    for x2 in x + 1..grid[0].len() {
        score_right += 1;
        if a <= grid[y][x2] {
            break;
        }
    }

    score_up * score_down * score_left * score_right
}

fn main() {
    let mut grid: Vec<Vec<u8>> = vec![];
    for line in stdin().lines().flatten() {
        let new_row: Vec<u8> = line.bytes().map(|c| c).collect();
        grid.push(new_row);
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut part1 = 0;
    let mut part2: usize = 0;

    for y in 0..rows {
        for x in 0..cols {
            if visible(&grid, x, y) {
                part1 += 1;
            }

            let part2_score = scenic_score(&grid, x, y);
            if part2_score > part2 {
                part2 = part2_score;
            }
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
