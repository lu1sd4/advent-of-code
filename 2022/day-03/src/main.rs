fn main() {
    let file_contents = include_str!("input");
    part_one(file_contents);
    println!();
    part_two(file_contents);
}

fn item_priority(item_letter: char) -> usize {
    if item_letter.is_ascii_lowercase() {
        return item_letter as usize - 'a' as usize + 1;
    } else {
        return item_letter as usize - 'A' as usize + 27;
    }
}

const ALPHA_LENGTH: usize = 26 * 2;

fn part_one(file_contents: &str) {
    let mut total_sum: u32 = 0;
    for line in file_contents.lines() {
        let len_sack = line.chars().count() / 2;
        let mut sack_one: [bool; ALPHA_LENGTH] = [false; ALPHA_LENGTH];
        let mut items_iter = line.chars();
        for item in items_iter.by_ref().take(len_sack) {
            sack_one[item_priority(item) - 1] = true;
        }
        for item in items_iter.take(len_sack) {
            let current_priority = item_priority(item);
            if sack_one[current_priority - 1] {
                total_sum += current_priority as u32;
                break;
            }
        }
    }
    println!("{total_sum}");
}

fn process_sack(sack: &str) -> [bool; ALPHA_LENGTH] {
    let mut processed_sack: [bool; ALPHA_LENGTH] = [false; ALPHA_LENGTH];
    for item in sack.chars() {
       processed_sack[item_priority(item) - 1] = true;
    }
    return processed_sack;
}

fn part_two(file_contents: &str) {
    let mut total_sum: u32 = 0;
    let mut lines = file_contents.lines().peekable();
    while lines.peek().is_some() {
        let sack_one = process_sack(lines.by_ref().next().unwrap());
        let sack_two = process_sack(lines.by_ref().next().unwrap());
        let sack_three = process_sack(lines.by_ref().next().unwrap());
        for i in 0..ALPHA_LENGTH {
            if sack_one[i] && sack_two[i] && sack_three[i] {
                total_sum += i as u32 + 1;
            }
        }
    }
    println!("{total_sum}");
}
