use itertools::Either;
use nom::bytes::complete::tag;
use nom::character::complete::{newline, usize};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use nom::Parser;

type PathPoint = (usize, usize);

fn rock_structures(input: &str) -> IResult<&str, Vec<Vec<PathPoint>>> {
  separated_list1(
    newline,
    separated_list1(tag(" -> "), separated_pair(usize, tag(","), usize)),
  )
  .parse(input)
}

fn fill_structure(structure: &Vec<PathPoint>) -> Vec<PathPoint> {
  structure
    .windows(2)
    .flat_map(|w| {
      let ((a_x, a_y), (b_x, b_y)) = (w[0], w[1]);
      if a_x == b_x {
        let (start, end) = if a_y <= b_y { (a_y, b_y) } else { (b_y, a_y) };
        Either::Left((start..=end).map(move |y| (a_x, y)))
      } else {
        let (start, end) = if a_x <= b_x { (a_x, b_x) } else { (b_x, a_x) };
        Either::Right((start..=end).map(move |x| (x, a_y)))
      }
    })
    .collect()
}

fn make_grid(input: &str, with_bottom: bool) -> Vec<Vec<bool>> {
  let (_, structures) = rock_structures(input).unwrap();
  let max_x = structures
    .iter()
    .flat_map(|l| l.iter().map(|point| point.0))
    .max()
    .expect("no max x?");
  let max_y = structures
    .iter()
    .flat_map(|l| l.iter().map(|point| point.1))
    .max()
    .expect("no max y?");
  let mut grid = vec![vec![false; max_y + 1]; max_x + 1];
  if with_bottom {
    grid = vec![vec![false; max_y + 1 + 2]; (max_x + 1) * 2];
    for i in 0..(max_x + 1) * 2 {
      grid[i][max_y + 2] = true;
    }
  }
  structures
    .iter()
    .flat_map(|s| fill_structure(s))
    .for_each(|(x, y)| grid[x][y] = true);
  return grid;
}

fn simulate_bottomless(grid: &mut Vec<Vec<bool>>, starting_point: PathPoint) -> usize {
  let max_y = grid[0].len();
  let mut done = false;
  let mut resting = 0;
  while !done {
    let mut current_point = starting_point;
    let mut move_possible = true;
    while move_possible {
      let (x, y) = current_point;
      if y + 1 >= max_y {
        done = true;
        break;
      }
      if !grid[x][y + 1] {
        current_point = (x, y + 1);
      } else {
        if x > 0 && !grid[x - 1][y + 1] {
          current_point = (x - 1, y + 1);
        } else if x + 1 < grid.len() && !grid[x + 1][y + 1] {
          current_point = (x + 1, y + 1);
        } else {
          grid[current_point.0][current_point.1] = true;
          move_possible = false;
          resting += 1;
        }
      }
    }
  }
  return resting;
}

fn simulate_with_bottom(grid: &mut Vec<Vec<bool>>, starting_point: PathPoint) -> usize {
  let mut done = false;
  let mut resting = 0;
  while !done {
    let mut current_point = starting_point;
    let mut move_possible = true;
    while move_possible {
      let (x, y) = current_point;
      if !grid[x][y + 1] {
        current_point = (x, y + 1);
      } else {
        if x > 0 && !grid[x - 1][y + 1] {
          current_point = (x - 1, y + 1);
        } else if x + 1 < grid.len() && !grid[x + 1][y + 1] {
          current_point = (x + 1, y + 1);
        } else {
          grid[current_point.0][current_point.1] = true;
          move_possible = false;
          resting += 1;
        }
      }
    }
    done = grid[starting_point.0][starting_point.1];
  }
  return resting;
}

fn part_one(input: &str) -> usize {
  let mut grid = make_grid(input, false);
  let result = simulate_bottomless(&mut grid, (500, 0));
  return result;
}

fn part_two(input: &str) -> usize {
  let mut grid = make_grid(input, true);
  let result = simulate_with_bottom(&mut grid, (500, 0));
  return result;
}

fn main() {
  let input = include_str!("input");
  println!("{}", part_one(input));
  println!();
  println!("{}", part_two(input));
}

#[cfg(test)]
mod test {
  use super::*;
  const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

  #[test]
  fn part_one_example() {
    assert_eq!(part_one(INPUT), 24);
  }

  #[test]
  fn part_two_example() {
    assert_eq!(part_two(INPUT), 93);
  }
}
