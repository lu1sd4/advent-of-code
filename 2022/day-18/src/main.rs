use nom::{
  bytes::complete::tag, character::complete::newline, combinator::map, multi::separated_list1,
  IResult, Parser,
};
use std::{collections::HashSet, time::Instant};

const L: usize = 22;

fn bounded_add(lhs: usize, rhs: usize, max: usize) -> Option<usize> {
  let result = lhs.checked_add(rhs)?;
  if result < max {
    Some(result)
  } else {
    None
  }
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
          for [adj_x, adj_y, adj_z] in Space::all_adjacent_positions([x, y, z]) {
            if &cells[x][y][z].material != &cells[adj_x][adj_y][adj_z].material {
              updates.push((x, y, z));
            }
          }
        }
      }
    }

    for (x, y, z) in &mut updates {
      cells[*x][*y][*z].opposite_neighbors += 1;
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
      .flat_map(|col| col.iter().flat_map(|layer| layer.iter()))
      .filter(|cell| cell.material == Material::Droplet)
      .map(|droplet| droplet.opposite_neighbors)
      .sum()
  }
  fn connected_positions_same_material(&self, start: [usize; 3]) -> HashSet<[usize; 3]> {
    let mut stack: Vec<[usize; 3]> = Vec::new();
    let mut result: HashSet<[usize; 3]> = HashSet::new();
    let search_material = self.cells[start[0]][start[1]][start[2]].material;
    stack.push(start);
    while !stack.is_empty() {
      let [x, y, z] = stack.pop().expect("stack is empty?");
      let cell = self.cells[x][y][z];
      if cell.material == search_material {
        result.insert([x, y, z]);
        for [adj_x, adj_y, adj_z] in Space::all_adjacent_positions([x, y, z]) {
          if !result.contains(&[adj_x, adj_y, adj_z]) {
            stack.push([adj_x, adj_y, adj_z]);
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
      .map(|[x, y, z]| &self.cells[*x][*y][*z])
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
