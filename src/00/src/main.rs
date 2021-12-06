use util::dummy::new_year;

#[allow(dead_code)]
pub fn solve_part1() -> i64 {
    new_year()
}

#[allow(dead_code)]
pub fn solve_part2() -> i64 {
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
        assert_eq!(solve_part1(), 2022);
    }
}

fn main() {
    println!("{}", solve_part1());
    println!("{}", solve_part2());
}
