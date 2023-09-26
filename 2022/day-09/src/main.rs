use std::collections::HashSet;
use std::convert::TryInto;
use std::str::FromStr;

fn main() {
    let file_contents = include_str!("input");
    part_one(file_contents);
    println!();
    part_two(file_contents);
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Position {
    x: i32,
    y: i32,
}
impl Position {
    fn new() -> Self {
        Position { x: 0, y: 0 }
    }
}
type Instruction = (Direction, usize);

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "U" => Ok(Direction::Up),
            "R" => Ok(Direction::Right),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            _ => Err(()),
        }
    }
}

fn update_knot(knots: &mut [Position]) {
    if let [front_knot, back_knot] = knots {
        let dx = front_knot.x - back_knot.x;
        let dy = front_knot.y - back_knot.y;
        if dx.abs() > 1 {
            back_knot.x += dx / dx.abs();
            if dy.abs() > 1 {
                back_knot.y += dy / dy.abs();
            } else if dy.abs() > 0 {
                back_knot.y += dy;
            }
        } else if dy.abs() > 1 {
            back_knot.y += dy / dy.abs();
            if dx.abs() > 1 {
                back_knot.x += dx / dx.abs();
            } else if dx.abs() > 0 {
                back_knot.x += dx;
            }
        }
    }
}

fn print_rope(rope: &[Position], grid_len: usize) {
    let mut grid = vec![vec!['.'; grid_len]; grid_len];
    for (pos, knot) in rope.iter().enumerate() {
        grid[grid_len - 1 - knot.y as usize][knot.x as usize] =
            std::char::from_digit(pos.try_into().unwrap(), 10).unwrap();
    }
    for char_line in grid {
        for character in char_line {
            print!("{}", character);
        }
        println!();
    }
    println!("----------");
}

fn solve_for(rope_length: usize, file_contents: &str) {
    let mut visited_positions = HashSet::new();
    let mut rope = vec![Position::new(); rope_length];
    for line in file_contents.lines() {
        let mut split = line.split(" ");
        let instruction: Instruction = (
            Direction::from_str(split.next().unwrap()).unwrap(),
            split.next().unwrap().parse::<usize>().unwrap(),
        );
        for _ in 0..instruction.1 {
            match instruction.0 {
                Direction::Up => rope[0].y += 1,
                Direction::Right => rope[0].x += 1,
                Direction::Down => rope[0].y -= 1,
                Direction::Left => rope[0].x -= 1,
            }
            for front in 0..=(rope.len().saturating_sub(2)) {
                update_knot(&mut rope[front..front + 2]);
            }
            visited_positions.insert(rope.last().unwrap().clone());
        }
    }
    println!("{}", visited_positions.len());
}

fn part_one(file_contents: &str) {
    solve_for(2, file_contents);
}

fn part_two(file_contents: &str) {
    solve_for(10, file_contents);
}
