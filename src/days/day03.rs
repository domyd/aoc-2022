use itertools::Itertools;
use std::collections::HashSet;

#[allow(dead_code)]
pub fn one(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_at(l.len() / 2);
            let ac = a.chars().collect::<HashSet<_>>();
            let bc = b.chars().collect::<HashSet<_>>();
            ac.intersection(&bc).map(|x| priority(x)).sum::<u32>()
        })
        .sum()
}

#[allow(dead_code)]
pub fn two(input: &str) -> u32 {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|group| {
            let rucksacks: Vec<HashSet<_>> = group.map(|l| l.chars().collect()).collect();
            let mut shared = rucksacks[0].clone();
            shared.retain(|x| rucksacks[1].contains(&x));
            shared.retain(|x| rucksacks[2].contains(&x));
            shared.iter().map(|x| priority(x)).sum::<u32>()
        })
        .sum()
}

fn priority(c: &char) -> u32 {
    if c.is_lowercase() {
        *c as u32 - 96
    } else {
        *c as u32 - 38
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/03.txt";

    #[test]
    pub fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(7597, super::one(&input));
    }

    #[test]
    pub fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(2607, super::two(&input));
    }
}
