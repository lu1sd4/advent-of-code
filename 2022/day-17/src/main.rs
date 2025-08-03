use std::{
  collections::HashMap,
  iter::{Cycle, Peekable},
  time::Instant,
  vec::IntoIter,
};

#[derive(Eq, Debug)]
struct ShapePoint {
  x: i64,
  y: i64,
}

impl PartialEq for ShapePoint {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == other.y
  }
}

impl From<(i64, i64)> for ShapePoint {
  fn from(tuple: (i64, i64)) -> Self {
    ShapePoint {
      x: tuple.0,
      y: tuple.1,
    }
  }
}

impl From<&ShapePoint> for (usize, usize) {
  fn from(point: &ShapePoint) -> Self {
    (
      point.x.try_into().expect("Negative x?"),
      point.y.try_into().expect("Negative y?"),
    )
  }
}

const HORIZONTAL_LINE_PATTERN: &[(i64, i64)] = &[(0, 0), (1, 0), (2, 0), (3, 0)];

const PLUS_PATTERN: &[(i64, i64)] = &[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)];

const INVERTED_L_PATTERN: &[(i64, i64)] = &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)];

const VERTICAL_LINE_PATTERN: &[(i64, i64)] = &[(0, 0), (0, 1), (0, 2), (0, 3)];

const SQUARE_PATTERN: &[(i64, i64)] = &[(0, 0), (1, 0), (0, 1), (1, 1)];

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum RockType {
  HorizontalLine,
  Plus,
  LShape,
  VerticalLine,
  Square,
}

impl RockType {
  fn pattern(&self) -> &'static [(i64, i64)] {
    match self {
      RockType::HorizontalLine => HORIZONTAL_LINE_PATTERN,
      RockType::Plus => PLUS_PATTERN,
      RockType::LShape => INVERTED_L_PATTERN,
      RockType::VerticalLine => VERTICAL_LINE_PATTERN,
      RockType::Square => SQUARE_PATTERN,
    }
  }
}

#[derive(Debug)]
struct Rock {
  points: Vec<ShapePoint>
}

impl Rock {
  fn new(rock_type: RockType, x_offset: i64, bottom_at: i64) -> Self {
    let points: Vec<ShapePoint> = rock_type
      .pattern()
      .iter()
      .map(|(x, y)| (x + x_offset, y + bottom_at))
      .map(Into::into)
      .collect();

    Self { points }
  }
  fn shift_down(&mut self) {
    self.points.iter_mut().for_each(|point| point.y -= 1);
  }
  fn shift_up(&mut self) {
    self.points.iter_mut().for_each(|point| point.y += 1);
  }
  fn shift_right(&mut self) {
    self.points.iter_mut().for_each(|point| point.x += 1);
  }
  fn shift_left(&mut self) {
    self.points.iter_mut().for_each(|point| point.x -= 1);
  }
}

#[derive(Clone, Copy)]
enum WindDirection {
  Left,
  Right,
}

struct Wind {
  wind_directions: Cycle<IntoIter<WindDirection>>,
  current_index: usize,
  len: usize,
}

impl Wind {
  fn new(directions: Vec<WindDirection>) -> Self {
    let len = directions.len();
    Self {
      wind_directions: directions.into_iter().cycle(),
      current_index: 0,
      len,
    }
  }

  fn next_direction(&mut self) -> WindDirection {
    self.current_index = (self.current_index + 1) % self.len;
    self
      .wind_directions
      .next()
      .expect("no next wind direction in cycle?")
  }

