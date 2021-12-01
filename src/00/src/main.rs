use util::dummy::new_year;

#[allow(dead_code)]
pub fn solve() -> i64 {
    new_year()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 2022);
    }
}

fn main() {
    println!("{}", solve());
}
