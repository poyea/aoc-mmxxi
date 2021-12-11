use util::input::{get_input_string, make_grid, Grid};

fn flash(grid: &mut Grid, i: usize, j: usize, count: &mut i64) {
    let m: i64 = grid.len().try_into().unwrap();
    let n: i64 = grid[0].len().try_into().unwrap();
    for x in -1..=1 {
        for y in -1..=1 {
            let new_x = i as i64 + x;
            let new_y = j as i64 + y;
            if !(x == 0 && y == 0) && 0 <= new_x && new_x < m && 0 <= new_y && new_y < n {
                let x_idx = new_x as usize;
                let y_idx = new_y as usize;
                if grid[x_idx][y_idx] != 0 {
                    grid[x_idx][y_idx] += 1;
                    if grid[x_idx][y_idx] > 9 {
                        grid[x_idx][y_idx] = 0;
                        *count += 1;
                        flash(grid, x_idx, y_idx, count);
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
pub fn solve_part1() -> i64 {
    let strings = get_input_string("11");
    let mut grid = make_grid(&strings);
    let mut count = 0;
    for _ in 1..=100 {
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                grid[i][j] += 1;
            }
        }
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] > 9 {
                    grid[i][j] = 0;
                    count += 1;
                    flash(&mut grid, i, j, &mut count);
                }
            }
        }
    }
    count
}

#[allow(dead_code)]
pub fn solve_part2() -> i64 {
    let strings = get_input_string("11");
    let mut grid = make_grid(&strings);
    for step in 1..10000 {
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                grid[i][j] += 1;
            }
        }
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] > 9 {
                    grid[i][j] = 0;
                    flash(&mut grid, i, j, &mut 0);
                }
            }
        }
        let mut non_zero = false;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] != 0 {
                    non_zero = true;
                }
            }
        }
        if !non_zero {
            return step;
        }
    }
    panic!("Something went wrong with the input")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(), 1735);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(), 400);
    }
}

fn main() {
    println!("{}", solve_part1());
    println!("{}", solve_part2());
}
