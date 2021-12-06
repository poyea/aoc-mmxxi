use util::input::{get_input_string, split_string};

#[allow(dead_code)]
fn simulate(days: i64) -> i64 {
    let strings = get_input_string("06");
    let fishes: Vec<i64> = split_string(",", strings.first().unwrap());
    let mut fishes_days = vec![0; 9];
    for fish in fishes {
        fishes_days[fish as usize] += 1;
    }
    let mut current_day = 0;
    for _ in 0..days {
        let accumulated_fishes = fishes_days[current_day];
        current_day = (current_day + 1) % 9;
        let next_day = (current_day + 6) % 9;
        fishes_days[next_day] += accumulated_fishes;
    }
    fishes_days.iter().sum()
}

#[allow(dead_code)]
pub fn solve_part1() -> i64 {
    simulate(80)
}

#[allow(dead_code)]
pub fn solve_part2() -> i64 {
    simulate(256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(), 391888);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(), 1754597645339);
    }
}

fn main() {
    println!("{}", solve_part1());
    println!("{}", solve_part2());
}
