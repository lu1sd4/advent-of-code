use std::cmp::max;
use nom::combinator::map;
use nom::sequence::separated_pair;
use nom::Parser;
use nom::{
  bytes::complete::tag,
  character::complete::newline,
  multi::separated_list1,
  sequence::{pair, preceded},
  IResult,
};

type GridPosition = (i32, i32);

type Interval = (i32, i32);

#[derive(Debug)]
struct Report {
  sensor_position: GridPosition,
  beacon_position: GridPosition,
}

impl Report {
  fn unused_interval_at(&self, y: i32) -> Option<Interval> {
    let radius = (self.sensor_position.0 - self.beacon_position.0).abs()
      + (self.sensor_position.1 - self.beacon_position.1).abs();
    let dy_sensor_to_y = (self.sensor_position.1 - y).abs();
    if dy_sensor_to_y > radius {
      return None;
    }
    let dx_sensor_to_edge = radius - dy_sensor_to_y;
    return Some((
      self.sensor_position.0 - dx_sensor_to_edge,
      self.sensor_position.0 + dx_sensor_to_edge,
    ));
  }
}

fn xy_position(input: &str) -> IResult<&str, GridPosition> {
  separated_pair(
    preceded(tag("x="), nom::character::complete::i32),
    tag(", "),
    preceded(tag("y="), nom::character::complete::i32),
  )
  .parse(input)
}

fn reports(input: &str) -> IResult<&str, Vec<Report>> {
  separated_list1(
    newline,
    map(
      pair(
        preceded(tag("Sensor at "), xy_position),
        preceded(tag(": closest beacon is at "), xy_position),
      ),
      |(sensor_position, beacon_position)| Report {
        sensor_position,
        beacon_position,
      },
    ),
  )
  .parse(input)
}

fn merge_intervals(intervals: &mut Vec<Interval>) -> Vec<Interval> {
  let mut result: Vec<Interval> = Vec::new();
  let mut last_interval: Interval = *intervals.get(0).unwrap();
  for interval in intervals.into_iter().skip(1) {
    if interval.0 <= last_interval.1 {
      last_interval.1 = max(interval.1, last_interval.1);
    } else {
      result.push(last_interval);
      last_interval = *interval;
    }
  }
  result.push(last_interval);
  return result;
}

fn part_one(input: &str, y: i32) -> usize {
  let (_, reports) = reports(input).unwrap();
  let mut intervals: Vec<Interval> = reports
    .iter()
    .filter_map(|r| r.unused_interval_at(y))
    .collect();
  intervals.sort();
  let merged_intervals = merge_intervals(&mut intervals);
  merged_intervals
    .iter()
    .map(|int| (int.0 - int.1).abs() as usize)
    .sum()
}

fn part_two(input: &str, limit: i32) -> u64 {
  let (_, reports) = reports(input).unwrap();
  for y in 0..limit {
    let mut intervals: Vec<Interval> = reports
      .iter()
      .filter_map(|r| r.unused_interval_at(y))
      .collect();
    intervals.sort();
    let merged_intervals = merge_intervals(&mut intervals);
    let beacon_position = merged_intervals
      .iter()
      .map(|(_, end)| end)
      .find_map(|&e| {
        if e >= -1 && e <= limit - 1 {
          return Some(e + 1);
        }
        None
      });
    if beacon_position.is_some() {
      let res_x: u64 = beacon_position.unwrap().try_into().unwrap();
      let res_y: u64 = y.try_into().unwrap();
      return 4000000 * res_x + res_y;
    }
  }
  todo!("return")
}

fn main() {
  let input = include_str!("input");
  println!("{}", part_one(input, 2000000));
  println!();
  println!("{}", part_two(input, 4000000));
}

#[cfg(test)]
mod test {
  use super::*;
  const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

  #[test]
  fn part_one_example() {
    assert_eq!(part_one(INPUT, 10), 26);
  }

  #[test]
  fn part_two_example() {
    assert_eq!(part_two(INPUT, 20), 56000011);
  }

  #[test]
  fn intervals() {
    let vals = vec![
      ((0, 0), (15, 15), 2, (-28, 28)),
      ((0, 0), (15, -15), 2, (-28, 28)),
      ((0, 0), (-15, 15), 2, (-28, 28)),
      ((0, 0), (-15, -15), 2, (-28, 28)),
      ((0, 0), (3, 5), 3, (-5, 5)),
      ((-1, 2), (3, 5), 3, (-7, 5)),
      ((8, 7), (2, 10), 6, (0, 16)),
    ];
    for (s, b, y, i) in vals {
      let report = Report {
        sensor_position: s,
        beacon_position: b,
      };
      assert_eq!(report.unused_interval_at(y), Some(i));
    }
  }
}
