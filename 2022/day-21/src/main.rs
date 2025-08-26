use nom::{
  branch::alt,
  bytes::complete::tag,
  character::complete::{alpha1, newline, space1},
  combinator::map,
  multi::separated_list1,
  sequence::{delimited, terminated},
  IResult, Parser,
};
use std::{collections::HashMap, time::Instant};

#[derive(Debug, Clone, Copy)]
enum Operator {
  Plus,
  Minus,
  Times,
  Divide,
}

impl Operator {
  fn from_char(c: char) -> Self {
    match c {
      '+' => Self::Plus,
      '-' => Self::Minus,
      '*' => Self::Times,
      '/' => Self::Divide,
      _ => panic!("unknwon operator"),
    }
  }
  fn apply(&self, left: u64, right: u64) -> u64 {
    match self {
      Self::Plus => left + right,
      Self::Minus => left - right,
      Self::Times => left * right,
      Self::Divide => left / right,
    }
  }
  fn isolate_operand<'a>(
    &self,
    variable: &'a str,
    other_operand: &'a str,
    is_left: bool,
  ) -> (&'a str, &'a str, Self) {
    match (self, is_left) {
      (Self::Plus, _) => (variable, other_operand, Self::Minus),
      (Self::Minus, true) => (variable, other_operand, Self::Plus),
      (Self::Minus, false) => (other_operand, variable, Self::Minus),
      (Self::Times, _) => (variable, other_operand, Self::Divide),
      (Self::Divide, true) => (variable, other_operand, Self::Times),
      (Self::Divide, false) => (other_operand, variable, Self::Divide),
    }
  }
}

#[derive(Debug, Clone)]
struct Monkey {
  name: String,
  left: Option<String>,
  right: Option<String>,
  number: Option<u64>,
  op: Option<Operator>,
}

impl Monkey {
  fn from_number(name: &str, number: u64) -> Self {
    Self {
      name: name.to_owned(),
      left: None,
      right: None,
      number: Some(number),
      op: None,
    }
  }
  fn from_op(name: &str, left: &str, right: &str, op: Operator) -> Self {
    Self {
      name: name.to_owned(),
      left: Some(left.to_owned()),
      right: Some(right.to_owned()),
      number: None,
      op: Some(op),
    }
  }
  fn invert_for(&self, child_name: &str) -> Self {
    let is_left = {
      if let Some(left_name) = &self.left {
        child_name == left_name
      } else {
        false
      }
    };
    let other_child_name = {
      if is_left {
        self.right.clone().expect("no right child")
      } else {
        self.left.clone().expect("no left child")
      }
    };
    let self_op = self.op.expect("no op?");
    let (left, right, new_op) = { self_op.isolate_operand(&self.name, &other_child_name, is_left) };
    Monkey::from_op(child_name, left, right, new_op)
  }
}

struct Riddle {
  monkeys: HashMap<String, Monkey>,
}

impl Riddle {
  fn solve_monkey(&mut self, monkey_name: &str) -> u64 {
    if let Some(number) = self.monkeys[monkey_name].number {
      return number;
    }
    let (left, right, op) = {
      let monkey = &self.monkeys.get(monkey_name).unwrap();
      match (&monkey.left, &monkey.right, &monkey.op) {
        (Some(left), Some(right), Some(op)) => (left.clone(), right.clone(), op.clone()),
        _ => panic!("no other monkeys?"),
      }
    };
    let left_res = self.solve_monkey(&left);
    let right_res = self.solve_monkey(&right);
    let result = op.apply(left_res, right_res);

    self.monkeys.get_mut(monkey_name).unwrap().number = Some(result);
    result
  }
  fn reverse_riddle(&self) -> Self {
    let root = self.monkeys.get("root").expect("no root monkey");
    let left_name = root.left.as_ref().expect("root monkey has no left");
    let right_name = root.right.as_ref().expect("root monkey has no left");
    let in_left = self.branch_contains(left_name, "humn");
    let mut result: HashMap<String, Monkey> = HashMap::new();
    let new_root = Monkey::from_number("root", 0);
    let (final_monkey, new_final) = {
      if in_left {
        (
          left_name,
          Monkey::from_op(left_name, right_name, "root", Operator::Plus),
        )
      } else {
        (
          right_name,
          Monkey::from_op(right_name, left_name, "root", Operator::Plus),
        )
      }
    };
    result.insert("root".to_owned(), new_root);
    result.insert(final_monkey.to_string(), new_final);
    let mut monkey_to_reverse = Some("humn");
    while let Some(current_monkey) = monkey_to_reverse {
      let parent_monkey = self
        .monkeys
        .values()
        .find(|monkey| match (&monkey.left, &monkey.right) {
          (Some(left), Some(right)) => left == current_monkey || right == current_monkey,
          _ => false,
        })
        .expect("no parent monkey for current monkey");
      let new_monkey = parent_monkey.invert_for(current_monkey);
      result.insert(current_monkey.to_string(), new_monkey);
      monkey_to_reverse = {
        if parent_monkey.name == *final_monkey {
          None
        } else {
          Some(parent_monkey.name.as_ref())
        }
      }
    }
    let names_to_add: Vec<String> = self
      .monkeys
      .values()
      .filter(|m| !result.contains_key(&m.name))
      .map(|m| m.name.clone())
      .collect();

    names_to_add
      .iter()
      .map(|name| self.monkeys.get(name).unwrap())
      .for_each(|m| {
        result.insert(
          m.name.clone(),
          m.clone(),
        );
      });
    Self { monkeys: result }
  }
  fn branch_contains(&self, current_monkey: &str, target_monkey: &str) -> bool {
    if current_monkey == target_monkey {
      return true;
    }
    let (left, right) = {
      let monkey = &self.monkeys.get(current_monkey).unwrap();
      match (&monkey.left, &monkey.right) {
        (Some(left), Some(right)) => (left.clone(), right.clone()),
        _ => return false,
      }
    };
    let left_res = self.branch_contains(&left, target_monkey);
    let right_res = self.branch_contains(&right, target_monkey);
    return left_res || right_res;
  }
  fn from_str(input: &str) -> Self {
    let (_, monkas) = monkeys(input).expect("monka parsing error");
    let map: HashMap<String, Monkey> = monkas
      .into_iter()
      .map(|monka| (monka.name.clone(), monka))
      .collect();
    Self { monkeys: map }
  }
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
  let (input, name) = terminated(alpha1, tag(": ")).parse(input)?;
  alt((
    map(nom::character::complete::u64, |number| {
      Monkey::from_number(name, number)
    }),
    map(
      (
        alpha1,
        delimited(
          space1,
          map(nom::character::complete::anychar, Operator::from_char),
          space1,
        ),
        alpha1,
      ),
      |(left, op, right)| Monkey::from_op(name, left, right, op),
    ),
  ))
  .parse(input)
}

fn monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
  separated_list1(newline, monkey).parse(input)
}

fn part_one(input: &str) -> u64 {
  let mut riddle = Riddle::from_str(input);
  riddle.solve_monkey("root")
}

fn part_two(input: &str) -> u64 {
  let riddle = Riddle::from_str(input);
  let mut reversed_riddle = riddle.reverse_riddle();
  reversed_riddle.solve_monkey("humn")
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
  const INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

  #[test]
  fn part_one_example() {
    assert_eq!(part_one(INPUT), 152);
  }

  #[test]
  fn part_two_example() {
    assert_eq!(part_two(INPUT), 301);
  }
}
