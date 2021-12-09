use std::collections::HashSet;
use util::input::{get_input_string, make_grid, Grid, GridCell};

#[allow(dead_code)]
pub fn solve_part1() -> i64 {
    let strings = get_input_string("09");
    let grid = make_grid(&strings);
    let m: i64 = grid.len().try_into().unwrap();
    let n: i64 = grid[0].len().try_into().unwrap();
    let dir = Vec::<i64>::from([1, 0, -1, 0, 1]);

    let mut level_total = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let mut greater = false;
            for d in 1..dir.len() {
                let new_x = i as i64 + dir[d];
                let new_y = j as i64 + dir[d - 1];
                if new_x >= 0 && new_x < m && new_y >= 0 && new_y < n {
                    greater |= grid[i][j] >= grid[new_x as usize][new_y as usize];
                }
            }
            if !greater {
                level_total += grid[i][j] + 1;
            }
        }
    }
    level_total
}

#[allow(dead_code)]
fn flood_fill(grid: &Grid, loc: GridCell, seen: &mut HashSet<GridCell>) -> i64 {
    let (i, j) = loc;
    if !(i < grid.len() && j < grid[0].len()) || seen.contains(&loc) || grid[i][j] == 9 {
        return 0;
    }
    seen.insert(loc);
    let mut total = 0;
    if i != 0 {
        total += flood_fill(grid, (i - 1, j), seen);
    }
    if j != 0 {
        total += flood_fill(grid, (i, j - 1), seen)
    }
    flood_fill(grid, (i + 1, j), seen) + flood_fill(grid, (i, j + 1), seen) + total + 1
}

#[allow(dead_code)]
fn top_three(list: &Vec<i64>) -> i64 {
    let mut top_three = Vec::<i64>::from([0; 3]);
    for &count in list.iter() {
        if count > top_three[0] {
            top_three[2] = top_three[1];
            top_three[1] = top_three[0];
            top_three[0] = count;
        } else if count > top_three[1] {
            top_three[2] = top_three[1];
            top_three[1] = count;
        } else if count > top_three[2] {
            top_three[2] = count;
        }
    }
    top_three[0] * top_three[1] * top_three[2]
}

#[allow(dead_code)]
pub fn solve_part2() -> i64 {
    let strings = get_input_string("09");
    let grid = make_grid(&strings);

    let mut seen: HashSet<GridCell> = HashSet::new();
    let mut results: Vec<i64> = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if !seen.contains(&(i, j)) {
                results.push(flood_fill(&grid, (i, j), &mut seen));
            }
        }
    }
    top_three(&results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(), 512);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(), 1600104);
    }
}

fn main() {
    println!("{}", solve_part1());
    println!("{}", solve_part2());
}
