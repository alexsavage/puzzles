use regex::Regex;
use std::{collections::HashMap, io::stdin};

struct MemoryDir {
    file_sizes: HashMap<String, u32>,
    subdirs: HashMap<String, MemoryDir>,
}

impl MemoryDir {
    fn new() -> Self {
        Self {
            file_sizes: HashMap::new(),
            subdirs: HashMap::new(),
        }
    }

    fn create_subdir(&mut self, name: &str) {
        let new_dir: MemoryDir = MemoryDir::new();
        self.subdirs.insert(String::from(name), new_dir);
    }

    fn create_file(&mut self, name: &str, size: u32) {
        self.file_sizes.insert(String::from(name), size);
    }

    fn size_threshold(
        &self,
        max_size: u32,
        running_total: &mut u32,
    ) -> u32 {
        let my_size = self.file_sizes.values().sum::<u32>()
            + self
                .subdirs
                .values()
                .map(|d| d.size_threshold(max_size,  running_total))
                .sum::<u32>();
        if my_size <= max_size {
            *running_total += my_size;
        }

        my_size
    }

    fn best_target(
        &self,
        min_size: u32,
        best_size: &mut u32,
    ) -> u32 {
        let my_size = self.file_sizes.values().sum::<u32>()
            + self
                .subdirs
                .values()
                .map(|d| d.best_target(min_size, best_size))
                .sum::<u32>();
        eprintln!("my_size = {}, min_size = {}, smallest_dir = {}", my_size, min_size, best_size);
        if (my_size <= *best_size) && (my_size >= min_size) {
            eprintln!("Selecting {}", my_size);
            *best_size = my_size;
        }

        my_size
    }
}

fn main() {
    let cmd_cd = Regex::new(r"^\$ cd (.+)$").unwrap();
    let cmd_ls = "$ ls";
    let ls_dir = Regex::new(r"^dir (.+)$").unwrap();
    let ls_file = Regex::new(r"^(\d+) (.+)$").unwrap();

    let mut root = MemoryDir::new();
    let mut current_directory = &mut root;
    let mut current_path: Vec<String> = vec![];

    for line in stdin().lines().flatten() {
        if line == cmd_ls { // Nothing
        } else if let Some(captures) = cmd_cd.captures(&line) {
            match &captures[1] {
                "/" => {
                    current_path = vec![];
                }
                ".." => {
                    current_path.pop();
                }
                dir => {
                    current_path.push(dir.to_string());
                }
            }

            current_directory = &mut root;
            for dir in current_path.iter() {
                current_directory = current_directory.subdirs.get_mut(dir).unwrap();
            }
        } else if let Some(captures) = ls_dir.captures(&line) {
            let dir_name = &captures[1];
            if let None = current_directory.subdirs.get(dir_name) {
                current_directory.create_subdir(dir_name);
            }
        } else if let Some(captures) = ls_file.captures(&line) {
            let file_name = &captures[2];
            current_directory.create_file(file_name, captures[1].parse().unwrap());
        } else {
            break;
        }
    }

    let mut part1: u32 = 0;
    for dir in root.subdirs.values() {
        dir.size_threshold(100000, &mut part1);
    }
    
    let mut part2: u32 = 0;
    let total_size = root.size_threshold(0, &mut part2);
    let part2_target = 30000000 - (70000000 - total_size);
    part2 = total_size;

    _ = root.best_target(part2_target, &mut part2);
    
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
