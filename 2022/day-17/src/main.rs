use std::{collections::HashMap, time::Instant};

#[derive(Eq, Debug, Clone, Copy)]
struct Point {
  x: i64,
  y: i64,
}

impl PartialEq for Point {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == other.y
  }
}

impl From<(i64, i64)> for Point {
  fn from((x, y): (i64, i64)) -> Self {
    Self { x, y }
  }
}

impl From<Point> for (usize, usize) {
  fn from(point: Point) -> Self {
    (
      point.x.try_into().expect("negative x -> usize"),
      point.y.try_into().expect("negative y -> usize"),
    )
  }
}

const PATTERNS: &[&[(i64, i64)]] = &[
  &[(0, 0), (1, 0), (2, 0), (3, 0)],         // horizontal line
  &[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)], // plus
  &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], // inverted L
  &[(0, 0), (0, 1), (0, 2), (0, 3)],         // vertical line
  &[(0, 0), (1, 0), (0, 1), (1, 1)],         // square
];

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum RockType {
  HorizontalLine = 0,
  Plus = 1,
  InvertedL = 2,
  VerticalLine = 3,
  Square = 4,
}

impl RockType {
  fn pattern(self) -> &'static [(i64, i64)] {
    PATTERNS[self as usize]
  }

  fn next(self) -> Self {
    match self {
      Self::HorizontalLine => Self::Plus,
      Self::Plus => Self::InvertedL,
      Self::InvertedL => Self::VerticalLine,
      Self::VerticalLine => Self::Square,
      Self::Square => Self::HorizontalLine,
    }
  }
}

#[derive(Debug)]
struct Rock {
  points: Vec<Point>,
}

impl Rock {
  fn new(rock_type: RockType, x_offset: i64, bottom_y: i64) -> Self {
    let points = rock_type
      .pattern()
      .iter()
      .map(|&(x, y)| Point::from((x + x_offset, y + bottom_y)))
      .collect();

    Self { points }
  }

  fn translate(&mut self, dx: i64, dy: i64) {
    for point in &mut self.points {
      point.x += dx;
      point.y += dy;
    }
  }

  fn move_down(&mut self) {
    self.translate(0, -1);
  }

  fn move_up(&mut self) {
    self.translate(0, 1);
  }

  fn move_right(&mut self) {
    self.translate(1, 0);
  }

  fn move_left(&mut self) {
    self.translate(-1, 0);
  }
}

#[derive(Clone, Copy, Debug)]
enum WindDirection {
  Left,
  Right,
}

impl From<char> for WindDirection {
  fn from(c: char) -> Self {
    match c {
      '>' => Self::Right,
      '<' => Self::Left,
      _ => panic!("{} is not a wind direction", c),
    }
  }
}

struct Wind {
  directions: Vec<WindDirection>,
  index: usize,
}

impl Wind {
  fn new(directions: Vec<WindDirection>) -> Self {
    Self {
      directions,
      index: 0,
    }
  }

  fn next(&mut self) -> WindDirection {
    let direction = self.directions[self.index];
    self.index = (self.index + 1) % self.directions.len();
    direction
  }

  fn current_index(&self) -> usize {
    self.index
  }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct ChamberState {
  rock_type: RockType,
  wind_position: usize,
  peaks: [i64; 7],
}

#[derive(Debug, Clone)]
struct StateInfo {
  rows: i64,
  rock_count: i64,
}

struct Chamber {
  rows: Vec<[bool; 7]>,
  current_rock_type: RockType,
  wind: Wind,
  state_cache: HashMap<ChamberState, StateInfo>,
}

impl Chamber {
  const CHAMBER_WIDTH: usize = 7;
  const INITIAL_X_OFFSET: i64 = 2;
  const SPAWN_HEIGHT_OFFSET: i64 = 3;

  fn new(wind_pattern: Vec<WindDirection>) -> Self {
    Self {
      rows: Vec::new(),
      current_rock_type: RockType::HorizontalLine,
      wind: Wind::new(wind_pattern),
      state_cache: HashMap::new(),
    }
  }

  fn height_after(&mut self, n_rocks: i64) -> i64 {
    let mut rock_count = 0;

    let initial_state = self.current_state();
    self.state_cache.insert(
      initial_state,
      StateInfo {
        rows: 0,
        rock_count: 0,
      },
    );

    while rock_count < n_rocks {
      self.simulate_rock();
      rock_count += 1;

      let current_state = self.current_state();
      if let Some(cycle_start) = self.state_cache.get(&current_state).cloned() {
        return self.calculate_final_height(n_rocks, rock_count, &cycle_start);
      }

      self.state_cache.insert(
        current_state,
        StateInfo {
          rows: self.height(),
          rock_count,
        },
      );
    }

    self.height()
  }

  fn calculate_final_height(
    &mut self,
    target_rocks: i64,
    current_rocks: i64,
    cycle_start: &StateInfo,
  ) -> i64 {
    let rows_per_cycle = self.height() - cycle_start.rows;
    let rocks_per_cycle = current_rocks - cycle_start.rock_count;
    let remaining_rocks = target_rocks - current_rocks;
    let n_cycles = remaining_rocks / rocks_per_cycle;
    let leftover_rocks = remaining_rocks % rocks_per_cycle;

    let bottom_height = self.height();

    for _ in 0..leftover_rocks {
      self.simulate_rock();
    }

    let top_height = self.height() - bottom_height;

    bottom_height + (n_cycles * rows_per_cycle) + top_height
  }

