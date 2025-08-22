use nom::{character::complete::newline, multi::separated_list1, IResult, Parser};
use std::time::Instant;

fn sequence(input: &str) -> IResult<&str, Vec<i64>> {
  separated_list1(newline, nom::character::complete::i64).parse(input)
}

fn wrap_index(i: i64, len: usize) -> usize {
  return i.rem_euclid(len as i64) as usize; // wrap index around length
}

fn decode_signal(numbers: &Vec<i64>, n_mixes: usize, key: i64) -> i64 {
  let mut positions: Vec<usize> = Vec::from_iter(0..numbers.len());
  for _ in 0..n_mixes {
    for index in 0..numbers.len() {
      let cur_pos = positions
        .iter()
        .position(|&e| e == index)
        .expect("index not in positions array");
      positions.remove(cur_pos);
      let new_pos = wrap_index(cur_pos as i64 + numbers[index] * key, positions.len());
      positions.insert(new_pos, index);
    }
  }
  let reordered: Vec<i64> = positions.iter().map(|i| numbers[*i] * key).collect();
  let zero_pos = reordered
    .iter()
    .position(|&element| element == 0)
    .expect("0 not in array");
  reordered
    .iter()
    .cycle()
    .skip(zero_pos + 1000)
    .step_by(1000)
    .take(3)
    .sum()
}

fn part_one(input: &str) -> i64 {
  let (_, numbers) = sequence(input).expect("parsing error");
  decode_signal(&numbers, 1, 1)
}

fn part_two(input: &str) -> i64 {
  let (_, numbers) = sequence(input).expect("parsing error");
  decode_signal(&numbers, 10, 811589153)
}

fn main() {
  let input = include_str!("input");
  let start = Instant::now();
  println!("{}", part_one(input));
  println!("time: {:?}", start.elapsed());

  let start = Instant::now();
  println!("{}", part_two(input));
  println!("time: {:?}", start.elapsed());
}

#[cfg(test)]
mod test {
  use super::*;
  const INPUT: &str = "1
2
-3
3
-2
0
4";

  #[test]
  fn part_one_example() {
    assert_eq!(part_one(INPUT), 3);
  }

  #[test]
  fn part_two_example() {
    assert_eq!(part_two(INPUT), 1623178306);
  }
}
