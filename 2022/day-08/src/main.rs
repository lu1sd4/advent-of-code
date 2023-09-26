use std::ops::{Index, IndexMut};

enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Default)]
struct VisibilityThreshold {
    left: u32,
    right: u32,
    top: u32,
    bottom: u32,
}

impl Index<Direction> for VisibilityThreshold {
    type Output = u32;
    fn index(&self, direction: Direction) -> &Self::Output {
        match direction {
            Direction::Left => &self.left,
            Direction::Right => &self.right,
            Direction::Top => &self.top,
            Direction::Bottom => &self.bottom,
        }
    }
}

impl IndexMut<Direction> for VisibilityThreshold {
    fn index_mut(&mut self, direction: Direction) -> &mut Self::Output {
        match direction {
            Direction::Left => &mut self.left,
            Direction::Right => &mut self.right,
            Direction::Top => &mut self.top,
            Direction::Bottom => &mut self.bottom,
        }
    }
}

impl VisibilityThreshold {
    fn new() -> Self {
        Default::default()
    }
    fn allows_viewing(&self, height: u32) -> bool {
        return height > self.left
            || height > self.right
            || height > self.top
            || height > self.bottom;
    }
}

struct Forest {
    grid: Vec<Vec<u32>>,
    visibilities: Vec<Vec<VisibilityThreshold>>,
}

impl Forest {
    fn from_lines(input: &str) -> Self {
        let grid: Vec<Vec<u32>> = input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() + 1).collect())
            .collect();
        let visibilities = Self::calculate_visibilities(&grid);
        Self { grid, visibilities }
    }
    fn calculate_visibilities(grid: &Vec<Vec<u32>>) -> Vec<Vec<VisibilityThreshold>> {
        let mut visibilities: Vec<Vec<VisibilityThreshold>> = Vec::new();
        let rows = grid.len();
        let cols = grid[0].len();

        for i in 0..rows {
            let mut vis_row: Vec<VisibilityThreshold> = Vec::new();
            let vis_prev = visibilities.get(i.wrapping_sub(1));

            for j in 0..cols {
                let mut current_threshold = VisibilityThreshold::new();

                current_threshold[Direction::Left] = match vis_row.get(j.wrapping_sub(1)) {
                    Some(threshold) => threshold[Direction::Left].max(grid[i][j - 1]),
                    None => 0,
                };

                current_threshold[Direction::Top] = match vis_prev {
                    Some(top_row) => top_row[j][Direction::Top].max(grid[i - 1][j]),
                    None => 0,
                };

                vis_row.push(current_threshold);
            }
            visibilities.push(vis_row);
        }

        for i in (0..rows).rev() {
            for j in (0..cols).rev() {
                visibilities[i][j][Direction::Right] = match visibilities[i].get(j.wrapping_add(1))
                {
                    Some(threshold) => threshold[Direction::Right].max(grid[i][j + 1]),
                    None => 0,
                };

                visibilities[i][j][Direction::Bottom] = match visibilities.get(i.wrapping_add(1)) {
                    Some(vis_row) => vis_row[j][Direction::Bottom].max(grid[i + 1][j]),
                    None => 0,
                };
            }
        }

        visibilities
    }
    fn is_visible(&self, i: usize, j: usize) -> bool {
        return self.visibilities[i][j].allows_viewing(self.grid[i][j]);
    }
    fn number_visibles(&self) -> u32 {
        let mut count = 0;
        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                if self.is_visible(i, j) {
                    count += 1;
                }
            }
        }
        count
    }
    fn scenic_score(&self, row: usize, col: usize) -> u32 {
        let mut left = 0;
        let mut right = 0;
        let mut top = 0;
        let mut bottom = 0;

        for j in (0..col).rev() {
            left += 1;
            if self.grid[row][j] >= self.grid[row][col] {
                break;
            }
        }

        for j in col + 1..self.grid[row].len() {
            right += 1;
            if self.grid[row][j] >= self.grid[row][col] {
                break;
            }
        }

        for i in (0..row).rev() {
            top += 1;
            if self.grid[i][col] >= self.grid[row][col] {
                break;
            }
        }

        for i in row + 1..self.grid.len() {
            bottom += 1;
            if self.grid[i][col] >= self.grid[row][col] {
                break;
            }
        }

        return left * right * top * bottom;
    }
    fn best_scenic_score(&self) -> u32 {
        let mut best = 0;
        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                best = best.max(self.scenic_score(i, j));
            }
        }
        best
    }
}

fn main() {
    let file_contents = include_str!("input");
    part_one(file_contents);
    println!();
    part_two(file_contents);
}

fn part_one(file_contents: &str) {
    let forest = Forest::from_lines(file_contents);
    println!("{}", forest.number_visibles());
}

fn part_two(file_contents: &str) {
    let forest = Forest::from_lines(file_contents);
    println!("{}", forest.best_scenic_score());
}
