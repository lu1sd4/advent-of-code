use itertools::Itertools;
fn main() {
    let input = include_str!("input");
    println!("{}", part_one(input));
    println!();
    println!("{}", part_two(input));
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

fn parse_instruction(line: &str) -> Instruction {
    let mut split = line.split(" ");
    let _ = split.next().unwrap();
    match split.next() {
        Some(secs) => Instruction::Addx(secs.parse::<i32>().unwrap()),
        _ => Instruction::Noop,
    }
}

struct Cpu {
    register_value: i32,
    total_signal_strength: i32,
    current_cycle: i32,
    pixels: String,
}

impl Cpu {
    fn new() -> Self {
        Self {
            register_value: 1,
            total_signal_strength: 0,
            current_cycle: 0,
            pixels: "".to_string(),
        }
    }
    fn advance_clock(&mut self) {
        self.current_cycle += 1;
        if (self.register_value - (self.current_cycle - 1) % 40).abs() < 2 {
            self.pixels += "#";
        } else {
            self.pixels += ".";
        }
        if self.current_cycle == 20 || (self.current_cycle + 20) % 40 == 0 {
            self.total_signal_strength += self.current_cycle * self.register_value;
        }
    }
    fn update_register(&mut self, val: &i32) {
        self.register_value += val;
    }
    fn process_instruction(&mut self, instr: &Instruction) {
        match instr {
            Instruction::Noop => {
                self.advance_clock();
            }
            Instruction::Addx(val) => {
                self.advance_clock();
                self.advance_clock();
                self.update_register(val);
            }
        }
    }
    fn display_screen(&self) -> String {
        self.pixels
            .chars()
            .chunks(40)
            .into_iter()
            .map(|row| row.collect::<String>())
            .join("\n")
    }
}

fn simulate_program(input: &str) -> Cpu {
    let mut cpu = Cpu::new();
    for line in input.lines() {
        let current_instr = parse_instruction(line);
        cpu.process_instruction(&current_instr);
    }
    cpu
}

fn part_one(input: &str) -> i32 {
    simulate_program(input).total_signal_strength
}

fn part_two(input: &str) -> String {
    simulate_program(input).display_screen()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part_one_example() {
        assert_eq!(part_one(INPUT), 13140);
    }

    #[test]
    fn part_two_example() {
        assert_eq!(
            part_two(INPUT),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
