use util::input::get_input_string;

pub type Grid = Vec<Vec<char>>;

pub fn make_grid(strings: &Vec<String>) -> Grid {
    let mut grid: Grid = Vec::new();
    for line in strings {
        let mut line_vec = Vec::new();
        for c in line.chars() {
            line_vec.push(c);
        }
        grid.push(line_vec);
    }
    grid
}

#[allow(dead_code)]
pub fn solve_part1() -> i64 {
    let strings = get_input_string("25");
    let mut grid = make_grid(&strings);
    let m = grid.len();
    let n = grid[0].len();
    let mut step = 1;
    loop {
        let mut ok = true;
        for ((dx, dy), symbol) in [((0, 1), '>'), ((1, 0), 'v')] {
            let mut g = grid.clone();
            for i in 0..m {
                for j in 0..n {
                    if grid[i][j] == symbol && grid[(i + dx) % m][(j + dy) % n] == '.' {
                        g[(i + dx) % m][(j + dy) % n] = symbol;
                        g[i][j] = '.';
                        ok = false;
                    }
                }
            }
            grid = g;
        }
        if ok {
            return step;
        }
        step += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(), 360);
    }
}

fn main() {
    println!("{}", solve_part1());
}