  fn next_index(&self) -> usize {
    self.current_index % self.len
  }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct ChamberState {
  rock_type: RockType,
  wind_position: usize,
  peaks: [i64; 7],
}

#[derive(Debug)]
struct StateHeights {
  rows: i64,
  rocks: i64,
}

fn parse_wind_directions(input: &str) -> Vec<WindDirection> {
  input
    .chars()
    .map(|c| {
      if c == '>' {
        WindDirection::Right
      } else {
        WindDirection::Left
      }
    })
    .collect()
}

struct Chamber {
  rows: Vec<[bool; 7]>,
  rocks_types: Peekable<Cycle<IntoIter<RockType>>>,
  wind: Wind,
  initial_x_offset: i64,
  state_cache: HashMap<ChamberState, StateHeights>,
}

impl Chamber {
  fn new(jet_pattern: Vec<WindDirection>) -> Self {
    Self {
      rows: Vec::new(),
      rocks_types: vec![
        RockType::HorizontalLine,
        RockType::Plus,
        RockType::LShape,
        RockType::VerticalLine,
        RockType::Square,
      ]
      .into_iter()
      .cycle()
      .peekable(),
      wind: Wind::new(jet_pattern),
      initial_x_offset: 2,
      state_cache: HashMap::new(),
    }
  }
  fn height_after(&mut self, n_rocks: i64) -> i64 {
    let mut current_state = ChamberState {
      rock_type: RockType::HorizontalLine,
      wind_position: self.wind.current_index,
      peaks: [0; 7],
    };
    self
      .state_cache
      .insert(current_state, StateHeights { rows: 0, rocks: 0 });
    let mut rock_count = 0;
    let mut loop_found = false;
    while rock_count < n_rocks && !loop_found {
      self.simulate_rock();
      rock_count += 1;
      current_state = self.make_current_state();
      if self.state_cache.contains_key(&current_state) {
        loop_found = true;
      } else {
        self.state_cache.insert(
          current_state,
          StateHeights {
            rows: self.rows_height(),
            rocks: rock_count,
          },
        );
      }
    }
    if !loop_found {
      return self.rows_height();
    }
    let pattern_start_heights = self
      .state_cache
      .get(&current_state)
      .expect("loop found = true means the state is in the map");
    let bottom_height = self.rows_height();
    let pattern_height_rows = bottom_height - pattern_start_heights.rows;
    let pattern_rocks = rock_count - pattern_start_heights.rocks;
    let rocks_after_pattern_found = n_rocks - rock_count;
    let patterns_repeat = rocks_after_pattern_found / pattern_rocks;
    let total_pattern_height = patterns_repeat * pattern_height_rows;
    let remaining_rocks = rocks_after_pattern_found % pattern_rocks;
    for _ in 0..remaining_rocks {
      self.simulate_rock();
    }
    let top_height = self.rows_height() - bottom_height;
    bottom_height + total_pattern_height + top_height
  }
  fn make_current_state(&mut self) -> ChamberState {
    ChamberState {
      rock_type: self
        .rocks_types
        .peek()
        .expect("no next rock in cycle?")
        .clone(),
      wind_position: self.wind.next_index(),
      peaks: self.calculate_peaks(),
    }
  }
  fn rows_height(&self) -> i64 {
    self
      .rows
      .len()
      .try_into()
      .expect("cant cast rows.len() into i64?")
  }
  fn simulate_rock(&mut self) {
    let mut rock = self.next_rock();
    let mut can_fall = true;
    while can_fall {
      let jet = self.wind.next_direction();
      self.try_push_step(&mut rock, jet);
      can_fall = self.try_fall(&mut rock);
    }
    self.settle_rock(&rock);
  }
  fn calculate_peaks(&self) -> [i64; 7] {
    let mut peaks = [-1_i64; 7];
    let mut min_peak = i64::MAX;
    for col in 0..self.rows[0].len() {
      let mut current_row = Some(self.rows.len() - (1));
      while let Some(row) = current_row {
        if self.rows[row][col] {
          break;
        }
        current_row = row.checked_sub(1)
      }
      peaks[col] = current_row.map_or(-1, |row| row.try_into().expect("cant usize -> i64"));
      min_peak = min_peak.min(peaks[col]);
    }
    peaks.iter_mut().for_each(|v| *v = *v - min_peak);
    peaks
  }
  fn next_rock(&mut self) -> Rock {
    Rock::new(
      self.rocks_types.next().expect("No next rock in cycle?"),
      self.initial_x_offset,
      3 + (self.rows.len() as i64),
    )
  }
  fn settle_rock(&mut self, rock: &Rock) {
    let max_y = usize::try_from(
      rock
        .points
        .iter()
        .map(|point| point.y)
        .max()
        .expect("no max y?"),
    )
    .expect("max y cant be usize?");
    let required_length = max_y + 1;
    while self.rows.len() < required_length {
      self.rows.push([false; 7])
    }
    rock
      .points
      .iter()
      .map(|point| point.into())
      .for_each(|(x, y)| {
        self.rows[y][x] = true;
      });
  }
  fn rock_collides(&self, rock: &Rock) -> bool {
    for point in rock.points.iter() {
      if point.x < 0 {
        return true;
      }
      if point.x >= 7 {
        return true;
      }
      if point.y < 0 {
        return true;
      }
      if point.y >= (self.rows.len() as i64) {
        continue;
      }
      let indexes: (usize, usize) = point.into();
      if self.rows[indexes.1][indexes.0] {
        return true;
      }
    }
    return false;
  }
  fn try_fall(&self, rock: &mut Rock) -> bool {
    rock.shift_down();
    if self.rock_collides(&rock) {
      rock.shift_up();
      return false;
    }
    return true;
  }
  fn try_push_step(&self, rock: &mut Rock, wind_direction: WindDirection) {
    match wind_direction {
      WindDirection::Left => {
        rock.shift_left();
      }
      WindDirection::Right => {
        rock.shift_right();
      }
    }
    if self.rock_collides(&rock) {
      // reverse if rock collision
      match wind_direction {
        WindDirection::Left => {
          rock.shift_right();
        }
        WindDirection::Right => {
          rock.shift_left();
        }
      }
    }
  }
}

fn part_one(input: &str) -> i64 {
  let wind = parse_wind_directions(input);
  let mut chamber = Chamber::new(wind);
  chamber.height_after(2022)
}

fn part_two(input: &str) -> i64 {
  let wind = parse_wind_directions(input);
  let mut chamber = Chamber::new(wind);
  chamber.height_after(1000000000000)
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
    let rock_1 = chamber.next_rock();
    let expected_rock_1: Vec<ShapePoint> = vec![(2, 3), (3, 3), (4, 3), (5, 3)]
      .into_iter()
      .map(Into::into)
      .collect();
    assert_eq!(rock_1.points, expected_rock_1);
    let rock_2 = chamber.next_rock();
    let expected_rock_2: Vec<ShapePoint> = vec![(3, 3), (2, 4), (3, 4), (4, 4), (3, 5)]
      .into_iter()
      .map(Into::into)
      .collect();
    assert_eq!(rock_2.points, expected_rock_2);
    let rock_3 = chamber.next_rock();
    let expected_rock_3: Vec<ShapePoint> = vec![(2, 3), (3, 3), (4, 3), (4, 4), (4, 5)]
      .into_iter()
      .map(Into::into)
      .collect();
    assert_eq!(rock_3.points, expected_rock_3);
    let rock_4 = chamber.next_rock();
    let expected_rock_4: Vec<ShapePoint> = vec![(2, 3), (2, 4), (2, 5), (2, 6)]
      .into_iter()
      .map(Into::into)
      .collect();
    assert_eq!(rock_4.points, expected_rock_4);
    let rock_5 = chamber.next_rock();
    let expected_rock_5: Vec<ShapePoint> = vec![(2, 3), (3, 3), (2, 4), (3, 4)]
      .into_iter()
      .map(Into::into)
      .collect();
    assert_eq!(rock_5.points, expected_rock_5);
    let rock_6 = chamber.next_rock();
    assert_eq!(rock_6.points, expected_rock_1);
  }

  #[test]
  fn simulate_rock() {
    let wind = parse_wind_directions(">");
    let mut chamber = Chamber::new(wind);
    chamber.simulate_rock();
    let expected_1 = "...####";
    assert_eq!(chamber.draw(), expected_1);
    for _ in 0..4 {
      chamber.simulate_rock();
    }
    let expected_2 = ".....##
.....##
......#
......#
......#
......#
......#
......#
....###
.....#.
....###
.....#.
...####";
    assert_eq!(chamber.draw(), expected_2);
  }
}
