pub fn sums(input: &str) -> Vec<u32> {
    dbg!(input);
    let blocks: Vec<Vec<u32>> = input
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|b| b.lines().map(|l| l.parse().unwrap()).collect())
        .collect();

    let mut sums: Vec<_> = blocks.iter().map(|b| b.iter().sum::<u32>()).collect();
    sums.sort();
    sums
}

#[allow(dead_code)]
pub fn one(input: &str) -> u32 {
    *sums(input).iter().rev().next().unwrap()
}

#[allow(dead_code)]
pub fn two(input: &str) -> u32 {
    sums(input).iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/01.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(69281, super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(201524, super::two(&input));
    }
}
