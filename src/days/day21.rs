use std::{collections::HashMap, fmt::Display, str::FromStr};

use itertools::Itertools;

#[allow(dead_code)]
pub fn one(input: &str) -> i64 {
    let monkeys = parse(input);
    let equations = HashMap::from_iter(monkeys.clone().into_iter().map(|m| (m.name, m.yell)));

    let root_m = {
        let idx = monkeys.iter().position(|m| m.name == "root").unwrap();
        monkeys[idx].clone()
    };

    let op = &root_m.yell.construct_from_system(&equations);
    let val = op.eval().unwrap();
    val
}

#[allow(dead_code)]
pub fn two(input: &str) -> i64 {
    let monkeys = {
        let mut monkeys = parse(input);
        monkeys.remove(monkeys.iter().position(|m| m.name == "humn").unwrap());
        monkeys
    };
    let equations = HashMap::from_iter(monkeys.clone().into_iter().map(|m| (m.name, m.yell)));

    let root_m = {
        let idx = monkeys.iter().position(|m| m.name == "root").unwrap();
        monkeys[idx].clone()
    };

    let expr = &root_m.yell.construct_from_system(&equations).clone();
    let expr = match expr {
        Expr::Binary(expr) => expr,
        _ => panic!("root op must be binary"),
    };
    let value = solve1(expr).unwrap();
    value
}

/// Solves a single-degree equation system of the form x = y.
fn solve1(expr: &ExprBinary) -> Option<i64> {
    match (expr.lhs.eval(), expr.rhs.eval()) {
        (None, None) => None,
        (None, Some(v)) => Some(expr.lhs.substitute(v)),
        (Some(v), None) => Some(expr.rhs.substitute(v)),
        (Some(_), Some(_)) => None,
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    name: String,
    yell: Expr,
}

#[derive(Clone, Debug)]
enum Expr {
    Val(i64),
    Var(String),
    Binary(ExprBinary),
}

#[derive(Clone, Debug)]
struct ExprBinary {
    lhs: Box<Expr>,
    rhs: Box<Expr>,
    op: OpKind,
}

#[derive(Clone, Copy, Debug)]
enum OpKind {
    Add,
    Sub,
    Mul,
    Div,
}

impl ExprBinary {
    pub fn eval(&self) -> Option<i64> {
        let lhs = self.lhs.eval()?;
        let rhs = self.rhs.eval()?;
        Some(match self.op {
            OpKind::Add => lhs + rhs,
            OpKind::Sub => lhs - rhs,
            OpKind::Mul => lhs * rhs,
            OpKind::Div => lhs / rhs,
        })
    }
}

impl Expr {
    pub fn eval(&self) -> Option<i64> {
        match self {
            Expr::Val(v) => Some(*v),
            Expr::Var(_) => None,
            Expr::Binary(expr) => expr.eval(),
        }
    }

    pub fn construct_from_system(&self, eqs: &HashMap<String, Self>) -> Self {
        match self {
            Expr::Binary(expr) => {
                let lhs = Box::new(expr.lhs.construct_from_system(eqs));
                let rhs = Box::new(expr.rhs.construct_from_system(eqs));
                Expr::Binary(ExprBinary {
                    lhs,
                    rhs,
                    op: expr.op,
                })
            }
            expr @ Expr::Var(k) => {
                if let Some(expr) = eqs.get(k) {
                    expr.construct_from_system(eqs)
                } else {
                    expr.clone()
                }
            }
            expr => expr.clone(),
        }
    }

    pub fn substitute(&self, value: i64) -> i64 {
        match self {
            Expr::Var(_) => value,
            Expr::Val(v) => *v,
            Expr::Binary(expr) => {
                let lhs = expr.lhs.eval();
                let rhs = expr.rhs.eval();
                match (lhs, rhs) {
                    (Some(_), Some(_)) => expr.eval().unwrap(),
                    (Some(l), None) => {
                        match expr.op {
                            OpKind::Add => {
                                // l + x = value
                                // x = value - l
                                expr.rhs.substitute(value - l)
                            }
                            OpKind::Sub => {
                                // l - x = value
                                // -x = value - l
                                // x = -(value - l)
                                expr.rhs.substitute(-(value - l))
                            }
                            OpKind::Mul => {
                                // l * x = value
                                // x = value / l
                                expr.rhs.substitute(value / l)
                            }
                            OpKind::Div => {
                                // l / x = value
                                // x = l / value
                                expr.rhs.substitute(l / value)
                            }
                        }
                    }
                    (None, Some(r)) => {
                        match expr.op {
                            OpKind::Add => {
                                // x + r = value
                                // x = value - r
                                expr.lhs.substitute(value - r)
                            }
                            OpKind::Sub => {
                                // x - r = value
                                // x = value + r
                                expr.lhs.substitute(value + r)
                            }
                            OpKind::Mul => {
                                // x * r = value
                                // x = value / r
                                expr.lhs.substitute(value / r)
                            }
                            OpKind::Div => {
                                // x / l = value
                                // x = value * l
                                expr.lhs.substitute(value * r)
                            }
                        }
                    }
                    (None, None) => panic!("left and right can't eval"),
                }
            }
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Val(v) => write!(f, "{}", *v),
            Expr::Var(k) => write!(f, "{}", k),
            Expr::Binary(binary) => write!(f, "{}", binary),
        }
    }
}

impl Display for ExprBinary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.lhs, self.op, self.rhs)
    }
}

impl Display for OpKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OpKind::Add => '+',
                OpKind::Sub => '-',
                OpKind::Mul => '*',
                OpKind::Div => '/',
            }
        )
    }
}

impl FromStr for Expr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(expr) = s.parse::<i64>().map(|n| Expr::Val(n)) {
            // unary
            Ok(expr)
        } else {
            // binary
            let expr_str = s.split_ascii_whitespace().collect_vec();
            let expr = match expr_str[..] {
                [l, op, r] => {
                    let lhs = Box::new(Expr::Var(l.to_string()));
                    let rhs = Box::new(Expr::Var(r.to_string()));
                    let op = match op {
                        "+" => OpKind::Add,
                        "-" => OpKind::Sub,
                        "/" => OpKind::Div,
                        "*" => OpKind::Mul,
                        _ => panic!("invalid op"),
                    };
                    Expr::Binary(ExprBinary { lhs, rhs, op })
                }
                _ => panic!("invalid op line"),
            };
            Ok(expr)
        }
    }
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, rest) = s.split_once(": ").unwrap();
        Ok(Monkey {
            name: name.to_owned(),
            yell: rest.parse().unwrap(),
        })
    }
}

fn parse(input: &str) -> Vec<Monkey> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/21.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(353837700405464, super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(3678125408017, super::two(&input));
    }
}
