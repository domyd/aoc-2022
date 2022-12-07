use itertools::Itertools;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn one(input: &str) -> u64 {
    let dirs = dir_sizes(parse_files(input));

    let sum = dirs
        .iter()
        .filter_map(|(_, v)| if *v <= 100_000 { Some(*v) } else { None })
        .sum();

    sum
}

#[allow(dead_code)]
pub fn two(input: &str) -> u64 {
    let dirs = dir_sizes(parse_files(input));

    let space = 70_000_000 as u64;
    let min_needed_free = 30_000_000 as u64;
    let used = *dirs.get(&vec![]).unwrap();
    let needed_to_delete = min_needed_free - (space - used);

    let result = dirs
        .values()
        .sorted()
        .skip_while(|v| **v < needed_to_delete)
        .next()
        .unwrap();

    *result
}

fn dir_sizes(files: HashMap<Vec<String>, u64>) -> HashMap<Vec<String>, u64> {
    let mut map: HashMap<Vec<String>, u64> = HashMap::new();

    for (path, size) in files {
        let mut p = path.clone();
        while let Some(_) = p.pop() {
            (*map.entry(p.clone()).or_default()) += size as u64;
        }
    }

    map
}

fn parse_files(input: &str) -> HashMap<Vec<String>, u64> {
    let mut map = HashMap::new();

    let mut ls = false;
    let mut path = Vec::new();

    for l in input.lines() {
        if l.starts_with('$') {
            ls = false;

            let l = l.trim_start_matches("$ ");
            if l.starts_with("cd") {
                let dir = l.split_once(' ').unwrap().1;
                match dir {
                    "/" => {
                        path.clear();
                    }
                    ".." => {
                        path.pop();
                    }
                    d => {
                        path.push(d.to_owned());
                    }
                }
            } else if l.starts_with("ls") {
                ls = true;
            }
        } else if ls {
            if let Some((Some(size), filename)) = l
                .split_once(' ')
                .map(|(size, filename)| (size.parse::<u64>().ok(), filename))
            {
                let mut path = path.clone();
                path.push(filename.to_owned());
                map.insert(path, size);
            }
        }
    }

    map
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/07.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(2104783, super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(5883165, super::two(&input));
    }
}
