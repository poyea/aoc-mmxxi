use std::collections::HashMap;
use util::input::get_input_string;

struct Token {
    register_token: Option<usize>,
    value_token: Option<i64>,
}

impl Token {
    fn get(&self, registers: &[i64; 4]) -> i64 {
        if let Some(v) = self.value_token {
            return v;
        }
        if let Some(r) = self.register_token {
            return registers[r];
        }
        panic!("Something went wrong")
    }
}

enum Op {
    Inp(Token),
    Add(Token, Token),
    Mul(Token, Token),
    Eql(Token, Token),
    Div(Token, Token),
    Mod(Token, Token),
}

impl Op {
    fn eval(&self, reg: &mut [i64; 4], input: Option<i64>) {
        match self {
            Op::Inp(a) => {
                let pos = a.register_token.unwrap();
                reg[pos] = input.unwrap();
            }
            Op::Add(a, b) => {
                let pos = a.register_token.unwrap();
                reg[pos] += b.get(reg);
            }
            Op::Mul(a, b) => {
                let pos = a.register_token.unwrap();
                reg[pos] *= b.get(reg);
            }
            Op::Eql(a, b) => {
                let pos = a.register_token.unwrap();
                reg[pos] = match reg[pos] == b.get(reg) {
                    true => 1,
                    _ => 0,
                }
            }
            Op::Div(a, b) => {
                let pos = a.register_token.unwrap();
                reg[pos] /= b.get(reg);
            }
            Op::Mod(a, b) => {
                let pos = a.register_token.unwrap();
                reg[pos] %= b.get(reg);
            }
        }
    }
}

fn search(
    ops: &[Op],
    ptr: usize,
    registers: [i64; 4],
    digits: [i64; 9],
    cache: &mut HashMap<([i64; 4], usize), Option<i64>>,
) -> Option<i64> {
    if let Some(v) = cache.get(&(registers, ptr)) {
        return *v;
    }
    for digit in digits {
        let mut registers = registers;
        ops[ptr].eval(&mut registers, Some(digit));
        let mut i = ptr + 1;
        while let Some(ins) = ops.get(i) {
            if matches!(ops[i], Op::Inp(_)) {
                if let Some(next) = search(ops, i, registers, digits, cache) {
                    cache.insert((registers, i), Some(10 * next + digit));
                    return Some(10 * next + digit);
                } else {
                    break;
                }
            }
            ins.eval(&mut registers, None);
            i += 1;
        }
        if registers[3] == 0 {
            cache.insert((registers, ptr), Some(digit));
            return Some(digit);
        }
    }
    cache.insert((registers, ptr), None);
    None
}

pub fn solve(id: i64) -> i64 {
    let input = get_input_string("24");
    let mut ops = Vec::new();
    for line in input {
        let (ins, register) = line.split_once(" ").unwrap();
        if ins == "inp" {
            let slot = register.parse::<char>().unwrap();
            ops.push(Op::Inp(match slot {
                'w' => Token {
                    register_token: Some(0),
                    value_token: None,
                },
                'x' => Token {
                    register_token: Some(1),
                    value_token: None,
                },
                'y' => Token {
                    register_token: Some(2),
                    value_token: None,
                },
                'z' => Token {
                    register_token: Some(3),
                    value_token: None,
                },
                _ => panic!("Something went wrong"),
            }));
        } else {
            let (name, source) = register.split_once(" ").unwrap();
            let a: Token = match name.parse::<char>().unwrap() {
                'w' => Token {
                    register_token: Some(0),
                    value_token: None,
                },
                'x' => Token {
                    register_token: Some(1),
                    value_token: None,
                },
                'y' => Token {
                    register_token: Some(2),
                    value_token: None,
                },
                'z' => Token {
                    register_token: Some(3),
                    value_token: None,
                },
                _ => panic!("Something went wrong"),
            };
            let b: Token = match source {
                "x" | "y" | "z" | "w" => match source.parse::<char>().unwrap() {
                    'w' => Token {
                        register_token: Some(0),
                        value_token: None,
                    },
                    'x' => Token {
                        register_token: Some(1),
                        value_token: None,
                    },
                    'y' => Token {
                        register_token: Some(2),
                        value_token: None,
                    },
                    'z' => Token {
                        register_token: Some(3),
                        value_token: None,
                    },
                    _ => panic!("Something went wrong"),
                },
                _ => Token {
                    value_token: Some(source.parse::<i64>().unwrap()),
                    register_token: None,
                },
            };
            ops.push(match (ins, a, b) {
                ("add", a, b) => Op::Add(a, b),
                ("mul", a, b) => Op::Mul(a, b),
                ("div", a, b) => Op::Div(a, b),
                ("mod", a, b) => Op::Mod(a, b),
                ("eql", a, b) => Op::Eql(a, b),
                _ => {
                    panic!("Something went wrong");
                }
            });
        }
    }
    let mut cache = HashMap::new();
    let answer;
    if id == 1 {
        answer = search(
            &ops,
            0,
            [0; 4],
            (1..=9).rev().collect::<Vec<_>>().try_into().unwrap(),
            &mut cache,
        );
    } else {
        answer = search(
            &ops,
            0,
            [0; 4],
            (1..=9).collect::<Vec<_>>().try_into().unwrap(),
            &mut cache,
        );
    }
    answer
        .unwrap()
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<i64>()
        .unwrap()
}

#[allow(dead_code)]
pub fn solve_part1() -> i64 {
    solve(1)
}

#[allow(dead_code)]
pub fn solve_part2() -> i64 {
    solve(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(), 52926995971999);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(), 11811951311485);
    }
}

fn main() {
    println!("{}", solve_part1());
    println!("{}", solve_part2());
}
