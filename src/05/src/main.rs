use std::cmp;
use std::collections::HashMap;
use util::input::{get_input_string, split_string};

type Pair = Vec<i64>;

fn count_map(map: &HashMap<i64, i64>) -> i64 {
    let mut total = 0;
    for (_, &count) in map {
        if count >= 2 {
            total += 1;
        }
    }
    total
}

fn solve(diagonal: bool) -> i64 {
    let strings = get_input_string("05");
    let mut board = HashMap::new();
    for line in &strings {
        let points: Vec<Pair> = line.split(" -> ").map(|x| split_string(",", x)).collect();
        let (x1, y1) = (&points[0][0], &points[0][1]);
        let (x2, y2) = (&points[1][0], &points[1][1]);
        let (min_x, max_x) = (*cmp::min(x1, x2), *cmp::max(x1, x2));
        let (min_y, max_y) = (*cmp::min(y1, y2), *cmp::max(y1, y2));
        if y1 == y2 {
            for i in min_x..=max_x {
                *board.entry((i * 0x1f1f1f1f) ^ y1).or_insert(0) += 1;
            }
        } else if x1 == x2 {
            for i in min_y..=max_y {
                *board.entry((x1 * 0x1f1f1f1f) ^ i).or_insert(0) += 1;
            }
        } else if y1 != y2 && x1 != x2 && diagonal {
            let dx = match x1 < x2 {
                true => 1,
                false => -1,
            };
            let dy = match y1 < y2 {
                true => 1,
                false => -1,
            };
            for i in 0..=((x1 - x2).abs()) {
                *board
                    .entry(((i * dx + x1) * 0x1f1f1f1f) ^ (i * dy + y1))
                    .or_insert(0) += 1;
            }
        }
    }
    count_map(&board)
}

#[allow(dead_code)]
pub fn solve_part1() -> i64 {
    solve(false)
}

#[allow(dead_code)]
pub fn solve_part2() -> i64 {
    solve(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(), 8111);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(), 22088);
    }
}

fn main() {
    println!("{}", solve_part1());
    println!("{}", solve_part2());
}
