use util::dummy::new_year;
use util::input::get_input_string;

#[allow(dead_code)]
pub fn solve_part1() -> i64 {
    let _strings = get_input_string("00");
    new_year()
}

#[allow(dead_code)]
pub fn solve_part2() -> i64 {
    let _strings = get_input_string("00");
    new_year()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(), 2022);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(), 2022);
    }
}

fn main() {
    println!("{}", solve_part1());
    println!("{}", solve_part2());
}
