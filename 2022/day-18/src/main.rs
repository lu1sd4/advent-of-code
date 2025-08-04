use nom::{
  bytes::complete::tag, character::complete::newline, combinator::map, multi::separated_list1,
  IResult, Parser,
};
use std::{collections::HashSet, time::Instant};

struct Space {
  slots: Vec<Vec<Vec<Option<Cell>>>>,
  dimensions: [usize; 3],
}

impl Space {
  fn from(positions: Vec<Position>) -> Self {
    let max_x = positions.iter().map(|d| d.x).max().unwrap_or(0) + 1 + 2;
    let max_y = positions.iter().map(|d| d.y).max().unwrap_or(0) + 1 + 2;
    let max_z = positions.iter().map(|d| d.z).max().unwrap_or(0) + 1 + 2;

    let dimensions = [max_x, max_y, max_z];

    let mut slots: Vec<Vec<Vec<Option<Cell>>>> = vec![vec![vec![None; max_z]; max_y]; max_x];

    for position in positions {
      slots[position.x + 1][position.y + 1][position.z + 1] =
        Some(Cell::from(position.offset_by(1), Material::Droplet));
    }

    for x in 0..max_x {
      for y in 0..max_y {
        for z in 0..max_z {
          if let None = slots[x][y][z] {
            slots[x][y][z] = Some(Cell::new(x, y, z, Material::Air));
          }
        }
      }
    }

    let mut updates = Vec::new();

    for x in 0..max_x {
      // calculate opposite contact surfaces
      for y in 0..max_y {
        for z in 0..max_z {
          if let Some(cell) = &slots[x][y][z] {
            for [adj_x, adj_y, adj_z] in cell.all_adjacent_positions() {
              if adj_x < max_x && adj_y < max_y && adj_z < max_z {
                if let Some(adj_cell) = &slots[adj_x][adj_y][adj_z] {
                  if cell.material != adj_cell.material {
                    updates.push((x, y, z));
                  }
                }
              }
            }
          }
        }
      }
    }

    for (x, y, z) in updates {
      if let Some(cell) = slots[x][y][z].as_mut() {
        cell.opposite_neighbors += 1;
      }
    }

    Self { slots, dimensions }
  }
  fn droplet_sides(&self) -> u64 {
    self
      .slots
      .iter() // rows
      .flat_map(|col| col.iter().flat_map(|layer| layer.iter()))
      .filter_map(|cell| cell.as_ref())
      .filter(|cell| cell.material == Material::Droplet)
      .map(|droplet| droplet.opposite_neighbors)
      .sum()
  }
  fn connected_positions_same_material(&self, start: [usize; 3]) -> HashSet<[usize; 3]> {
    let mut stack: Vec<[usize; 3]> = Vec::new();
    let mut result: HashSet<[usize; 3]> = HashSet::new();
    let search_material = self.slots[start[0]][start[1]][start[2]]
      .expect("no cell at start position?")
      .material;
    let [max_x, max_y, max_z] = self.dimensions;
    stack.push(start);
    while !stack.is_empty() {
      let [x, y, z] = stack.pop().expect("stack is empty?");
      if let Some(cell) = &self.slots[x][y][z] {
        if cell.material == search_material {
          result.insert([cell.x, cell.y, cell.z]);
          for [adj_x, adj_y, adj_z] in cell.all_adjacent_positions() {
            if adj_x < max_x
              && adj_y < max_y
              && adj_z < max_z
              && !result.contains(&[adj_x, adj_y, adj_z])
            {
              stack.push([adj_x, adj_y, adj_z]);
            }
          }
        }
      }
    }
    result
  }
  fn exterior_air_sides(&self) -> u64 {
    let exterior_air_cells = self.connected_positions_same_material([0, 0, 0]);
    exterior_air_cells
      .iter()
      .map(|[x, y, z]| &self.slots[*x][*y][*z])
      .filter_map(|cell| cell.as_ref())
      .map(|air_block| air_block.opposite_neighbors)
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

impl Position {
  fn offset_by(&self, amount: usize) -> Position {
    return Position {
      x: self.x + amount,
      y: self.y + amount,
      z: self.z + amount,
    };
  }
}

#[derive(Debug, Clone, Copy)]
struct Cell {
  x: usize,
  y: usize,
  z: usize,
  opposite_neighbors: u64,
  material: Material,
}

impl Cell {
  fn new(x: usize, y: usize, z: usize, material: Material) -> Self {
    Self {
      x,
      y,
      z,
      opposite_neighbors: 0,
      material,
    }
  }
  fn from(position: Position, material: Material) -> Self {
    Self::new(position.x, position.y, position.z, material)
  }
  fn previous_adjacent_positions(&self) -> Vec<[usize; 3]> {
    let mut positions: Vec<[usize; 3]> = Vec::new();
    if let Some(adjacent_x) = self.x.checked_sub(1) {
      positions.push([adjacent_x, self.y, self.z]);
    }
    if let Some(adjacent_y) = self.y.checked_sub(1) {
      positions.push([self.x, adjacent_y, self.z]);
    }
    if let Some(adjacent_z) = self.z.checked_sub(1) {
      positions.push([self.x, self.y, adjacent_z]);
    }
    positions
  }
  fn all_adjacent_positions(&self) -> Vec<[usize; 3]> {
    let mut positions: Vec<[usize; 3]> = Vec::new();
    if let Some(adjacent_x) = self.x.checked_add(1) {
      positions.push([adjacent_x, self.y, self.z]);
    }
    if let Some(adjacent_y) = self.y.checked_add(1) {
      positions.push([self.x, adjacent_y, self.z]);
    }
    if let Some(adjacent_z) = self.z.checked_add(1) {
      positions.push([self.x, self.y, adjacent_z]);
    }
    positions.append(&mut self.previous_adjacent_positions());
    positions
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
