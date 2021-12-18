use util::input::get_input_string;

#[derive(Clone)]
enum Node {
    Pair(Box<Node>, Box<Node>),
    Num(i64),
}

enum AddDirection {
    Left,
    Right,
}

fn add(dir: AddDirection, node: &mut Box<Node>, number: i64) {
    match dir {
        AddDirection::Left => match node.as_mut() {
            Node::Num(n) => *n += number,
            Node::Pair(l, _) => add(dir, l, number),
        },
        AddDirection::Right => match node.as_mut() {
            Node::Num(n) => *n += number,
            Node::Pair(_, r) => add(dir, r, number),
        },
    }
}

struct Explosion {
    exploded: bool,
    left_pop: i64,
    right_pop: i64,
}

fn explode(node: &mut Box<Node>, times: i64) -> Explosion {
    match node.as_mut() {
        Node::Num(_) => Explosion {
            exploded: false,
            left_pop: -1,
            right_pop: -1,
        },
        Node::Pair(l, r) => match times {
            4 => match (&**l, &**r) {
                (Node::Num(left), Node::Num(right)) => {
                    let res = Explosion {
                        exploded: true,
                        left_pop: *left,
                        right_pop: *right,
                    };
                    **node = Node::Num(0);
                    res
                }
                (_, _) => panic!("Something went wrong"),
            },
            _ => {
                let left = explode(l, times + 1);
                if left.exploded {
                    if left.right_pop != -1 {
                        add(AddDirection::Left, r, left.right_pop);
                        Explosion {
                            exploded: true,
                            left_pop: left.left_pop,
                            right_pop: -1,
                        }
                    } else {
                        left
                    }
                } else {
                    let right = explode(r, times + 1);
                    if right.exploded {
                        if right.left_pop != -1 {
                            add(AddDirection::Right, l, right.left_pop);
                            Explosion {
                                exploded: true,
                                left_pop: -1,
                                right_pop: right.right_pop,
                            }
                        } else {
                            right
                        }
                    } else {
                        Explosion {
                            exploded: false,
                            left_pop: -1,
                            right_pop: -1,
                        }
                    }
                }
            }
        },
    }
}

fn magnitude(node: &Node) -> i64 {
    match node {
        Node::Pair(l, r) => 3 * magnitude(l) + 2 * magnitude(r),
        Node::Num(num) => *num as i64,
    }
}

fn split(node: &mut Box<Node>) -> bool {
    match node.as_mut() {
        Node::Pair(l, r) => split(l) || split(r),
        Node::Num(n) => match n < &mut 10 {
            true => false,
            false => {
                **node = Node::Pair(
                    Box::new(Node::Num(*n / 2)),
                    Box::new(Node::Num((*n + 1) / 2)),
                );
                true
            }
        },
    }
}

fn reduce(node: &mut Box<Node>) {
    let sp = explode(node, 0);
    if !sp.exploded && !split(node) {
        return;
    }
    reduce(node);
}

#[allow(dead_code)]
fn print(node: &Node, d: i64) {
    match node {
        Node::Num(num) => {
            print!("{}", num);
        }
        Node::Pair(l, r) => {
            print!("[");
            print(l, d + 1);
            print!(",");
            print(r, d + 1);
            print!("]");
        }
    }
    if d == 0 {
        print!("\n");
    }
}

fn parse(string: &str) -> (usize, Box<Node>) {
    if &string[0..1] == "[" {
        // [xxx,.....
        // ^
        let (lptr, l) = parse(&string[1..]);
        // [xxx,.....
        //    ^
        let (rptr, r) = parse(&string[lptr + 2..]);
        // [xxx,xxx].....
        //        ^
        (lptr + rptr + 3, Box::new(Node::Pair(l, r)))
    } else {
        (1, Box::new(Node::Num(string[0..1].parse::<i64>().unwrap())))
    }
}

fn get_input() -> Vec<Box<Node>> {
    let strings = get_input_string("18");
    let mut res = Vec::new();
    for string in strings {
        res.push(parse(&string).1);
    }
    res
}

#[allow(dead_code)]
pub fn solve_part1() -> i64 {
    let input = get_input();
    let mut total = input[0].clone();
    for node in input[1..].iter() {
        total = Box::new(Node::Pair(total, node.clone()));
        reduce(&mut total);
    }
    magnitude(&total).try_into().unwrap()
}

#[allow(dead_code)]
pub fn solve_part2() -> i64 {
    let input = get_input();
    let mut maxi = -(1 << 31);
    for (i, node) in input.iter().enumerate() {
        for next in input[i + 1..].iter() {
            let mut left = Box::new(Node::Pair(node.clone(), next.clone()));
            let mut right = Box::new(Node::Pair(next.clone(), node.clone()));
            reduce(&mut left);
            reduce(&mut right);
            maxi = maxi.max(magnitude(&left)).max(magnitude(&right));
        }
    }
    maxi.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(), 3699);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(), 4735);
    }
}

fn main() {
    println!("{}", solve_part1());
    println!("{}", solve_part2());
}
