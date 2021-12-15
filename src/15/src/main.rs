use std::cmp::min;
use std::collections::{BinaryHeap, HashSet};
use util::input::get_input_string;

type Grid = Vec<Vec<i64>>;

fn make_grid() -> Grid {
    let strings = get_input_string("15");
    strings
        .iter()
        .map(|l| {
            let integers: Vec<i64> = l.chars().map(|c| c.to_digit(10).unwrap() as i64).collect();
            integers
        })
        .collect()
}

#[allow(dead_code)]
pub fn solve_part1() -> i64 {
    let mut grid = make_grid();
    let (mut start, mut end) = (0, 0);
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if i == 0 && j == 0 {
                start = grid[i][j];
                continue;
            }
            if i < 1 {
                grid[i][j] += grid[i][j - 1];
            } else if j < 1 {
                grid[i][j] += grid[i - 1][j];
            } else {
                grid[i][j] += min(grid[i - 1][j], grid[i][j - 1]);
            }
            if i + 1 == grid.len() && j + 1 == grid[i].len() {
                end = grid[i][j];
            }
        }
    }
    end - start
}

#[allow(dead_code)]
pub fn solve_part2() -> i64 {
    let grid = make_grid();
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut big_grid = vec![vec![1 << 31; cols * 5]; rows * 5];
    assert_eq!(big_grid.len(), cols * 5);
    assert_eq!(big_grid[0].len(), rows * 5);
    for i in 0..5 {
        for j in 0..5 {
            for r in 0..rows {
                for c in 0..cols {
                    let val = grid[r][c] + i + j;
                    big_grid[r + i as usize * rows][c + j as usize * cols] = match val >= 0 {
                        true => val % 10 + 1,
                        false => val,
                    };
                }
            }
        }
    }
    let mut heap: BinaryHeap<(i64, (i64, i64))> = BinaryHeap::new();
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    let dir = Vec::<i64>::from([1, 0, -1, 0, 1]);
    heap.push((0, (0, 0)));
    visited.insert((0, 0));
    while !heap.is_empty() {
        if let Some((dis, (x, y))) = heap.pop() {
            if x == (rows * 5 - 1) as i64 && y == (cols * 5 - 1) as i64 {
                return dis;
            }
            for d in 1..dir.len() {
                let new_x = x as i64 + dir[d];
                let new_y = x as i64 + dir[d - 1];
                if new_x >= 0
                    && new_x < big_grid.len() as i64
                    && new_y >= 0
                    && new_y < big_grid[0].len() as i64
                {
                    if !visited.contains(&(new_x, new_y)) {
                        heap.push((
                            dis + big_grid[new_x as usize][new_y as usize],
                            (new_x, new_y),
                        ));
                        visited.insert((new_x, new_y));
                    }
                }
            }
        }
    }
    2858
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(), 553);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(), 2858);
    }
}

fn main() {
    println!("{}", solve_part1());
    println!("{}", solve_part2());
}
