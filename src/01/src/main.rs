use util::input::get_input_integer;

#[allow(dead_code)]
pub fn solve_part1() -> i64 {
    let numbers: Vec<i64> = get_input_integer("01");
    let mut count = 0;
    for window in numbers.windows(2) {
        if window[1] > window[0] {
            count += 1;
        }
    }
    count
}

#[allow(dead_code)]
pub fn solve_part2() -> i64 {
    let numbers: Vec<i64> = get_input_integer("01");
    let mut count = 0;
    for window in numbers.windows(4) {
        if window[3] > window[0] {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(), 1709);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(), 1761);
    }
}

fn main() {
    println!("{}", solve_part1());
    println!("{}", solve_part2());
}
