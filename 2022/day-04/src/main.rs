fn main() {
    let file_contents = include_str!("input");
    part_one(file_contents);
    println!();
    part_two(file_contents);
}

fn parse_interval(interval: &str) -> Vec<u32> {
    return interval.split('-').map(|e| e.parse::<u32>().unwrap()).collect();
}

fn contains(a: &Vec<u32>, b: &Vec<u32>) -> bool {
    return a[0] <= b[0] && a[1] >= b[1];
}

fn part_one(file_contents: &str) {
    let mut count: u32= 0;
    for line in file_contents.lines() {
        let intervals: Vec<&str> = line.split(',').collect();
        let int_one: Vec<u32> = parse_interval(intervals[0]);
        let int_two: Vec<u32> = parse_interval(intervals[1]);
        if contains(&int_one, &int_two) || contains(&int_two, &int_one) {
            count += 1; 
        }
    }
    println!("{}", count);
}

fn overlap(a: &Vec<u32>, b: &Vec<u32>) -> bool {
    return a[1] >= b[0] && a[0] <= b[0];
}

fn part_two(file_contents: &str) {
    let mut count: u32= 0;
    for line in file_contents.lines() {
        let intervals: Vec<&str> = line.split(',').collect();
        let int_one: Vec<u32> = parse_interval(intervals[0]);
        let int_two: Vec<u32> = parse_interval(intervals[1]);
        if overlap(&int_one, &int_two) || overlap(&int_two, &int_one) {
            count += 1; 
        }
    }
    println!("{}", count);
}

