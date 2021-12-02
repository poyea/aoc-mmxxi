use util::input::get_input_string;

#[allow(dead_code)]
pub fn solve_part1() -> i64 {
    let strings: Vec<String> = get_input_string("02");
    let (mut distance, mut depth): (i64, i64) = (0, 0);
    for s in &strings {
        let split: Vec<&str> = s.split(" ").collect();
        let amount: i64 = split[1].parse().expect("Unable to parse integer");
        match split[0] {
            "up" => depth -= amount,
            "down" => depth += amount,
            "forward" => distance += amount,
            _ => panic!("Unable to decide: {:?}", split[0]),
        }
    }
    distance * depth
}

#[allow(dead_code)]
pub fn solve_part2() -> i64 {
    let strings: Vec<String> = get_input_string("02");
    let (mut distance, mut depth, mut aim): (i64, i64, i64) = (0, 0, 0);
    for s in &strings {
        let split: Vec<&str> = s.split(" ").collect();
        let amount: i64 = split[1].parse().expect("Unable to parse integer");
        match split[0] {
            "up" => aim -= amount,
            "down" => aim += amount,
            "forward" => {
                distance += amount;
                depth += amount * aim;
            }
            _ => panic!("Unable to decide: {:?}", split[0]),
        }
    }
    distance * depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(), 1670340);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(), 1954293920);
    }
}

fn main() {
    println!("{}", solve_part1());
    println!("{}", solve_part2());
}
