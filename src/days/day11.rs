use core::panic;
use std::{cell::RefCell, collections::VecDeque};

use itertools::Itertools;

#[allow(dead_code)]
pub fn one(input: &str) -> u64 {
    let monkeys = parse_monkeys(input);
    ensue_monkey_business(monkeys, 20, false)
}

#[allow(dead_code)]
pub fn two(input: &str) -> u64 {
    let monkeys = parse_monkeys(input);
    ensue_monkey_business(monkeys, 10_000, true)
}

fn ensue_monkey_business(mut monkeys: Vec<Monkey>, rounds: usize, part2: bool) -> u64 {
    let lcm = monkeys
        .iter()
        .map(|m| m.test.0)
        .fold(1u64, |acc, x| acc * x);

    for _ in 0..rounds {
        monkeys = round(monkeys, lcm, part2);
    }

    monkeys
        .into_iter()
        .map(|m| m.inspection_count)
        .sorted()
        .rev()
        .take(2)
        .fold(1u64, |acc, x| acc * x as u64)
}

fn round(monkeys: Vec<Monkey>, lcm: u64, part2: bool) -> Vec<Monkey> {
    // We need runtime borrow-checking
    let monkeys = monkeys
        .into_iter()
        .map(|m| RefCell::new(m.clone()))
        .collect_vec();

    for m in &monkeys {
        while let Some(item) = {
            let mut m = m.borrow_mut();
            m.items.pop_front()
        } {
            m.borrow_mut().inspection_count += 1;

            let (next_m, item) = {
                let m = m.borrow();
                let mut item = m.operation.eval(item);
                if !part2 {
                    item /= 3;
                }
                item %= lcm;
                if item % m.test.0 == 0 {
                    (m.test.1, item)
                } else {
                    (m.test.2, item)
                }
            };

            monkeys[next_m].borrow_mut().items.push_back(item);
        }
    }

    monkeys.into_iter().map(|m| m.into_inner()).collect()
}

#[derive(Clone, Debug)]
pub struct Monkey {
    items: VecDeque<u64>,
    operation: Op,
    test: (u64, usize, usize),
    inspection_count: usize,
}

#[derive(Clone, Debug)]
pub enum Op {
    Add(Box<Op>, Box<Op>),
    Mul(Box<Op>, Box<Op>),
    Num(u64),
    Var(String),
}

impl Op {
    pub fn eval(&self, old: u64) -> u64 {
        match self {
            Op::Add(l, r) => l.eval(old) + r.eval(old),
            Op::Mul(l, r) => l.eval(old) * r.eval(old),
            Op::Num(i) => *i,
            Op::Var(name) if name == "old" => old,
            _ => panic!("unsupported eval"),
        }
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input
        .replace("\r\n", "\n")
        .split("\n\n")
        .into_iter()
        .map(|input| {
            // Parse monkey
            let mut lines = input.lines();

            // Monkey ID
            let line = lines.next().unwrap();
            let _ = scan_fmt::scan_fmt!(line, "Monkey {}:", usize).unwrap();

            // Items
            let line = lines.next().unwrap();
            let line = line.strip_prefix("  Starting items: ").unwrap();
            let items = line.split(", ").map(|x| x.parse().unwrap()).collect();

            // Operation
            let line = lines.next().unwrap();
            let parse_atom = |token| match token {
                "old" => Op::Var("old".to_string()),
                s => s.parse::<u64>().map(|x| Op::Num(x)).unwrap(),
            };
            let line = line.strip_prefix("  Operation: new = ").unwrap();
            let (l, op, r) = line.split_whitespace().collect_tuple().unwrap();
            let (l, r) = (parse_atom(l), parse_atom(r));
            let operation = match op {
                "*" => Op::Mul(Box::new(l), Box::new(r)),
                "+" => Op::Add(Box::new(l), Box::new(r)),
                x => panic!("encountered invalid operation {}", x),
            };

            // Test
            let (test, yes, no) = lines.take(3).collect_tuple().unwrap();
            let strip_num =
                |line: &str, prefix| line.strip_prefix(prefix).unwrap().parse::<usize>().unwrap();
            let test = strip_num(test, "  Test: divisible by ");
            let yes = strip_num(yes, "    If true: throw to monkey ");
            let no = strip_num(no, "    If false: throw to monkey ");

            Monkey {
                items,
                operation,
                test: (test as u64, yes, no),
                inspection_count: 0,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/11.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(56120, super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(24389045529u64, super::two(&input));
    }
}
