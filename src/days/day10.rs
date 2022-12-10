use std::str::FromStr;

#[allow(dead_code)]
pub fn one(input: &str) -> u32 {
    let instr = parse(input);
    let mut reg: isize = 1;
    let mut sum = 0;
    let mut cycle = 1;

    for instr in instr {
        let count = match instr {
            Instr::Noop => 1,
            Instr::Add(_) => 2,
        };
        for i in 0..count {
            if (cycle + 20) % 40 == 0 {
                let signal = (cycle as isize) * reg;
                sum += signal;
            }

            match instr {
                Instr::Add(v) => {
                    if i == count - 1 {
                        reg += v
                    }
                }
                _ => {}
            }

            cycle += 1;
        }
    }

    sum as u32
}

#[allow(dead_code)]
pub fn two(input: &str) -> String {
    let instr = parse(input);
    let mut reg: isize = 1;
    let mut cycle = 1;
    let mut buffer = String::new();

    for instr in instr {
        let count = match instr {
            Instr::Noop => 1,
            Instr::Add(_) => 2,
        };
        for i in 0..count {
            let row_pos = (cycle - 1) % 40;
            if row_pos >= (reg - 1) && row_pos <= (reg + 1) {
                buffer.push('#');
            } else {
                buffer.push('.');
            }

            match instr {
                Instr::Add(v) => {
                    if i == count - 1 {
                        reg += v
                    }
                }
                _ => {}
            }

            cycle += 1;
        }
    }

    buffer
}

fn parse(input: &str) -> Vec<Instr> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[derive(Clone, Copy, Debug)]
enum Instr {
    Noop,
    Add(isize),
}

impl FromStr for Instr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("noop") {
            Ok(Instr::Noop)
        } else {
            let v = s.split_once(' ').unwrap().1.parse::<isize>().unwrap();
            Ok(Instr::Add(v))
        }
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/10.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(13440, super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        let output = super::two(&input);
        let output = output
            .chars()
            .collect::<Vec<_>>()
            .chunks_exact(40)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        // Prepend \n so it looks nicer in the test
        let mut pretty_output = String::from("\n");
        pretty_output.push_str(&output);

        assert_eq!(
            r"
###..###..####..##..###...##..####..##..
#..#.#..#....#.#..#.#..#.#..#....#.#..#.
#..#.###....#..#....#..#.#..#...#..#..#.
###..#..#..#...#.##.###..####..#...####.
#....#..#.#....#..#.#.#..#..#.#....#..#.
#....###..####..###.#..#.#..#.####.#..#.",
            pretty_output
        );
    }
}
