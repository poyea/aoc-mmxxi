use std::str::FromStr;
use util::input::get_input_string;

#[derive(Debug, PartialEq)]
struct PacketInfo {
    pktit: usize,
    pktversion: i64,
    pktype: i64,
    pktoken: i64,
}

impl FromStr for PacketInfo {
    type Err = std::num::ParseIntError;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let pktype: i64 = i64::from_str_radix(&string[3..6], 2)?;
        let pktit: usize = match pktype {
            4 => 6,
            _ => match i64::from_str_radix(&string[6..7], 2)? {
                0 => 22,
                _ => 18,
            },
        };
        let pktversion: i64 = i64::from_str_radix(&string[..3], 2)?;
        let pktoken: i64 = 0;
        Ok(PacketInfo {
            pktit,
            pktversion,
            pktype,
            pktoken,
        })
    }
}

fn hex_to_bin(hex: &str) -> String {
    hex.chars().map(hex_to_bin_char).collect()
}

fn hex_to_bin_char(ch: char) -> &'static str {
    match ch {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

#[allow(dead_code)]
fn parse(string: &str) -> PacketInfo {
    let mut pkt = match PacketInfo::from_str(string) {
        Ok(pkt) => pkt,
        Err(_) => {
            panic!("Something went wrong at initialization");
        }
    };
    if pkt.pktype == 4 {
        let mut tokens: Vec<String> = Vec::new();
        while pkt.pktit < string.chars().count() && string.chars().nth(pkt.pktit).unwrap() == '1' {
            tokens.push(string[pkt.pktit + 1..pkt.pktit + 5].to_string());
            pkt.pktit += 5;
        }
        if string.chars().nth(pkt.pktit).unwrap() != '0' {
            panic!("Something went wrong at parsing literals");
        }
        tokens.push(string[pkt.pktit + 1..pkt.pktit + 5].to_string());
        pkt.pktit += 5;
        pkt.pktoken = i64::from_str_radix(&tokens.join(""), 2).unwrap();
    } else {
        let mut all_numbers: Vec<i64> = Vec::new();
        let subpkt_size = i64::from_str_radix(&string[7..pkt.pktit], 2).unwrap();
        match pkt.pktit {
            18 => {
                // number of subpackets
                for _ in 0..subpkt_size {
                    let next_pkt = parse(&string[pkt.pktit..]);
                    pkt.pktit += next_pkt.pktit;
                    pkt.pktversion += next_pkt.pktversion;
                    all_numbers.push(next_pkt.pktoken);
                }
            }
            22 => {
                // length of subpackets
                while (pkt.pktit as i64) < subpkt_size + 22 {
                    let next_pkt = parse(&string[pkt.pktit..]);
                    pkt.pktit += next_pkt.pktit;
                    pkt.pktversion += next_pkt.pktversion;
                    all_numbers.push(next_pkt.pktoken);
                }
            }
            _ => panic!("Something went wrong at parsing operators"),
        }
        pkt.pktoken = match pkt.pktype {
            0 => all_numbers.iter().sum::<i64>(),
            1 => all_numbers.iter().fold(1, |a, &b| a * b),
            2 => *all_numbers.iter().min().unwrap(),
            3 => *all_numbers.iter().max().unwrap(),
            5 => (all_numbers[0] > all_numbers[1]) as i64,
            6 => (all_numbers[0] < all_numbers[1]) as i64,
            7 => (all_numbers[0] == all_numbers[1]) as i64,
            _ => panic!("Something went wrong at evaluation"),
        };
    }
    pkt
}

#[allow(dead_code)]
pub fn solve_part1() -> i64 {
    let strings = get_input_string("16");
    parse(&hex_to_bin(&strings[0])).pktversion
}

#[allow(dead_code)]
pub fn solve_part2() -> i64 {
    let strings = get_input_string("16");
    parse(&hex_to_bin(&strings[0])).pktoken
}

#[allow(dead_code)]
fn solve_string(string: &str) -> PacketInfo {
    parse(&hex_to_bin(string))
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_example {
        ($($name:ident, $func:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!($func(input).pktoken, expected.0);
                assert_eq!($func(input).pktversion, expected.1);
            }

        )*
        }
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(), 873);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(), 402817863665);
    }

    test_example! {
        test_example_1_solve_part1, solve_string: ("8A004A801A8002F478", (15, 16)),
        test_example_2_solve_part1, solve_string: ("620080001611562C8802118E34", (46, 12)),
        test_example_3_solve_part1, solve_string: ("C0015000016115A2E0802F182340", (46, 23)),
        test_example_4_solve_part1, solve_string: ("A0016C880162017C3686B18A3D4780", (54, 31)),
        test_example_1_solve_part2, solve_string: ("04005AC33890", (54, 8)),
        test_example_2_solve_part2, solve_string: ("880086C3E88112", (7, 15)),
        test_example_3_solve_part2, solve_string: ("CE00C43D881120", (9, 11)),
        test_example_4_solve_part2, solve_string: ("D8005AC2A8F0", (1, 13)),
        test_example_5_solve_part2, solve_string: ("F600BC2D8F", (0, 19)),
        test_example_6_solve_part2, solve_string: ("9C005AC2F8F0", (0, 16)),
        test_example_7_solve_part2, solve_string: ("9C0141080250320F1802104A08", (1, 20)),
    }
}

fn main() {
    println!("{}", solve_part1());
    println!("{}", solve_part2());
}
