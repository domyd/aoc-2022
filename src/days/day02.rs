use scan_fmt::scan_fmt;

#[allow(dead_code)]
pub fn one(input: &str) -> u32 {
    rounds(input)
        .iter()
        .map(|(opponent, mine)| score_guide(opponent, mine))
        .sum()
}

#[allow(dead_code)]
pub fn two(input: &str) -> u32 {
    rounds(input)
        .iter()
        .map(|(opponent, mine)| score_to_win(opponent, mine))
        .sum()
}

fn rounds(input: &str) -> Vec<(Hand, Hand)> {
    input
        .lines()
        .map(|l| {
            let (a, b) = scan_fmt!(l, "{} {}", char, char).unwrap();
            (Hand::parse(&a), Hand::parse(&b))
        })
        .collect()
}

fn score_guide(opponent: &Hand, mine: &Hand) -> u32 {
    let guaranteed = mine.score();
    let round = match (opponent, mine) {
        (Hand::Rock, Hand::Rock) => 3,
        (Hand::Rock, Hand::Paper) => 6,
        (Hand::Rock, Hand::Scissors) => 0,
        (Hand::Paper, Hand::Rock) => 0,
        (Hand::Paper, Hand::Paper) => 3,
        (Hand::Paper, Hand::Scissors) => 6,
        (Hand::Scissors, Hand::Rock) => 6,
        (Hand::Scissors, Hand::Paper) => 0,
        (Hand::Scissors, Hand::Scissors) => 3,
    };
    round + guaranteed
}

fn score_to_win(opponent: &Hand, mine: &Hand) -> u32 {
    let needs_to = NeedsTo::from(*mine);
    let choice = match (opponent, needs_to) {
        (Hand::Rock, NeedsTo::Lose) => Hand::Scissors,
        (Hand::Rock, NeedsTo::Draw) => Hand::Rock,
        (Hand::Rock, NeedsTo::Win) => Hand::Paper,
        (Hand::Paper, NeedsTo::Lose) => Hand::Rock,
        (Hand::Paper, NeedsTo::Draw) => Hand::Paper,
        (Hand::Paper, NeedsTo::Win) => Hand::Scissors,
        (Hand::Scissors, NeedsTo::Lose) => Hand::Paper,
        (Hand::Scissors, NeedsTo::Draw) => Hand::Scissors,
        (Hand::Scissors, NeedsTo::Win) => Hand::Rock,
    };
    score_guide(opponent, &choice)
}

#[derive(Copy, Clone, Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Copy, Clone, Debug)]
enum NeedsTo {
    Lose,
    Draw,
    Win,
}

impl Hand {
    pub fn parse(c: &char) -> Hand {
        match c {
            'A' | 'X' => Hand::Rock,
            'B' | 'Y' => Hand::Paper,
            'C' | 'Z' => Hand::Scissors,
            _ => unreachable!(),
        }
    }

    pub fn score(self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

impl From<Hand> for NeedsTo {
    fn from(hand: Hand) -> Self {
        match hand {
            Hand::Rock => NeedsTo::Lose,
            Hand::Paper => NeedsTo::Draw,
            Hand::Scissors => NeedsTo::Win,
        }
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/02.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(17189, super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(13490, super::two(&input));
    }
}
