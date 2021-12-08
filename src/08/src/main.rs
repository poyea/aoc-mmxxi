use util::input::get_input_string;

#[allow(dead_code)]
pub fn solve_part1() -> i64 {
    let strings = get_input_string("08");
    let mut total = 0;
    for string in strings {
        let input_output = string.split(" | ").collect::<Vec<&str>>();
        if let [_, output] = &input_output[..] {
            for entry in output.split(" ") {
                total += match entry.len() {
                    2 | 3 | 4 | 7 => 1,
                    _ => 0,
                };
            }
        }
    }
    total
}

fn id(word: &str) -> i64 {
    let mut ret = 0;
    for ch in word.chars() {
        ret |= match ch {
            'a' => 1,
            'b' => 2,
            'c' => 4,
            'd' => 8,
            'e' => 16,
            'f' => 32,
            'g' => 64,
            _ => panic!("Something went wrong at the hasher!"),
        }
    }
    ret
}

#[allow(dead_code)]
pub fn solve_part2() -> i64 {
    let strings = get_input_string("08");
    let mut results = Vec::<i64>::new();
    for string in strings {
        let input_output = string.split(" | ").collect::<Vec<&str>>();
        let (mut one, mut four, mut seven) = (0, 0, 0);
        let mut current: String = String::new();
        if let [input, output] = &input_output[..] {
            for entry in input.split(" ") {
                match entry.len() {
                    2 => one = id(entry),
                    3 => seven = id(entry),
                    4 => four = id(entry),
                    _ => (),
                };
            }
            for entry in output.split(" ") {
                match entry.len() {
                    2 => one = id(entry),
                    3 => seven = id(entry),
                    4 => four = id(entry),
                    _ => (),
                };
            }
            for entry in output.split(" ") {
                match entry.len() {
                    2 => current.push('1'),
                    3 => current.push('7'),
                    4 => current.push('4'),
                    7 => current.push('8'),
                    5 => {
                        let entry_id = id(entry);
                        if (four & entry_id).count_ones() == 2 {
                            current.push('2');
                        } else {
                            let t = if one != 0 { one } else { seven };
                            if ((four & entry_id) & t).count_ones() == 1 {
                                current.push('5');
                            } else {
                                current.push('3');
                            }
                        }
                    }
                    6 => {
                        let entry_id = id(entry);
                        if (four & entry_id).count_ones() == 4 {
                            current.push('9');
                        } else {
                            let t = if one != 0 { one } else { seven };
                            if ((four & entry_id) & t).count_ones() == 1 {
                                current.push('6');
                            } else {
                                current.push('0');
                            }
                        }
                    }
                    _ => {
                        panic!("Something went wrong at a word!");
                    }
                };
            }
        }
        if current.len() != 4 {
            panic!("Something went wrong at the list!");
        }
        results.push(current.parse().unwrap());
    }
    results.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(), 383);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(), 998900);
    }
}

fn main() {
    println!("{}", solve_part1());
    println!("{}", solve_part2());
}
