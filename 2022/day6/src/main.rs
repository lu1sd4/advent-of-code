use std::collections::HashSet;


fn main() {
    let file_contents = include_str!("input");
    part_one(file_contents);
    println!();
    part_two(file_contents);
}

fn is_marker(sequence: &[char]) -> bool {
    let set: HashSet<char> = HashSet::from_iter(sequence.iter().cloned());
    return set.len() == sequence.len();
}

fn solve_with_window_size(file_contents: &str, window_size: &usize) {
    for line in file_contents.lines() {
        let message: Vec<char> = line.chars().collect();
        let mut i_left: usize = 0;
        let mut i_right: usize = *window_size;
        let mut window = &message[i_left .. i_right];
        while !is_marker(window) {
            i_left += 1;
            i_right += 1;
            window = &message[i_left .. i_right];
        }
        println!("{i_right}");
    }
}

fn part_one(file_contents: &str) {
    solve_with_window_size(file_contents, &4);
}

fn part_two(file_contents: &str) {
    solve_with_window_size(file_contents, &14);
}
