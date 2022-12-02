fn main() {
    let file_contents = include_str!("input");
    part_one(file_contents);
    println!();
    part_two(file_contents);
}

fn choice_to_usize(choice: char) -> usize {
    let mut choice_val: u16 = choice as u16;
    let diff_x_a: u16 = 'X' as u16 - 'A' as u16;
    choice_val = choice_val - 'A' as u16;
    if choice_val > 3 {
        choice_val -= diff_x_a;
    }
    return choice_val as usize;
}

fn round_score_one(first: char, second: char) -> u16 {
    let mat:[[u16;3];3] = [[4, 1, 7], [8, 5, 2], [3, 9, 6]];
    return mat[choice_to_usize(second)][choice_to_usize(first)];
}

fn round_score_two(first: char, second: char) -> u16 {
    let mat: [[u16;3];3] = [[3, 4, 8], [1, 5, 9], [2, 6, 7]];
    return mat[choice_to_usize(first)][choice_to_usize(second)];   
}

fn part_one(file_contents: &str) {
    let mut total_score: u16 = 0;
    for line in file_contents.lines() {
        let mut chars = line.chars();
        let first = chars.next().unwrap();
        let second = chars.nth(1).unwrap();
        total_score += round_score_one(first, second);
    }
    println!("{total_score}")
}

fn part_two(file_contents: &str) {
    let mut total_score: u16 = 0;
    for line in file_contents.lines() {
        let mut chars = line.chars();
        let first = chars.next().unwrap();
        let second = chars.nth(1).unwrap();
        total_score += round_score_two(first, second);
    }
    println!("{total_score}")
}
