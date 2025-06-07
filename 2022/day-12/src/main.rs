fn parse_hill_height_matrix(input: &str) -> Vec<Vec<char>> {
  input
    .lines()
    .map(str::chars)
    .map(Iterator::collect)
    .collect()
}

type Position = (usize, usize);

fn find_character_positions(grid: &Vec<Vec<char>>, target_char: char) -> Vec<(usize, usize)> {
  grid
    .iter()
    .enumerate()
    .flat_map(|(i, row)| {
      row.iter().enumerate().filter_map(
        move |(j, &c)| {
          if c == target_char {
            Some((i, j))
          } else {
            None
          }
        },
      )
    })
    .collect()
}

fn within_bounds<T>(grid: &Vec<Vec<T>>, position: (i32, i32)) -> bool {
  return position.0 >= 0
    && position.1 >= 0
    && (position.0 as usize) < grid.len()
    && (position.1 as usize) < grid[position.0 as usize].len();
}

fn legal_move(from: char, to: char) -> bool {
  if from.is_ascii_lowercase() && to.is_ascii_lowercase() {
    return (to as u32) <= (from as u32) + 1;
  }
  return from == 'S' || to == 'E' && (from == 'y' || from == 'z');
}

fn can_move(
  from: Position,
  to: (i32, i32),
  grid: &Vec<Vec<char>>,
  visited: &Vec<Vec<bool>>,
) -> bool {
  if within_bounds(grid, to) {
    let pos_to = (to.0 as usize, to.1 as usize);
    return legal_move(grid[from.0][from.1], grid[pos_to.0][pos_to.1])
      && !visited[pos_to.0][pos_to.1];
  }
  false
}

fn possible_moves(
  grid: &Vec<Vec<char>>,
  visited: &Vec<Vec<bool>>,
  position: Position,
) -> Vec<Position> {
  let deltas = [-1, 1];
  let mut moves: Vec<Position> = Vec::new();
  for delta in deltas {
    // vertical
    let p_pos = (position.0 as i32 + delta, position.1 as i32);
    if can_move(position, p_pos, grid, visited) {
      moves.push((p_pos.0 as usize, p_pos.1 as usize));
    }
    // horizontal
    let p_pos = (position.0 as i32, position.1 as i32 + delta);
    if can_move(position, p_pos, grid, visited) {
      moves.push((p_pos.0 as usize, p_pos.1 as usize));
    }
  }
  moves
}

fn shortest_path_length(grid: &Vec<Vec<char>>, start: &Position, target: &Position) -> Option<u32> {
  let mut visited: Vec<Vec<bool>> = grid.iter().map(|row| vec![false; row.len()]).collect();
  visited[start.0][start.1] = true;
  let mut depth = 0;
  let mut queue: Vec<Position> = Vec::new();
  queue.push(*start);
  while !queue.is_empty() {
    let mut new_queue: Vec<Position> = Vec::new();
    depth += 1;
    for (i, j) in queue.drain(..) {
      let mut moves = possible_moves(grid, &visited, (i, j));
      for (i_a, j_a) in moves.iter() {
        visited[*i_a][*j_a] = true;
        if *i_a == target.0 && *j_a == target.1 {
          return Some(depth);
        }
      }
      new_queue.append(&mut moves);
    }
    queue = new_queue;
  }
  None
}

fn part_one(input: &str) -> u32 {
  let grid = parse_hill_height_matrix(input);

  let start = *find_character_positions(&grid, 'S')
    .first()
    .expect("S not found");

  let end = *find_character_positions(&grid, 'E')
    .first()
    .expect("E not found");

  Option::expect(
    shortest_path_length(&grid, &start, &end),
    "could not reach end",
  )
}

fn part_two(input: &str) -> u32 {
  let grid = parse_hill_height_matrix(input);

  let s_start = *find_character_positions(&grid, 'S')
    .first()
    .expect("S not found");
  let a_starts = find_character_positions(&grid, 'a');

  let mut starts = vec![s_start];
  starts.extend(&a_starts);

  let end = *find_character_positions(&grid, 'E')
    .first()
    .expect("E not found");

  let res = starts
    .iter()
    .filter_map(|start| shortest_path_length(&grid, &start, &end))
    .min();

  Option::expect(res, "no end found")
}

fn main() {
  let input = include_str!("input");
  println!("{}", part_one(input));
  println!();
  println!("{}", part_two(input));
}

#[cfg(test)]
mod test {
  use super::*;
  const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

  #[test]
  fn part_one_example() {
    assert_eq!(part_one(INPUT), 31);
  }

  #[test]
  fn part_two_example() {
    assert_eq!(part_two(INPUT), 29);
  }
}
