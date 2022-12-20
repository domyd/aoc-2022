#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Num {
    value: i64,
    idx: usize,
}

#[allow(dead_code)]
pub fn one(input: &str) -> i64 {
    let mut nums = parse(input);
    let orig = nums.clone();

    for on in orig {
        mix(&mut nums, on);
    }

    eval(&nums)
}

#[allow(dead_code)]
pub fn two(input: &str) -> i64 {
    let mut nums = parse(input);
    nums = nums
        .into_iter()
        .map(|n| Num {
            value: n.value * 811589153,
            ..n
        })
        .collect();
    let orig = nums.clone();

    for _ in 0..10 {
        for on in &orig {
            mix(&mut nums, *on);
        }
    }

    eval(&nums)
}

fn mix(nums: &mut Vec<Num>, n: Num) {
    let from_idx = nums.iter().position(|num| *num == n).unwrap() as i64;
    let n = nums.remove(from_idx as usize);
    let to_idx = (from_idx + n.value).rem_euclid(nums.len() as i64);
    nums.insert(to_idx as usize, n);
}

fn eval(nums: &Vec<Num>) -> i64 {
    let zero = nums.iter().position(|n| n.value == 0).unwrap();
    [1000 + zero, 2000 + zero, 3000 + zero]
        .map(|n| n % nums.len())
        .map(|i| nums[i].value)
        .into_iter()
        .sum()
}

fn parse(input: &str) -> Vec<Num> {
    input
        .lines()
        .enumerate()
        .map(|(i, l)| Num {
            value: l.parse().unwrap(),
            idx: i,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/20.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(19559, super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(912226207972, super::two(&input));
    }
}
