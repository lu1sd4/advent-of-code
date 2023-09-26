fn main() {
    let file_contents = include_str!("input");
    part_one(file_contents);
    println!();
    part_two(file_contents);
}

fn parse_stack_line(line: &str) -> Vec<char> {
    let mut line_iter = line.chars().peekable();
    let mut result: Vec<char> = Vec::new();
    while line_iter.peek().is_some() {
        let sub: Vec<char> = line_iter.by_ref().take(4).collect();
        result.push(sub[1]);
    }
    return result;
}

fn parse_stacks(file_contents_lines: &mut std::str::Lines) -> Vec<Vec<char>> {
    let mut end_stacks: bool = false;
    let mut crates_hor: Vec<Vec<char>> = Vec::new();
    while !end_stacks {
        let line = file_contents_lines.next().unwrap();
        if line.eq("") {
            end_stacks = true;
        } else {
            let parsed_line = parse_stack_line(line);
            crates_hor.push(parsed_line);
        }
    }
    crates_hor.pop();
    let crates_hor_iter = crates_hor.iter().rev();
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); crates_hor[0].len()];
    for level in crates_hor_iter {
        for (index, crate_item) in level.iter().enumerate() {
            if *crate_item != ' ' {
                stacks[index].push(*crate_item);
            }
        }
    }
    return stacks;
}

fn parse_instruction_line(line: &str) -> (usize, usize, usize) {
    let split: Vec<&str> = line.split(' ').collect();
    return (split[1].parse::<usize>().unwrap(), split[3].parse::<usize>().unwrap(), split[5].parse::<usize>().unwrap());
}

fn execute_instruction_one(stacks: &mut Vec<Vec<char>>, (n_crates, from, to): &(usize, usize, usize)) {
    for _ in 0..*n_crates {
        let el = stacks[*from - 1].pop().unwrap();
        stacks[*to - 1].push(el);
    }
}

fn execute_instruction_two(stacks: &mut Vec<Vec<char>>, (n_crates, from, to): &(usize, usize, usize)) {
    let mut crane: Vec<char> = Vec::new();
    for _ in 0..*n_crates {
        crane.push(stacks[*from - 1].pop().unwrap());
    }
    for _ in 0..*n_crates {
        stacks[*to - 1].push(crane.pop().unwrap());
    }
}

fn solve_with(file_contents: &str, f: &dyn Fn(&mut Vec<Vec<char>>, &(usize, usize, usize)) -> ()) {
    let mut file_contents_iter = file_contents.lines();
    let mut stacks = parse_stacks(file_contents_iter.by_ref());
    for instruction_line in file_contents_iter {
        let instruction = parse_instruction_line(instruction_line);
        f(&mut stacks, &instruction);
    }
    for mut stack in stacks {
        print!("{}", stack.pop().unwrap());
    }
    println!();   
}

fn part_one(file_contents: &str) {
    solve_with(file_contents, &execute_instruction_one);
}

fn part_two(file_contents: &str) {
    solve_with(file_contents, &execute_instruction_two)
}
