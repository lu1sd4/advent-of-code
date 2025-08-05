use nom::{
  bytes::complete::tag, character::complete::newline, combinator::map, multi::separated_list1,
  IResult, Parser,
};
use std::{time::Instant};

const L: usize = 22;

fn bounded_add(lhs: usize, rhs: usize, max: usize) -> Option<usize> {
  lhs.checked_add(rhs).filter(|&result| result < max)
}

struct Space {
  cells: [[[Cell; L]; L]; L],
}

impl Space {
  fn from(positions: Vec<Position>) -> Self {
    let mut cells = [[[Cell::new(Material::Air); L]; L]; L];

    for position in &positions {
      cells[position.x + 1][position.y + 1][position.z + 1] = Cell::new(Material::Droplet);
    }

    for x in 0..L {
      // calculate opposite contact surfaces
      for y in 0..L {
        for z in 0..L {
          for [adj_x, adj_y, adj_z] in Self::previous_adjacent_positions([x, y, z]) {
            if cells[x][y][z].material != cells[adj_x][adj_y][adj_z].material {
              cells[x][y][z].opposite_neighbors += 1;
              cells[adj_x][adj_y][adj_z].opposite_neighbors += 1;
            }
          }
        }
      }
    }

    Self { cells }
  }
  fn previous_adjacent_positions(position: [usize; 3]) -> impl Iterator<Item = [usize; 3]> {
    [
      position[0]
        .checked_sub(1)
        .map(|x| [x, position[1], position[2]]),
      position[1]
        .checked_sub(1)
        .map(|y| [position[0], y, position[2]]),
      position[2]
        .checked_sub(1)
        .map(|z| [position[0], position[1], z]),
    ]
    .into_iter()
    .flatten()
  }
  fn all_adjacent_positions(position: [usize; 3]) -> impl Iterator<Item = [usize; 3]> {
    [
      bounded_add(position[0], 1, L).map(|x| [x, position[1], position[2]]),
      bounded_add(position[1], 1, L).map(|y| [position[0], y, position[2]]),
      bounded_add(position[2], 1, L).map(|z| [position[0], position[1], z]),
      position[0]
        .checked_sub(1)
        .map(|x| [x, position[1], position[2]]),
      position[1]
        .checked_sub(1)
        .map(|y| [position[0], y, position[2]]),
      position[2]
        .checked_sub(1)
        .map(|z| [position[0], position[1], z]),
    ]
    .into_iter()
    .flatten()
  }
  fn droplet_sides(&self) -> u64 {
    self
      .cells
      .iter()
      .flatten()
      .flatten()
      .filter(|cell| cell.material == Material::Droplet)
      .map(|droplet| droplet.opposite_neighbors)
      .sum()
  }
  fn exterior_air_sides(&self) -> u64 {
    let mut stack = vec![[0, 0, 0]]; // assume 0,0,0 is air
    let mut visited = [[[false; L]; L]; L];
    let mut sum: u64 = 0;
    let search_material = Material::Air;
    while let Some([x, y, z]) = stack.pop() {
      let cell = self.cells[x][y][z];
      if cell.material == search_material && !visited[x][y][z] {
        visited[x][y][z] = true;
        sum += cell.opposite_neighbors;
        stack.extend(Self::all_adjacent_positions([x, y, z]));
      }
    }
    sum
  }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Material {
  Droplet,
  Air,
}

struct Position {
  x: usize,
  y: usize,
  z: usize,
}

#[derive(Debug, Clone, Copy)]
struct Cell {
  opposite_neighbors: u64,
  material: Material,
}

impl Cell {
  fn new(material: Material) -> Self {
    Self {
      opposite_neighbors: 0,
      material,
    }
  }
}

fn positions(input: &str) -> IResult<&str, Vec<Position>> {
  separated_list1(
    newline,
    map(
      (
        nom::character::complete::usize,
        tag(","),
        nom::character::complete::usize,
        tag(","),
        nom::character::complete::usize,
      ),
      |(x, _, y, _, z)| Position { x, y, z },
    ),
  )
  .parse(input)
}

fn part_one(input: &str) -> u64 {
  let (_, positions) = positions(input).unwrap();
  let space = Space::from(positions);
  space.droplet_sides()
}

fn part_two(input: &str) -> u64 {
  let (_, positions) = positions(input).unwrap();
  let space = Space::from(positions);
  space.exterior_air_sides()
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
  const INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

  #[test]
  fn part_one_example() {
    assert_eq!(part_one(INPUT), 64);
  }

  #[test]
  fn part_two_example() {
    assert_eq!(part_two(INPUT), 58);
  }
}
