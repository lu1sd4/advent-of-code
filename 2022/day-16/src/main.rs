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

#[derive(Debug)]
struct Network<'a> {
  valves: HashMap<&'a str, Valve<'a>>,
  non_zero_valves: Vec<&'a str>,
  distance_matrix: HashMap<&'a str, HashMap<&'a str, i32>>,
}

#[derive(Debug)]
struct Valve<'a> {
  name: &'a str,
  flow_rate: i32,
  neighbors: Vec<&'a str>,
}

#[derive(Debug, Clone)]
struct Path<'a> {
  total_flow: i32,
  open_valves: Vec<&'a str>,
  open_valves_set: HashSet<&'a str>,
}

impl<'a> Path<'a> {
  fn empty_path() -> Self {
    Self {
      total_flow: 0,
      open_valves: Vec::new(),
      open_valves_set: HashSet::new(),
    }
  }
}

fn floyd_warshall<'a>(
  valves: &HashMap<&'a str, Valve<'a>>,
) -> HashMap<&'a str, HashMap<&'a str, i32>> {
  let mut distances: HashMap<&'a str, HashMap<&'a str, i32>> = HashMap::new();
  for &i in valves.keys() {
    for &j in valves.keys() {
      distances.entry(i).or_default().insert(
        j,
        if i == j {
          0
        } else if valves[i].neighbors.contains(&j) {
          1
        } else {
          i32::MAX / 2
        },
      );
    }
  }
  for &k in valves.keys() {
    for &i in valves.keys() {
      for &j in valves.keys() {
        let ik = distances[i][k];
        let kj = distances[k][j];
        let ij = distances[i][j];
        if ik + kj < ij {
          distances.get_mut(i).unwrap().insert(j, ik + kj);
        }
      }
    }
  }
  return distances;
}

impl<'a> Network<'a> {
  fn make_network(valves: HashMap<&'a str, Valve<'a>>) -> Self {
    let non_zero_valves: Vec<&str> = valves
      .values()
      .filter(|v| v.flow_rate > 0)
      .map(|v| v.name)
      .collect();
    let distance_matrix = floyd_warshall(&valves);
    return Self {
      valves,
      non_zero_valves,
      distance_matrix,
    };
  }
  fn depth_first_search(
    &self,
    current_valve: &'a str,
    depth_left: i32,
    current_path: &mut Path<'a>,
  ) -> Vec<Path<'a>> {
    let mut paths: Vec<Path> = vec![current_path.clone()];
    let mut next_depth_left: i32;
    for &next_valve in &self.non_zero_valves {
      next_depth_left = depth_left - self.distance_matrix[current_valve][next_valve] - 1;
      if current_path.open_valves_set.contains(next_valve) || next_depth_left < 0 {
        continue;
      }
      current_path.open_valves.push(next_valve);
      current_path.open_valves_set.insert(next_valve);
      current_path.total_flow += next_depth_left * self.valves[next_valve].flow_rate;
      paths.extend(self.depth_first_search(next_valve, next_depth_left, current_path));
      current_path.total_flow -= next_depth_left * self.valves[next_valve].flow_rate;
      current_path.open_valves.pop();
      current_path.open_valves_set.remove(next_valve);
    }
    return paths;
  }
  fn simulate_all_paths(&self, minutes: i32) -> Vec<Path> {
    return self.depth_first_search("AA", minutes, &mut Path::empty_path());
  }
}

fn valve_name(input: &str) -> IResult<&str, &str> {
  take(2usize).parse(input)
}

fn valve(input: &str) -> IResult<&str, Valve> {
  map(
    (
      preceded(tag("Valve "), valve_name),
      preceded(tag(" has flow rate="), nom::character::complete::i32),
      preceded(
        alt((
          tag("; tunnels lead to valves "),
          tag("; tunnel leads to valve "),
        )),
        separated_list1(tag(", "), valve_name),
      ),
    ),
    |(name, flow_rate, neighbors)| Valve {
      name,
      flow_rate,
      neighbors,
    },
  )
  .parse(input)
}

fn valves(input: &str) -> IResult<&str, Vec<Valve>> {
  separated_list1(newline, valve).parse(input)
}

fn max_flow_non_crossing_paths(paths: &Vec<Path>) -> i32 {
  let filtered: Vec<_> = paths.iter().filter(|p| !p.open_valves.is_empty()).collect();

  filtered
    .iter()
    .enumerate()
    .flat_map(|(i, my_path)| {
      filtered
        .iter()
        .skip(i + 1)
        .filter_map(move |elephant_path| {
          if my_path
            .open_valves_set
            .is_disjoint(&elephant_path.open_valves_set)
          {
            Some(my_path.total_flow + elephant_path.total_flow)
          } else {
            None
          }
        })
    })
    .max()
    .expect("no max flow?")
}

fn part_one(input: &str) -> i32 {
  let (_, valves_list) = valves(input).unwrap();
  let valves_map: HashMap<&str, Valve> = valves_list.into_iter().map(|v| (v.name, v)).collect();
  let network = Network::make_network(valves_map);
  let paths = network.simulate_all_paths(30);
  paths
    .iter()
    .map(|p| p.total_flow)
    .max()
    .expect("no max flow?")
}

fn part_two(input: &str) -> i32 {
  let (_, valves_list) = valves(input).unwrap();
  let valves_map: HashMap<&str, Valve> = valves_list.into_iter().map(|v| (v.name, v)).collect();
  let network = Network::make_network(valves_map);
  let paths = network.simulate_all_paths(26);
  max_flow_non_crossing_paths(&paths) // slowwwwww
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
