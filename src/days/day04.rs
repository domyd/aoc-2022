use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
struct Range {
    from: u32,
    to: u32,
}

#[derive(Debug)]
enum Overlap {
    Partial,
    Full,
}

impl Range {
    pub fn overlaps(&self, other: &Range) -> Option<Overlap> {
        if self.to < other.from || self.from > other.to {
            return None;
        }

        if (self.from >= other.from && self.to <= other.to)
            || (other.from >= self.from && other.to <= self.to)
        {
            return Some(Overlap::Full);
        }

        return Some(Overlap::Partial);
    }
}

impl FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.split_once('-')
            .map(|(from, to)| (from.parse().unwrap(), to.parse().unwrap()))
            .map(|(from, to)| Range { from, to })
            .unwrap())
    }
}

pub fn count_pairs(input: &str, full: bool) -> u32 {
    input
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(a, b)| (Range::from_str(a).unwrap(), Range::from_str(b).unwrap()))
        .filter(|(a, b)| {
            let m = a.overlaps(b);
            if full {
                matches!(m, Some(_))
            } else {
                matches!(m, Some(Overlap::Full))
            }
        })
        .count() as u32
}

#[allow(dead_code)]
pub fn one(input: &str) -> u32 {
    count_pairs(input, false)
}

#[allow(dead_code)]
pub fn two(input: &str) -> u32 {
    count_pairs(input, true)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/04.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(518, super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(909, super::two(&input));
    }
}
