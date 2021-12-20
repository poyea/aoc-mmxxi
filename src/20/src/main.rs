use util::input::get_input_string;

type Grid = Vec<Vec<i64>>;

fn make_grid(image: &[String]) -> Grid {
    let mut v = Vec::new();
    for row in image {
        let mut r = Vec::new();
        for ch in row.chars() {
            let grid_val = match ch {
                '#' => 1,
                '.' => 0,
                _ => panic!("Something went wrong"),
            };
            r.push(grid_val);
        }
        v.push(r);
    }
    v
}

fn add_padding(grid: &Grid, pad_val: i64) -> Vec<Vec<i64>> {
    let m = grid.len();
    let padded_m = m + 6;
    let n = grid[0].len();
    let padded_n = n + 6;
    let mut v = Vec::from(vec![vec![pad_val; padded_n]; padded_m]);
    let mut cnt = 0;
    for i in 0..padded_m {
        for j in 0..padded_n {
            if 3 <= i && i < padded_m - 3 && 3 <= j && j < padded_n - 3 {
                cnt += 1;
                v[i][j] = grid[i - 3][j - 3];
            }
        }
    }
    assert_eq!(cnt, m * n);
    v
}

fn enhance(grid: &Grid, enhancement: &Vec<i64>, pad_val: i64) -> Vec<Vec<i64>> {
    let fix_grid = add_padding(&grid, pad_val);
    let m = fix_grid.len();
    let n = fix_grid[0].len();
    let mut v = Vec::from(vec![vec![0; n]; m]);
    for i in 0..(m - 2) {
        for j in 0..(n - 2) {
            let mut enhancement_index = 0;
            for k in i..(i + 3) {
                for l in j..(j + 3) {
                    enhancement_index <<= 1;
                    enhancement_index |= fix_grid[k][l];
                }
            }
            v[i + 1][j + 1] = enhancement[enhancement_index as usize];
        }
    }
    let mut mini_v = Vec::from(vec![vec![0; n - 4]; m - 4]);
    for i in 2..(m - 2) {
        for j in 2..(n - 2) {
            mini_v[i - 2][j - 2] = v[i][j];
        }
    }
    mini_v
}

#[allow(dead_code)]
pub fn solve_part1() -> i64 {
    let strings = get_input_string("20");
    let enhancement = strings
        .first()
        .unwrap()
        .chars()
        .map(|ch| match ch {
            '#' => 1,
            '.' => 0,
            _ => panic!("Something went wrong"),
        })
        .collect::<Vec<_>>();
    let mut v = make_grid(&strings[2..]);
    for i in 0..=1 {
        v = enhance(&v, &enhancement, i & 1);
    }
    v.iter().map(|x| -> i64 { x.iter().sum() }).sum::<i64>()
}

#[allow(dead_code)]
pub fn solve_part2() -> i64 {
    let strings = get_input_string("20");
    let enhancement = strings
        .first()
        .unwrap()
        .chars()
        .map(|ch| match ch {
            '#' => 1,
            '.' => 0,
            _ => panic!("Something went wrong"),
        })
        .collect::<Vec<_>>();
    let grid = make_grid(&strings[2..]);
    let mut v = enhance(&grid, &enhancement, 0);
    for i in 1..50 {
        v = enhance(&v, &enhancement, i & 1);
    }
    v.iter().map(|x| -> i64 { x.iter().sum() }).sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(), 5097);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(), 17987);
    }
}

fn main() {
    println!("{}", solve_part1());
    println!("{}", solve_part2());
}
