use nom::{
    branch::alt, bytes::complete::{tag, take_till, take_until}, character::complete::{alphanumeric0, digit0, digit1, multispace0, newline}, combinator::{map_res, rest}, multi::{many0, separated_list1}, sequence::{delimited, pair, preceded, terminated}, IResult, Parser
};

#[derive(Debug)]
enum Operation {
  Mult(u64),
  Plus(u64),
  Square,
}

fn from_text(input: &str) -> Result<u64, std::num::ParseIntError> {
  return input.parse::<u64>();
}

fn operator_parser(input: &str) -> IResult<&str, &str> {
  preceded(
    pair(
      multispace0,
      tag("Operation: new = old ")
    ),
    alt((
      tag("+"),
      tag("*")
    ))
  ).parse(input)
}

fn operation_parser(input: &str) -> IResult<&str, Operation> {
  let (input, operator) = operator_parser(input)?;
  let (input, rhs) = delimited(
    multispace0,
    alphanumeric0,
    newline
  ).parse(input)?;
  if rhs.eq("old") {
    return Ok((input, Operation::Square));
  } else {
    let operand = from_text(rhs).unwrap();
    if operator.eq("+") {
      return Ok((input, Operation::Plus(operand)));
    } else if operator.eq("*") {
      return Ok((input, Operation::Mult(operand)));
    } else {
      todo!("unknown operation");
    }
  }
}

fn integer_list_from_line(input: &str) -> IResult<&str, Vec<u64>> {
  let (input, numbers) = preceded(
    take_till(char::is_numeric),
    separated_list1(
      tag(", "),
      map_res(
        digit0,
        from_text
      )
    )
  ).parse(input)?;
  let (input, _) = alt((
    terminated(take_until("\n"), tag("\n")),
    rest,
  )).parse(input)?;
  Ok((input, numbers))
}

fn integer_from_line(input: &str) -> IResult<&str, u64> {
  let (input, number) = preceded(
    take_till(char::is_numeric),
    map_res(digit1, from_text)
  ).parse(input)?;
  let (input, _) = alt((
    terminated(take_until("\n"), tag("\n")),
    rest,
  )).parse(input)?;
  Ok((input, number))
}

fn monkey_parser(input: &str) -> IResult<&str, Monkey> {
  let (input, _) = integer_from_line(input)?;
  let (input, starting_items) = integer_list_from_line(input)?;
  let (input, operation) = operation_parser(input)?;
  let (input, mod_factor) = integer_from_line(input)?;
  let (input, next_true) = integer_from_line(input)?;
  let (input, next_false) = integer_from_line(input)?;
  Ok((input, Monkey::create(
    starting_items,
    operation,
    mod_factor,
    usize::try_from(next_true).unwrap(),
    usize::try_from(next_false).unwrap()
  )))
}

fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    let (input, monkeys) = many0(monkey_parser).parse(input)?;
    Ok((input, monkeys))
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test_mod: u64,
    next_true: usize,
    next_false: usize,
    inspected_items: u64
}

impl Monkey {
  fn create(
    items: Vec<u64>,
    operation: Operation,
    test_mod: u64,
    next_true: usize,
    next_false: usize,
  ) -> Self {
    Self {
      items,
      operation,
      test_mod,
      next_true,
      next_false,
      inspected_items: 0
    }
  }
  fn inspect_and_throw_items(&mut self, observer: &Observer) -> Vec<ThrowInstruction> {
    let mut throw_instructions: Vec<ThrowInstruction> = Vec::new();
    self.inspected_items += u64::try_from(self.items.len()).unwrap();
    for initial_worry_level in self.items.drain(..) {
      let new_worry_level = observer.observe_inspection(
        initial_worry_level,
        &self.operation
      );
      throw_instructions.push((
        new_worry_level,
        if new_worry_level % self.test_mod == 0 { self.next_true } else { self.next_false }
      ));
    }
    return throw_instructions
  }
}

struct Observer {
  relief_factor: u64,
  test_prod: u64
}

impl Observer {
  fn create(
    monkeys: &Vec<Monkey>,
    relief_factor: u64
  ) -> Self {
    let test_prod = monkeys
      .iter()
      .map(|m| m.test_mod)
      .reduce(|a ,b| a * b);
    Self {
      test_prod: Option::expect(test_prod, "could not multiply test conditions"),
      relief_factor
    }
  }
  fn observe_inspection(&self, worry_level: u64, operation: &Operation) -> u64 {
    let new_worry_level = match operation {
      Operation::Mult(factor) => (worry_level * factor) % self.test_prod,
      Operation::Plus(addend) => (worry_level + addend) % self.test_prod,
      Operation::Square => (worry_level * worry_level) % self.test_prod
    };
    return (new_worry_level / self.relief_factor) % self.test_prod;
  }
}

type ThrowInstruction = (u64, usize);

fn simulate_round(monkeys: &mut Vec<Monkey>, observer: &Observer) {
  for m_i in 0..monkeys.len() {
    let monkey = &mut monkeys[m_i];
    let throw_instructions = monkey.inspect_and_throw_items(observer);
    for (item, monkey_index) in throw_instructions {
      monkeys[monkey_index].items.push(item);
    }
  }
}

fn monkey_business(input: &str, n_rounds: usize, relief_factor: u64) -> u64{
  let (_, mut monkeys) = parse_monkeys(input).unwrap();
  let observer = Observer::create(&monkeys, relief_factor);
  for _ in 0..n_rounds {
    simulate_round(&mut monkeys, &observer);
  }
  monkeys.sort_by(|a, b| b.inspected_items.cmp(&a.inspected_items));
  let first_two = &monkeys[..2];
  match first_two {
    [m1, m2] => {
      return m1.inspected_items * m2.inspected_items
    }
    _ => todo!("will we ever have less than two monkeys?")
  }
}

fn part_one(input: &str) -> u64 {
  monkey_business(input, 20, 3)
}

fn part_two(input: &str) -> u64 {
  monkey_business(input, 10000, 1)
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
    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part_one_example() {
        assert_eq!(part_one(INPUT), 10605);
    }
}
