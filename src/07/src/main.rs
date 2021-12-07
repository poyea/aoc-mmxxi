use std::cmp::min;
use util::input::{get_input_string, split_string};

#[allow(dead_code)]
pub fn solve_part1() -> i64 {
    let strings = get_input_string("07");
    let positions: Vec<i64> = split_string(",", strings.first().unwrap());
    let mut min_of_sum = i64::MAX;
    for i in 0..(positions.len() as i64) {
        let mut sum = 0;
        for j in 0..positions.len() {
            let diff = (i - positions[j]).abs();
            sum += diff;
        }
        min_of_sum = min(min_of_sum, sum);
    }
    min_of_sum
}

#[allow(dead_code)]
pub fn solve_part2() -> i64 {
    let strings = get_input_string("07");
    let positions: Vec<i64> = split_string(",", strings.first().unwrap());
    let mut min_of_sum = i64::MAX;
    for i in 0..(positions.len() as i64) {
        let mut sum = 0;
        for j in 0..positions.len() {
            let diff = (i - positions[j]).abs();
            sum += diff * (diff + 1) / 2;
        }
        min_of_sum = min(min_of_sum, sum);
    }
    min_of_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(), 353800);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(), 98119739);
    }
}

fn main() {
    println!("{}", solve_part1());
    println!("{}", solve_part2());
}
