use nom::{
  bytes::complete::tag,
  character::complete::{digit1, newline},
  combinator::map,
  multi::separated_list1,
  sequence::{delimited, preceded},
  IResult, Parser,
};
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
struct ResourceCount {
  ore: u32,
  clay: u32,
  obsidian: u32,
}

impl ResourceCount {
  fn for_ore_robot(ore_cost: u32) -> Self {
    Self {
      ore: ore_cost,
      clay: 0,
      obsidian: 0,
    }
  }
  fn for_clay_robot(ore_cost: u32) -> Self {
    Self {
      ore: ore_cost,
      clay: 0,
      obsidian: 0,
    }
  }
  fn for_obsidian_robot(ore_cost: u32, clay_cost: u32) -> Self {
    Self {
      ore: ore_cost,
      clay: clay_cost,
      obsidian: 0,
    }
  }
  fn for_geode_robot(ore_cost: u32, obsidian_cost: u32) -> Self {
    Self {
      ore: ore_cost,
      clay: 0,
      obsidian: obsidian_cost,
    }
  }
  fn all_zeros() -> Self {
    Self {
      ore: 0,
      clay: 0,
      obsidian: 0,
    }
  }
}

impl ResourceCount {
  fn checked_sub(&self, other: &Self) -> Option<Self> {
    match (
      self.ore.checked_sub(other.ore),
      self.clay.checked_sub(other.clay),
      self.obsidian.checked_sub(other.obsidian),
    ) {
      (Some(new_ore), Some(new_clay), Some(new_obsidian)) => Some(Self {
        ore: new_ore,
        clay: new_clay,
        obsidian: new_obsidian,
      }),
      _ => None,
    }
  }
}

#[derive(Debug, Clone, Copy)]
struct Blueprint {
  ore_robot_cost: ResourceCount,
  clay_robot_cost: ResourceCount,
  obsidian_robot_cost: ResourceCount,
  geode_robot_cost: ResourceCount,
}

impl Blueprint {
  fn need_ore_robots(&self, robots: u32) -> bool {
    let max_ore_needed = self
      .ore_robot_cost
      .ore
      .max(self.clay_robot_cost.ore)
      .max(self.obsidian_robot_cost.ore)
      .max(self.geode_robot_cost.ore);
    robots < max_ore_needed
  }
  fn need_clay_robots(&self, robots: u32) -> bool {
    robots < self.obsidian_robot_cost.clay
  }
  fn need_obsidian_robots(&self, robots: u32) -> bool {
    robots < self.geode_robot_cost.obsidian
  }
}

#[derive(Debug, Clone, Copy)]
struct State {
  geodes: u32,
  resources: ResourceCount,
  geode_robots: u32,
  obsidian_robots: u32,
  clay_robots: u32,
  ore_robots: u32,
}

impl State {
  fn initial() -> Self {
    Self {
      geodes: 0,
      resources: ResourceCount::all_zeros(),
      geode_robots: 0,
      obsidian_robots: 0,
      clay_robots: 0,
      ore_robots: 1,
    }
  }

  fn try_build_ore_robot(&self, robot_cost: &ResourceCount) -> Option<Self> {
    if let Some(remaining_resources) = self.resources.checked_sub(robot_cost) {
      let mut new_state = *self;
      new_state.resources = remaining_resources;
      new_state.collect_resources();
      new_state.ore_robots += 1;
      Some(new_state)
    } else {
      None
    }
  }

  fn try_build_clay_robot(&self, robot_cost: &ResourceCount) -> Option<Self> {
    if let Some(remaining_resources) = self.resources.checked_sub(robot_cost) {
      let mut new_state = *self;
      new_state.resources = remaining_resources;
      new_state.collect_resources();
      new_state.clay_robots += 1;
      Some(new_state)
    } else {
      None
    }
  }

  fn try_build_obsidian_robot(&self, robot_cost: &ResourceCount) -> Option<Self> {
    if let Some(remaining_resources) = self.resources.checked_sub(robot_cost) {
      let mut new_state = *self;
      new_state.resources = remaining_resources;
      new_state.collect_resources();
      new_state.obsidian_robots += 1;
      Some(new_state)
    } else {
      None
    }
  }

  fn try_build_geode_robot(&self, robot_cost: &ResourceCount) -> Option<Self> {
    if let Some(remaining_resources) = self.resources.checked_sub(robot_cost) {
      let mut new_state = *self;
      new_state.resources = remaining_resources;
      new_state.collect_resources();
      new_state.geode_robots += 1;
      Some(new_state)
    } else {
      None
    }
  }

  fn collect_resources(&mut self) {
    self.geodes += self.geode_robots;
    self.resources.ore += self.ore_robots;
    self.resources.clay += self.clay_robots;
    self.resources.obsidian += self.obsidian_robots;
  }

