use std::cmp::max;
use std::collections::HashMap;
use util::input::get_input_string;

type Pair = (i64, i64);
type Player = bool;
type CacheParams = (i64, i64, i64, i64, Player);

fn play(p: Pair) -> i64 {
    let mut die = 1;
    let (mut t1, mut t2) = (0, 0);
    let (mut c1, mut c2) = p;
    let mut player = true;
    let mut die_count = 0;
    loop {
        let mut die_accum = die
            + (1 + die)
            + (2 + die)
            + match player {
                true => c1,
                _ => c2,
            };
        while die_accum > 10 {
            die_accum -= 10;
        }
        match player {
            true => {
                t1 += die_accum;
                c1 = die_accum;
            }
            _ => {
                t2 += die_accum;
                c2 = die_accum;
            }
        };
        player = !player;
        die_count += 3;
        die += 3;
        if t1 >= 1000 {
            return t2 * die_count;
        }
        if t2 >= 1000 {
            return t1 * die_count;
        }
    }
}

fn verse(
    (p1, p2): Pair,
    (t1, t2): Pair,
    player: Player,
    map: &mut HashMap<CacheParams, Pair>,
) -> Pair {
    if let Some(e) = map.get(&(p1, p2, t1, t2, player)) {
        return *e;
    }
    let (mut w1, mut w2) = (0, 0);
    for r1 in 1..=3 {
        for r2 in 1..=3 {
            for r3 in 1..=3 {
                let pos = match player {
                    true => (p1 + r1 + r2 + r3 - 1) % 10 + 1,
                    _ => (p2 + r1 + r2 + r3 - 1) % 10 + 1,
                };
                let score = match player {
                    true => t1 + pos,
                    _ => t2 + pos,
                };
                if score >= 21 {
                    match player {
                        true => w1 += 1,
                        _ => w2 += 1,
                    };
                    continue;
                }
                match player {
                    true => {
                        let (nw1, nw2) = verse((pos, p2), (score, t2), !player, map);
                        w1 += nw1;
                        w2 += nw2;
                    }
                    _ => {
                        let (nw1, nw2) = verse((p1, pos), (t1, score), !player, map);
                        w1 += nw1;
                        w2 += nw2;
                    }
                }
            }
        }
    }
    map.insert((p1, p2, t1, t2, player), (w1, w2));
    return (w1, w2);
}

#[allow(dead_code)]
pub fn solve_part1() -> i64 {
    let strings = get_input_string("21");
    let player_one_init = strings
        .first()
        .unwrap()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap() as i64;
    let player_two_init = strings
        .last()
        .unwrap()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap() as i64;
    play((player_one_init, player_two_init))
}

#[allow(dead_code)]
pub fn solve_part2() -> i64 {
    let strings = get_input_string("21");
    let player_one_init = strings
        .first()
        .unwrap()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap() as i64;
    let player_two_init = strings
        .last()
        .unwrap()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap() as i64;
    let v = verse(
        (player_one_init, player_two_init),
        (0, 0),
        true,
        &mut HashMap::new(),
    );
    max(v.0, v.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(), 734820);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(), 193170338541590);
    }
}

fn main() {
    println!("{}", solve_part1());
    println!("{}", solve_part2());
}
