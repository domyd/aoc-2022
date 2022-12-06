use itertools::Itertools;

#[allow(dead_code)]
pub fn one(input: &str) -> u32 {
    let signal: Vec<char> = input.lines().map(|l| l.chars().collect()).next().unwrap();
    marker_idx(&signal, 4).unwrap() as u32
}

#[allow(dead_code)]
pub fn two(input: &str) -> u32 {
    let signal: Vec<char> = input.lines().map(|l| l.chars().collect()).next().unwrap();
    marker_idx(&signal, 14).unwrap() as u32
}

fn marker_idx(signal: &[char], marker_length: usize) -> Option<usize> {
    let mut marker = None;
    for i in marker_length..signal.len() {
        let slice = &signal[i - marker_length..i];
        if slice.iter().all_unique() {
            marker = Some(i);
            break;
        }
    }

    marker
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/06.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(1300, super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(3986, super::two(&input));
    }
}