  fn with_collected_resources(&self) -> Self {
    Self {
      geodes: self.geodes + self.geode_robots,
      resources: ResourceCount {
        ore: self.resources.ore + self.ore_robots,
        clay: self.resources.clay + self.clay_robots,
        obsidian: self.resources.obsidian + self.obsidian_robots,
      },
      ore_robots: self.ore_robots,
      clay_robots: self.clay_robots,
      obsidian_robots: self.obsidian_robots,
      geode_robots: self.geode_robots,
    }
  }
}

fn max_geodes(state: State, minutes_remaining: u32, blueprint: &Blueprint) -> u32 {
  let mut max_seen: u32 = 0;
  depth_first_search(state, &mut max_seen, minutes_remaining, blueprint)
}

fn depth_first_search(
  state: State,
  max_seen: &mut u32,
  minutes_remaining: u32,
  blueprint: &Blueprint,
) -> u32 {
  if minutes_remaining <= 0 {
    return state.geodes;
  }

  if state.geodes + minutes_remaining * state.geode_robots + minutes_remaining * (minutes_remaining - 1) / 2 <= *max_seen {
    return *max_seen;
  }

  if let Some(new_state) = state.try_build_geode_robot(&blueprint.geode_robot_cost) {
    let next_geodes = depth_first_search(new_state, max_seen, minutes_remaining - 1, blueprint);
    *max_seen = (*max_seen).max(next_geodes);
    return *max_seen;
  }

  if let Some(new_state) = state.try_build_ore_robot(&blueprint.ore_robot_cost) {
    if blueprint.need_ore_robots(state.ore_robots) {
      let next_geodes = depth_first_search(new_state, max_seen, minutes_remaining - 1, blueprint);
      *max_seen = (*max_seen).max(next_geodes);
    }
  }

  if let Some(new_state) = state.try_build_clay_robot(&blueprint.clay_robot_cost) {
    if blueprint.need_clay_robots(state.clay_robots) {
      let next_geodes = depth_first_search(new_state, max_seen, minutes_remaining - 1, blueprint);
      *max_seen = (*max_seen).max(next_geodes);
    }
  }

  if let Some(new_state) = state.try_build_obsidian_robot(&blueprint.obsidian_robot_cost) {
    if blueprint.need_obsidian_robots(state.obsidian_robots) {
      let next_geodes = depth_first_search(new_state, max_seen, minutes_remaining - 1, blueprint);
      *max_seen = (*max_seen).max(next_geodes);
    }
  }

  let next_state = state.with_collected_resources();

  let next_geodes = depth_first_search(next_state, max_seen, minutes_remaining - 1, blueprint);

  *max_seen = (*max_seen).max(next_geodes);

  return *max_seen;
}

fn blueprint(input: &str) -> IResult<&str, Blueprint> {
  map(
    (
      preceded(
        delimited(tag("Blueprint "), digit1, tag(": Each ore robot costs ")),
        nom::character::complete::u32,
      ),
      preceded(
        tag(" ore. Each clay robot costs "),
        nom::character::complete::u32,
      ),
      preceded(
        tag(" ore. Each obsidian robot costs "),
        nom::character::complete::u32,
      ),
      preceded(tag(" ore and "), nom::character::complete::u32),
      preceded(
        tag(" clay. Each geode robot costs "),
        nom::character::complete::u32,
      ),
      delimited(
        tag(" ore and "),
        nom::character::complete::u32,
        tag(" obsidian."),
      ),
    ),
    |(ore_ore, clay_ore, obsidian_ore, obsidian_clay, geode_ore, geode_obsidian)| Blueprint {
      ore_robot_cost: ResourceCount::for_ore_robot(ore_ore),
      clay_robot_cost: ResourceCount::for_clay_robot(clay_ore),
      obsidian_robot_cost: ResourceCount::for_obsidian_robot(obsidian_ore, obsidian_clay),
      geode_robot_cost: ResourceCount::for_geode_robot(geode_ore, geode_obsidian),
    },
  )
  .parse(input)
}

fn blueprints(input: &str) -> IResult<&str, Vec<Blueprint>> {
  separated_list1(newline, blueprint).parse(input)
}

fn part_one(input: &str) -> u32 {
  let (_, blueprints) = blueprints(input).unwrap();
  let minutes = 24;
  blueprints
    .into_iter()
    .enumerate()
    .map(|(i, blueprint)| ((i as u32) + 1) * max_geodes(State::initial(), minutes, &blueprint)) // slow ~6s
    .sum()
}

fn part_two(input: &str) -> u32 {
  let (_, blueprints) = blueprints(input).unwrap();
  let minutes = 32;
  blueprints
    .into_iter()
    .take(3)
    .map(|blueprint| max_geodes(State::initial(), minutes, &blueprint)) // slooooooow ~40s
    .product()
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
  const INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

  #[test]
  fn part_one_example() {
    assert_eq!(part_one(INPUT), 33);
  }

  #[test]
  fn part_two_example() {
    assert_eq!(part_two(INPUT), 3472);
  }
}
