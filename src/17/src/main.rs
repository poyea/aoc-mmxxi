use std::cmp::{max, Ordering};
use std::collections::HashSet;
use util::input::get_input_string;

type Pair = (i64, i64);

#[allow(dead_code)]
fn input() -> Vec<Vec<i64>> {
    str::replace(get_input_string("17").first().unwrap(), "target area: ", "")
        .split(", ")
        .map(|s| {
            s[2..]
                .split("..")
                .map(|i| i.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<_>>()
}

#[allow(dead_code)]
fn solve(px: Pair, py: Pair) -> Pair {
    let (x1, x2) = px;
    let (y1, y2) = py;
    let mut max_y = -(1 << 31);
    let mut set = HashSet::new();
    for i in -200..200 {
        for j in -200..400 {
            let (mut posx, mut posy, mut velox, mut veloy) = (0, 0, i, j);
            let mut max_cur_y = 0;
            let mut inside = x1 <= posx && posx <= x2 && y1 <= posy && posy <= y2;
            while posy >= y1 {
                posx += velox;
                posy += veloy;
                velox += match velox.cmp(&0) {
                    Ordering::Less => 1,
                    Ordering::Greater => -1,
                    Ordering::Equal => 0,
                };
                veloy -= 1;
                inside |= x1 <= posx && posx <= x2 && y1 <= posy && posy <= y2;
                max_cur_y = max(max_cur_y, posy);
            }
            if inside {
                set.insert((i, j));
                max_y = max(max_y, max_cur_y);
            }
        }
    }
    (max_y, set.len().try_into().unwrap())
}

#[allow(dead_code)]
pub fn solve_part1() -> i64 {
    if let [px, py] = &input()[..] {
        return solve((px[0], px[1]), (py[0], py[1])).0;
    };
    panic!("Something went wrong")
}

#[allow(dead_code)]
pub fn solve_part2() -> i64 {
    if let [px, py] = &input()[..] {
        return solve((px[0], px[1]), (py[0], py[1])).1;
    };
    panic!("Something went wrong")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(), 10878);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(), 4716);
    }
}

fn main() {
    println!("{}", solve_part1());
    println!("{}", solve_part2());
}
