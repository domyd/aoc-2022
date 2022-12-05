use scan_fmt::scan_fmt;
use std::collections::{BTreeMap, VecDeque};

use itertools::Itertools;

#[allow(dead_code)]
pub fn one(input: &str) -> String {
    let (mut map, moves) = parse_input(input);
    dbg!(&map);
    for (n, from, to) in moves {
        for _ in 0..n {
            let from_char = map.get_mut(&from).unwrap().pop_back().unwrap();
            map.get_mut(&to).unwrap().push_back(from_char);
        }
    }

    map.values().map(|v| v.back().unwrap()).collect()
}

#[allow(dead_code)]
pub fn two(input: &str) -> String {
    let (mut map, moves) = parse_input(input);
    for (n, from, to) in moves {
        let from_vec = map.get_mut(&from).unwrap();
        let crates = from_vec.drain((from_vec.len() - n)..).collect_vec();
        for c in crates {
            map.entry(to).and_modify(|v| v.push_back(c));
        }
    }

    map.values().map(|v| v.back().unwrap()).collect()
}

fn parse_input(input: &str) -> (BTreeMap<usize, VecDeque<char>>, Vec<(usize, usize, usize)>) {
    // crate stack
    let mut map: BTreeMap<usize, VecDeque<char>> = BTreeMap::new();
    for l in input.lines().take_while(|l| !l.is_empty()) {
        let chars = l
            .chars()
            .collect_vec()
            .chunks(4)
            .map(|c| if c[1] == ' ' { None } else { Some(c[1]) })
            .collect_vec();
        for (i, c) in chars.into_iter().enumerate() {
            if let Some(c) = c {
                map.entry(i + 1).or_default().push_front(c);
            }
        }
    }

    // moves
    let moves = input
        .lines()
        .skip_while(|l| !l.is_empty())
        .filter_map(|l| scan_fmt!(l, "move {} from {} to {}", usize, usize, usize).ok())
        .collect_vec();

    (map, moves)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/05.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!("BSDMQFLSP", super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!("PGSQBFLDP", super::two(&input));
    }
}
