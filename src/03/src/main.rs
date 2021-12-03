use util::input::get_input_string;

#[allow(dead_code)]
fn most_common_bit(list: &Vec<String>, idx: usize) -> i64 {
    let mut count = 0;
    for s in list {
        let s_vec: Vec<char> = s.chars().collect();
        if s_vec[idx] == '0' {
            count -= 1
        } else {
            count += 1
        }
    }
    (count >= 0) as i64
}

#[allow(dead_code)]
pub fn solve_part1() -> i64 {
    let strings: Vec<String> = get_input_string("03");
    let length = strings[0].len();
    let mut common_digits_list = vec![String::new(); strings[0].len()];
    for i in 0..length {
        common_digits_list[i] = most_common_bit(&strings, i).to_string();
    }
    let epsilon_string = common_digits_list
        .iter()
        .map(|x| if x == "1" { "0" } else { "1" })
        .collect::<Vec<_>>()
        .join("");
    let gamma_string = common_digits_list.join("");

    i64::from_str_radix(&gamma_string, 2).unwrap()
        * i64::from_str_radix(&epsilon_string, 2).unwrap()
}

#[allow(dead_code)]
pub fn solve_part2() -> i64 {
    let (mut oxygen_list, mut co2_list): (Vec<String>, Vec<String>) =
        (get_input_string("03"), get_input_string("03"));
    let (mut ogr, mut co2): (String, String) = (String::new(), String::new());
    let length = oxygen_list[0].len();
    for i in 0..length {
        let oxy_bit = most_common_bit(&oxygen_list, i).to_string();
        let co2_bit = (1 - most_common_bit(&co2_list, i)).to_string();
        oxygen_list.retain(|e| e[i..i + 1] == oxy_bit);
        co2_list.retain(|e| e[i..i + 1] == co2_bit);
        if oxygen_list.len() == 1 && ogr.is_empty() {
            ogr = oxygen_list[0].clone();
        }
        if co2_list.len() == 1 && co2.is_empty() {
            co2 = co2_list[0].clone();
        }
    }
    i64::from_str_radix(&ogr, 2).unwrap() * i64::from_str_radix(&co2, 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(), 3633500);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(), 4550283);
    }
}

fn main() {
    println!("{}", solve_part1());
    println!("{}", solve_part2());
}
