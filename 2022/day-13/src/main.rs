use std::cmp::Ordering;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{delimited, pair, separated_pair};
use nom::Parser;
use nom::{combinator::map, multi::many1, IResult};

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
      (List::Value(self_value), List::Value(other_value)) => self_value.cmp(other_value),
      (List::Value(self_value), List::Nested(_)) => {
        List::Nested(vec![List::Value(*self_value)]).cmp(other)
      }
      (List::Nested(_), List::Value(other_value)) => {
        self.cmp(&List::Nested(vec![List::Value(*other_value)]))
      }
      (List::Nested(self_list), List::Nested(other_list)) => {
        let mut self_index = 0;
        let mut other_index = 0;
        while self_index < self_list.len() && other_index < other_list.len() {
          match self_list[self_index].cmp(&other_list[other_index]) {
            Ordering::Equal => {
              self_index += 1;
              other_index += 1;
            }
            cmp_result => return cmp_result,
          }
        }
        self_list.len().cmp(&other_list.len())
      }
    }
  }
}

fn parse_number_element(input: &str) -> IResult<&str, List> {
  map(digit1, |s: &str| List::Value(s.parse::<u32>().unwrap())).parse(input)
}

fn parse_element(input: &str) -> IResult<&str, List> {
  alt((parse_number_element, parse_list_element)).parse(input)
}

fn parse_list_element(input: &str) -> IResult<&str, List> {
  map(
    delimited(tag("["), separated_list0(tag(","), parse_element), tag("]")),
    |l: Vec<List>| List::Nested(l),
  )
  .parse(input)
}

fn parse_list(input: &str) -> List {
  let (_, list) = parse_list_element(input).unwrap();
  return list;
}

fn parse_pairs(input: &str) -> Vec<(List, List)> {
  let (_, result) = separated_list1(
    pair(newline, newline),
    separated_pair(parse_list_element, newline, parse_list_element),
  )
  .parse(input)
  .unwrap();
  return result;
}

fn parse_all_lists(input: &str) -> Vec<List> {
  let (_, result) = separated_list1(many1(newline), parse_list_element)
    .parse(input)
    .unwrap();
  return result;
}

fn part_one(input: &str) -> u32 {
  let pairs = parse_pairs(input);
  pairs
    .iter()
    .enumerate()
    .map(|(i, pair)| (i + 1, pair))
    .filter(|(_, (l, r))| l < r)
    .map(|(i, _)| u32::try_from(i).unwrap())
    .sum()
}

fn part_two(input: &str) -> u32 {
  let list_a = "[[2]]";
  let list_b = "[[6]]";
  let altered_input = String::from(input) + "\n" + list_a + "\n" + list_b;
  let mut lists = parse_all_lists(altered_input.as_str());
  let divider_packet_a = parse_list(list_a);
  let divider_packet_b = parse_list(list_b);
  lists.sort();
  lists
    .iter()
    .enumerate()
    .map(|(i, list)| (i + 1, list))
    .filter(|(_, list)| **list == divider_packet_a || **list == divider_packet_b)
    .map(|(i, _)| u32::try_from(i).unwrap())
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
}