  fn current_state(&self) -> ChamberState {
    ChamberState {
      rock_type: self.current_rock_type,
      wind_position: self.wind.current_index(),
      peaks: self.calculate_relative_peaks(),
    }
  }

  fn height(&self) -> i64 {
    self.rows.len() as i64
  }

  fn simulate_rock(&mut self) {
    let mut rock = self.spawn_rock();

    loop {
      let wind_direction = self.wind.next();
      self.try_push_rock(&mut rock, wind_direction);
      if !self.try_move_rock_down(&mut rock) {
        break;
      }
    }

    self.settle_rock(&rock);
  }

  fn calculate_relative_peaks(&self) -> [i64; 7] {
    if self.rows.is_empty() {
      return [-1; 7];
    }

    let mut peaks = [-1_i64; 7];
    let mut min_peak = i64::MAX;

    for col in 0..Self::CHAMBER_WIDTH {
      for row in (0..self.rows.len()).rev() {
        if self.rows[row][col] {
          peaks[col] = row as i64;
          break;
        }
      }
      min_peak = min_peak.min(peaks[col]);
    }

    for peak in &mut peaks {
      *peak -= min_peak;
    }

    peaks
  }

  fn spawn_rock(&mut self) -> Rock {
    let rock = Rock::new(
      self.current_rock_type,
      Self::INITIAL_X_OFFSET,
      self.height() + Self::SPAWN_HEIGHT_OFFSET,
    );
    self.current_rock_type = self.current_rock_type.next();
    rock
  }

  fn settle_rock(&mut self, rock: &Rock) {
    let max_y = rock
      .points
      .iter()
      .map(|point| point.y)
      .max()
      .expect("rock with no points?") as usize;

    while self.rows.len() <= max_y {
      self.rows.push([false; Self::CHAMBER_WIDTH]);
    }

    for point in &rock.points {
      let (x, y) = (*point).into();
      self.rows[y][x] = true;
    }
  }

  fn collides(&self, rock: &Rock) -> bool {
    rock.points.iter().any(|point| {
      point.x < 0
        || point.x >= Self::CHAMBER_WIDTH as i64
        || point.y < 0
        || (point.y < self.height() && {
          let (x, y) = (*point).into();
          self.rows[y][x]
        })
    })
  }

  fn try_move_rock_down(&self, rock: &mut Rock) -> bool {
    rock.move_down();
    if self.collides(rock) {
      rock.move_up();
      false
    } else {
      true
    }
  }

  fn try_push_rock(&self, rock: &mut Rock, direction: WindDirection) {
    match direction {
      WindDirection::Left => rock.move_left(),
      WindDirection::Right => rock.move_right(),
    }

    if self.collides(rock) {
      // reverse if rock collision
      match direction {
        WindDirection::Left => rock.move_right(),
        WindDirection::Right => rock.move_left(),
      }
    }
  }
}

fn parse_wind_directions(input: &str) -> Vec<WindDirection> {
  input.trim().chars().map(WindDirection::from).collect()
}

fn part_one(input: &str) -> i64 {
  let wind = parse_wind_directions(input);
  let mut chamber = Chamber::new(wind);
  chamber.height_after(2022)
}

fn part_two(input: &str) -> i64 {
  let wind = parse_wind_directions(input);
  let mut chamber = Chamber::new(wind);
  chamber.height_after(1_000_000_000_000)
}

fn main() {
  let input = include_str!("input");
  let start = Instant::now();
  println!("{}", part_one(input));
  println!("Part 1 time: {:?}", start.elapsed());

  let start = Instant::now();
  println!("{}", part_two(input));
  println!("Part 2 time: {:?}", start.elapsed());
}

#[cfg(test)]
mod test {
  use super::*;
  const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

  #[test]
  fn part_one_example() {
    assert_eq!(part_one(INPUT), 3068);
  }

  #[test]
  fn part_two_example() {
    assert_eq!(part_two(INPUT), 1514285714288);
  }

  #[test]
  fn next_rock() {
    let wind = parse_wind_directions(">>>><<<<>");
    let mut chamber = Chamber::new(wind);
    let rock_1 = chamber.spawn_rock();
    let expected_rock_1: Vec<Point> = vec![(2, 3), (3, 3), (4, 3), (5, 3)]
      .into_iter()
      .map(Into::into)
      .collect();
    assert_eq!(rock_1.points, expected_rock_1);
    let rock_2 = chamber.spawn_rock();
    let expected_rock_2: Vec<Point> = vec![(3, 3), (2, 4), (3, 4), (4, 4), (3, 5)]
      .into_iter()
      .map(Into::into)
      .collect();
    assert_eq!(rock_2.points, expected_rock_2);
    let rock_3 = chamber.spawn_rock();
    let expected_rock_3: Vec<Point> = vec![(2, 3), (3, 3), (4, 3), (4, 4), (4, 5)]
      .into_iter()
      .map(Into::into)
      .collect();
    assert_eq!(rock_3.points, expected_rock_3);
    let rock_4 = chamber.spawn_rock();
    let expected_rock_4: Vec<Point> = vec![(2, 3), (2, 4), (2, 5), (2, 6)]
      .into_iter()
      .map(Into::into)
      .collect();
    assert_eq!(rock_4.points, expected_rock_4);
    let rock_5 = chamber.spawn_rock();
    let expected_rock_5: Vec<Point> = vec![(2, 3), (3, 3), (2, 4), (3, 4)]
      .into_iter()
      .map(Into::into)
      .collect();
    assert_eq!(rock_5.points, expected_rock_5);
    let rock_6 = chamber.spawn_rock();
    assert_eq!(rock_6.points, expected_rock_1);
  }
}
