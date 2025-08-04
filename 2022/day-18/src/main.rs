use nom::{
  bytes::complete::tag, character::complete::newline, combinator::map, multi::separated_list1,
  IResult, Parser,
};
use std::{collections::HashSet, time::Instant};

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

    for position in positions {
      cells[position.x + 1][position.y + 1][position.z + 1] = Cell::new(Material::Droplet);
    }

    let mut updates = Vec::new();

    for x in 0..L {
      // calculate opposite contact surfaces
      for y in 0..L {
        for z in 0..L {
          for [adj_x, adj_y, adj_z] in Self::all_adjacent_positions([x, y, z]) {
            if cells[x][y][z].material != cells[adj_x][adj_y][adj_z].material {
              updates.push((x, y, z));
            }
          }
        }
      }
    }

    for (x, y, z) in updates {
      cells[x][y][z].opposite_neighbors += 1;
    }

    Self { cells }
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
      .iter() // rows
      .flatten() // cols
      .flatten() // layers
      .filter(|cell| cell.material == Material::Droplet)
      .map(|droplet| droplet.opposite_neighbors)
      .sum()
  }
  fn connected_positions_same_material(&self, start: [usize; 3]) -> HashSet<[usize; 3]> {
    let mut stack = vec![start];
    let mut result = HashSet::new();
    let search_material = self.cells[start[0]][start[1]][start[2]].material;
    stack.push(start);
    while let Some([x, y, z]) = stack.pop() {
      let cell = self.cells[x][y][z];
      if cell.material == search_material && result.insert([x, y, z]) {
        stack.extend(Self::all_adjacent_positions([x, y, z]).filter(|pos| !result.contains(pos)));
      }
    }
    result
  }
  fn exterior_air_sides(&self) -> u64 {
    let exterior_air_cells = self.connected_positions_same_material([0, 0, 0]);
    exterior_air_cells
      .iter()
      .map(|&[x, y, z]| self.cells[x][y][z].opposite_neighbors)
      .sum()
  }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Material {
  Droplet,
  Air,
}

#[derive(Clone, Copy)]
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
