use std::cmp::Ordering;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{newline, u32};
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{delimited, pair, separated_pair};
use nom::Parser;
use nom::{combinator::map, IResult};

#[derive(Debug, Clone, PartialEq)]
enum List {
  Value(u32),
  Nested(Vec<List>),
}

impl Eq for List {}

impl PartialOrd for List {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    return Some(self.cmp(&other));
  }
}

impl Ord for List {
  fn cmp(&self, other: &Self) -> Ordering {
    match (self, other) {
      (List::Value(a), List::Value(b)) => a.cmp(b),
      (List::Value(a), List::Nested(_)) => List::Nested(vec![List::Value(*a)]).cmp(other),
      (List::Nested(_), List::Value(b)) => self.cmp(&List::Nested(vec![List::Value(*b)])),
      (List::Nested(a), List::Nested(b)) => a.cmp(b),
    }
  }
}

fn list(input: &str) -> IResult<&str, List> {
  alt((
    map(
      delimited(tag("["), separated_list0(tag(","), list), tag("]")),
      |l| List::Nested(l),
    ),
    map(u32, |n| List::Value(n)),
  ))
  .parse(input)
}

fn parse_list(input: &str) -> List {
  let (_, list) = list(input).unwrap();
  return list;
}

fn parse_pairs(input: &str) -> IResult<&str, Vec<(List, List)>> {
  separated_list1(pair(newline, newline), separated_pair(list, newline, list)).parse(input)
}

fn part_one(input: &str) -> usize {
  let (_, pairs) = parse_pairs(input).unwrap();
  pairs
    .iter()
    .enumerate()
    .filter_map(|(i, (l, r))| {
      match l.cmp(r) {
        Ordering::Less => Some(i + 1),
        _ => None,
      }
    })
    .sum()
}

fn part_two(input: &str) -> usize {
  let (_, pairs) = parse_pairs(input).unwrap();
  let divider_packet_a = parse_list("[[2]]");
  let divider_packet_b = parse_list("[[6]]");
  let mut lists: Vec<&List> = pairs
    .iter()
    .flat_map(|(l, r)| [l, r])
    .chain([&divider_packet_a, &divider_packet_b])
    .collect();
  lists.sort();
  lists
    .iter()
    .enumerate()
    .filter_map(|(i, list)| {
      if **list == divider_packet_a || **list == divider_packet_b {
        return Some(i + 1)
      }
      None
    })
    .product()
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
  const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

  #[test]
  fn part_one_example() {
    assert_eq!(part_one(INPUT), 13);
  }

  #[test]
  fn list_comp() {
    let left = List::Nested(vec![
      List::Value(1),
      List::Value(1),
      List::Value(3),
      List::Value(1),
      List::Value(1),
    ]);
    let right = List::Nested(vec![
      List::Value(1),
      List::Value(1),
      List::Value(5),
      List::Value(1),
      List::Value(1),
    ]);

    assert_eq!(left < right, true);

    let left = parse_list("[1,[2,[3,[4,[5,6,7]]]],8,9]");
    let right = parse_list("[1,[2,[3,[4,[5,6,0]]]],8,9]");

    assert_eq!(left < right, false);
  }

  #[test]
  fn list_parser() {
    let input = "[1,1,3,1,1]";
    let expected_output = List::Nested(vec![
      List::Value(1),
      List::Value(1),
      List::Value(3),
      List::Value(1),
      List::Value(1),
    ]);
    let output = parse_list(input);
    assert_eq!(output, expected_output);

    let input = "[1,[2,[3,[4,[5,6,7]]]],8,9]";
    let expected_output = List::Nested(vec![
      List::Value(1),
      List::Nested(vec![
        List::Value(2),
        List::Nested(vec![
          List::Value(3),
          List::Nested(vec![
            List::Value(4),
            List::Nested(vec![List::Value(5), List::Value(6), List::Value(7)]),
          ]),
        ]),
      ]),
      List::Value(8),
      List::Value(9),
    ]);

    let output = parse_list(input);
    assert_eq!(output, expected_output);
  }

  #[test]
  fn part_two_example() {
    assert_eq!(part_two(INPUT), 140);
  }

  #[test]
  fn what() {
    let v = vec![1, 1, 3, 1];
    let w = vec![1, 1, 5, 1, 1];
    dbg!(v.cmp(&w));
    assert_eq!(v < w, true);
  }
}
