use nom::branch::alt;
use nom::bytes::complete::take;
use nom::combinator::map;
use nom::Parser;
use nom::{
  bytes::complete::tag, character::complete::newline, multi::separated_list1, sequence::preceded,
  IResult,
};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

#[derive(Debug, Clone)]
struct Valve {
  name: String,
  flow_rate: u32,
  tunnels: Vec<String>,
  mapped_tunnels: Vec<usize>,
}

fn valve_name(input: &str) -> IResult<&str, String> {
  map(take(2usize), String::from).parse(input)
}

fn valves(input: &str) -> IResult<&str, Vec<Valve>> {
  separated_list1(
    newline,
    map(
      (
        preceded(tag("Valve "), valve_name),
        preceded(tag(" has flow rate="), nom::character::complete::u32),
        preceded(
          alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
          )),
          separated_list1(tag(", "), valve_name),
        ),
      ),
      |(name, flow_rate, tunnels)| Valve {
        name,
        flow_rate,
        tunnels,
        mapped_tunnels: Vec::new(),
      },
    ),
  )
  .parse(input)
}

fn make_adjacency_matrix(valves: &Vec<Valve>) -> Vec<Vec<u32>> {
  let mut adjacency_matrix = vec![vec![u32::MAX / 2; valves.len()]; valves.len()];
  valves
    .iter()
    .enumerate()
    .flat_map(|(i, v)| v.mapped_tunnels.iter().map(move |&j| (i, j)))
    .for_each(|(i, j)| adjacency_matrix[i][j] = 1);
  adjacency_matrix
}

fn floyd_warshall(adjacency_matrix: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
  let mut distance_matrix = adjacency_matrix.clone();
  for k in 0..distance_matrix.len() {
    for i in 0..distance_matrix.len() {
      for j in 0..distance_matrix.len() {
        if distance_matrix[i][k] + distance_matrix[k][j] < distance_matrix[i][j] {
          distance_matrix[i][j] = distance_matrix[i][k] + distance_matrix[k][j];
        }
      }
    }
  }
  distance_matrix
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct BitMask {
  bits: u64,
}

impl BitMask {
  pub fn new() -> Self {
    Self { bits: 0 }
  }

  fn is_bit_set(&self, index: usize) -> bool {
    self.bits & (1 << index) != 0
  }

  fn with_bit_set(&self, index: usize) -> BitMask {
    BitMask {
      bits: self.bits | (1 << index),
    }
  }

  fn overlaps(&self, other: &BitMask) -> bool {
    self.bits & other.bits != 0
  }
}

fn depth_first_search(
  current_valve: usize,
  open_valves: BitMask,
  current_flow: u32,
  time_remaining: u32,
  valves: &Vec<Valve>,
  distance_matrix: &Vec<Vec<u32>>,
  candidate_valves: &Vec<usize>,
  open_valves_flows: &mut HashMap<BitMask, u32>,
) -> u32 {
  let mut flow = current_flow;

  open_valves_flows
    .entry(open_valves)
    .and_modify(|v| {
      if flow > *v {
        *v = flow;
      }
    })
    .or_insert(flow);

  for &next_valve in candidate_valves {
    let time_left_after_moving = time_remaining
      .checked_sub(distance_matrix[current_valve][next_valve] + 1)
      .unwrap_or(0);
    if time_left_after_moving == 0 || open_valves.is_bit_set(next_valve) {
      continue;
    }
    flow = flow.max(depth_first_search(
      next_valve,
      open_valves.with_bit_set(next_valve),
      current_flow + (time_left_after_moving * valves[next_valve].flow_rate),
      time_left_after_moving,
      valves,
      distance_matrix,
      candidate_valves,
      open_valves_flows,
    ))
  }

  flow
}

fn simulate_all_paths(
  valves: &Vec<Valve>,
  distance_matrix: &Vec<Vec<u32>>,
  minutes: u32,
) -> (HashMap<BitMask, u32>, u32) {
  let aa_index = valves
    .iter()
    .position(|v| v.name == "AA")
    .expect("no AA valve?");

  let open_valves: BitMask = BitMask::new();
  let candidate_valves: Vec<usize> = valves
    .iter()
    .enumerate()
    .filter(|(_, v)| v.flow_rate > 0)
    .map(|(i, _)| i)
    .collect();
  let mut open_valves_flows: HashMap<BitMask, u32> = HashMap::new();

  let max_flow = depth_first_search(
    aa_index,
    open_valves,
    0,
    minutes,
    valves,
    distance_matrix,
    &candidate_valves,
    &mut open_valves_flows,
  );

  (open_valves_flows, max_flow)
}

fn max_flow_two_disjoint_valve_sets(open_valves: &HashMap<BitMask, u32>) -> u32 {
  open_valves
    .iter()
    .enumerate()
    .filter_map(|(i, (a, a_flow))| {
      open_valves
        .iter()
        .skip(i + 1)
        .filter(|(b, _)| !a.overlaps(b))
        .map(|(_, b_flow)| a_flow + b_flow)
        .max()
    })
    .max()
    .unwrap_or(0)
}

fn parse_valves(input: &str) -> Vec<Valve> {
  let (_, mut valves) = valves(input).unwrap();

  let valves_map: HashMap<String, usize> = valves
    .iter()
    .enumerate()
    .map(|(i, v)| (v.name.clone(), i))
    .collect();

  valves.iter_mut().for_each(|v| {
    v.tunnels
      .iter()
      .for_each(|t| v.mapped_tunnels.push(*valves_map.get(t).unwrap()))
  });

  valves
}

fn part_one(input: &str) -> u32 {
  let valves = parse_valves(input);
  let adjacency_matrix = make_adjacency_matrix(&valves);
  let distance_matrix = floyd_warshall(&adjacency_matrix);
  let (_, max_flow) = simulate_all_paths(&valves, &distance_matrix, 30);
  max_flow
}

fn part_two(input: &str) -> u32 {
  let valves = parse_valves(input);
  let adjacency_matrix = make_adjacency_matrix(&valves);
  let distance_matrix = floyd_warshall(&adjacency_matrix);
  let (valves_map, _) = simulate_all_paths(&valves, &distance_matrix, 26);
  max_flow_two_disjoint_valve_sets(&valves_map)
}

fn main() {
  let input = include_str!("input");
  let start = Instant::now();
  println!("{}", part_one(input));
  println!("time: {:?}", start.elapsed());
  println!();
  println!("{}", part_two(input));
  println!("time: {:?}", start.elapsed());
}

#[cfg(test)]
mod test {
  use super::*;
  const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

  #[test]
  fn part_one_example() {
    assert_eq!(part_one(INPUT), 1651);
  }

  #[test]
  fn part_two_example() {
    assert_eq!(part_two(INPUT), 1707);
  }
}
