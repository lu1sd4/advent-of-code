use std::cmp::max;
use std::collections::BTreeSet;

fn main() {
    let file_contents = include_str!("input");
    part_one(file_contents); 
    println!();
    part_two(file_contents);
}

fn part_one(file_contents: &str) {
    let mut max_cals: u32 = 0;
    let mut current_cals: u32 = 0;
    for line in file_contents.lines() {
        if line.is_empty() {
            max_cals = max(max_cals, current_cals);
            current_cals = 0;
        } else {
            current_cals += line.parse::<u32>().unwrap();
        }
    }
    println!("{max_cals}");
}

fn part_two(file_contents: &str) {
    let mut calories = BTreeSet::new();
    let mut current_cals: u32 = 0;
    for line in file_contents.lines() {
        if line.is_empty() {
            calories.insert(current_cals);
            current_cals = 0;
        } else {
            current_cals += line.parse::<u32>().unwrap();
        }
    }
    let total_top_three: u32 = calories.iter().rev().take(3).sum();
    println!("{total_top_three}");
}

