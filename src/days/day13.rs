use std::{cmp::Ordering, str::FromStr};

use itertools::Itertools;

#[allow(dead_code)]
pub fn one(input: &str) -> usize {
    let input = parse_input(input);
    input
        .into_iter()
        .tuples()
        .enumerate()
        .map(|(i, (l, r))| if l < r { i + 1 } else { 0 })
        .sum()
}

#[allow(dead_code)]
pub fn two(input: &str) -> usize {
    let mut input = parse_input(input);

    let dividers = vec!["[[2]]".parse().unwrap(), "[[6]]".parse().unwrap()];
    input.append(&mut dividers.clone());

    input.sort();

    let [a, b] = [&dividers[0], &dividers[1]].map(|d| {
        input
            .iter()
            .find_position(|x| **x == *d)
            .map(|x| x.0 + 1)
            .unwrap()
    });
    a * b
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Packet {
    Integer(i32),
    List(Vec<Self>),
}

impl Packet {
    fn as_slice(&self) -> &[Self] {
        if let Packet::List(list) = self {
            list.as_slice()
        } else {
            std::slice::from_ref(self)
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if let (Self::Integer(l), Self::Integer(r)) = (self, other) {
            l.cmp(r)
        } else {
            self.as_slice().cmp(other.as_slice())
        }

        // BEFORE I figured out that the sorting of lists is lexicographical, and that comparing
        // slices of T: Ord does that, I implemented this by hand:

        // let (mut left, mut right) = (
        //     VecDeque::from(Vec::from(self.as_slice())),
        //     VecDeque::from(Vec::from(other.as_slice())),
        // );
        // let mut result = Ordering::Equal;
        // loop {
        //     match (left.pop_front(), right.pop_front()) {
        //         (None, None) => break,
        //         (None, Some(_)) => {
        //             if result == Ordering::Equal {
        //                 result = Ordering::Less;
        //             };
        //             break;
        //         }
        //         (Some(_), None) => {
        //             if result == Ordering::Equal {
        //                 result = Ordering::Greater;
        //             };
        //             break;
        //         }
        //         (Some(l), Some(r)) => {
        //             let comp = match (&l, &r) {
        //                 (Packet::Integer(l), Packet::Integer(r)) => l.cmp(r),
        //                 (l, r) => l.cmp(r),
        //             };
        //             if comp != Ordering::Equal {
        //                 result = comp;
        //                 break;
        //             }
        //         }
        //     }
        // }

        // result
    }
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use chumsky::prelude::*;
        fn parser() -> impl Parser<char, Packet, Error = Simple<char>> {
            recursive(|bf| {
                bf.separated_by(just(','))
                    .delimited_by(just('['), just(']'))
                    .map(Packet::List)
                    .or(text::int(10).map(|v: String| Packet::Integer(v.parse().unwrap())))
                    .padded()
            })
        }

        parser()
            .parse(s)
            .map_err(|_| "failed to parse Packet".to_string())
    }
}

fn parse_input(input: &str) -> Vec<Packet> {
    input.lines().filter_map(|l| l.parse().ok()).collect()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/13.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(4643, super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(21614, super::two(&input));
    }
}
