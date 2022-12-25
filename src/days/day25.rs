use self::snafu::Snafu;

#[allow(dead_code)]
pub fn one(input: &str) -> String {
    let sum = input
        .lines()
        .map(|l| l.parse::<Snafu>().unwrap())
        .sum::<Snafu>();

    format!("{}", sum)
}

mod snafu {
    use std::{fmt::Display, iter::Sum, ops::Add, str::FromStr};

    use itertools::Itertools;

    #[derive(Clone, PartialEq, Eq, Debug)]
    pub struct Snafu(Vec<i8>);

    impl<'a, 'b> Add<&'_ Snafu> for &'_ Snafu {
        type Output = Snafu;

        fn add(self, rhs: &'_ Snafu) -> Snafu {
            let mut carry = 0;
            let mut digits = Vec::new();
            for v in self.0.iter().zip_longest(rhs.0.iter()) {
                let digit = match v {
                    itertools::EitherOrBoth::Both(lhs, rhs) => lhs + rhs + carry,
                    itertools::EitherOrBoth::Left(lhs) => lhs + carry,
                    itertools::EitherOrBoth::Right(rhs) => rhs + carry,
                };
                carry = (digit + 2).div_euclid(5);
                let digit = ((digit + 2).rem_euclid(5)) - 2;
                digits.push(digit);
            }

            if carry != 0 {
                digits.push(carry);
            }

            Snafu(digits)
        }
    }

    impl Add<Snafu> for Snafu {
        type Output = Snafu;

        fn add(self, rhs: Snafu) -> Self::Output {
            &self + &rhs
        }
    }

    impl Sum for Snafu {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            let mut sum = Snafu::zero();
            for snafu in iter {
                sum = sum + snafu;
            }
            sum
        }
    }

    impl Snafu {
        pub fn to_radix_10(&self) -> i64 {
            self.0
                .iter()
                .enumerate()
                .map(|(i, d)| {
                    let i = 5i64.pow(i as u32);
                    i * (*d as i64)
                })
                .sum()
        }

        pub fn zero() -> Snafu {
            Snafu(vec![0])
        }
    }

    impl FromStr for Snafu {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let res = s
                .chars()
                .rev()
                .map(|c| match c {
                    '2' => Ok(2),
                    '1' => Ok(1),
                    '0' => Ok(0),
                    '-' => Ok(-1),
                    '=' => Ok(-2),
                    c => Err(format!("invalid digit: {}", c)),
                })
                .collect::<Result<_, _>>()?;

            Ok(Snafu(res))
        }
    }

    impl Display for Snafu {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let str = self
                .0
                .iter()
                .rev()
                .map(|d| match *d {
                    -2 => '=',
                    -1 => '-',
                    0 => '0',
                    1 => '1',
                    2 => '2',
                    _ => panic!("snafu contains invalid digit"),
                })
                .collect::<String>();

            write!(f, "{}", str)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn add() {
            add2("1=", "1-");
            add2("12", "2=");
            add2("1=", "2=");
            add2("1=11-2", "1-0---0");
        }

        #[test]
        fn sum() {
            let add = [
                "1=-0-2", "12111", "2=0=", "21", "2=01", "111", "20012", "112", "1=-1=", "1-12",
                "12", "1=", "122",
            ]
            .map(|n| Snafu::from_str(n).unwrap())
            .into_iter()
            .sum::<Snafu>();

            assert_eq!(Snafu::from_str("2=-1=0").unwrap(), add);
        }

        fn add2(a: &str, b: &str) {
            let a = Snafu::from_str(a).unwrap();
            let b = Snafu::from_str(b).unwrap();
            let a10 = &a.to_radix_10();
            let b10 = &b.to_radix_10();

            let sum = a.clone() + b.clone();
            let sum10 = sum.to_radix_10();

            eprintln!("{} + {} => {}", &a, &b, &sum);
            assert_eq!(sum10, a10 + b10);
        }
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/25.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!("2=20---01==222=0=0-2", super::one(&input));
    }
}
