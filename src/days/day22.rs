use crate::utils::grid::{Direction, Grid, Point2};
use chumsky::prelude::*;
use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

#[allow(dead_code)]
pub fn one(input: &str) -> i32 {
    let (grid, mut instr) = parse(input);
    let x = grid.row(0).into_iter().next().unwrap().0.x;
    let mut pos = Point2 { x, y: 0 };
    let mut dir = Dir::East;

    'outer: while let Some(instr) = instr.pop_front() {
        eprintln!("{:?}, @{:?} facing {:?}", &instr, &pos, &dir);
        match instr {
            Instr::Fwd(n) => {
                for _ in 0..n {
                    pos = match move_2d(&grid, pos, dir) {
                        Some(p) => p,
                        None => continue 'outer,
                    };
                    eprintln!("pos after move: {:?}", &pos);
                }
            }
            Instr::Turn(turn) => {
                dir = match (dir, turn) {
                    (Dir::North, Turn::Right) => Dir::East,
                    (Dir::North, Turn::Left) => Dir::West,
                    (Dir::East, Turn::Right) => Dir::South,
                    (Dir::East, Turn::Left) => Dir::North,
                    (Dir::South, Turn::Right) => Dir::West,
                    (Dir::South, Turn::Left) => Dir::East,
                    (Dir::West, Turn::Right) => Dir::North,
                    (Dir::West, Turn::Left) => Dir::South,
                };
            }
        };
        eprintln!("pos after {:?}: {:?}", &instr, &pos);
    }

    score(pos, dir)
}

#[allow(dead_code)]
pub fn two(input: &str) -> i32 {
    // idea:
    // - turn into Point3 cloud

    unimplemented!()
}

fn move_on_map_2d(grid: &Grid<Space>, from: Point2, dir: Dir) -> Option<Point2> {
    let offset = dir.offset();
    let next = from + offset;

    // figure out the actual position that `next` points to
    let next = match grid.map.get(&next) {
        None => match dir {
            Dir::North => grid.col(from.x).into_iter().last().unwrap().0,
            Dir::East => grid.row(from.y).into_iter().next().unwrap().0,
            Dir::South => grid.col(from.x).into_iter().next().unwrap().0,
            Dir::West => grid.row(from.y).into_iter().last().unwrap().0,
        },
        Some(_) => next,
    };

    // check if we can go there
    match grid.map.get(&next) {
        Some(Space::Empty) => Some(next),
        Some(Space::Wall) => None,
        None => panic!("can't happen"),
    }
}

fn score(pos: Point2, dir: Dir) -> i32 {
    ((pos.y as i32 + 1) * 1000)
        + ((pos.x as i32 + 1) * 4)
        + match dir {
            Dir::North => 3,
            Dir::East => 0,
            Dir::South => 1,
            Dir::West => 2,
        }
}

#[derive(Copy, Clone, Debug)]
enum Space {
    Wall,
    Empty,
}

#[derive(Clone, Copy, Debug)]
enum Turn {
    Right,
    Left,
}

#[derive(Clone, Copy, Debug)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    pub fn offset(&self) -> Point2 {
        match self {
            Dir::North => Direction::North,
            Dir::East => Direction::East,
            Dir::South => Direction::South,
            Dir::West => Direction::West,
        }
        .offset()
    }
}

#[derive(Clone, Copy, Debug)]
enum Instr {
    Fwd(isize),
    Turn(Turn),
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Space::Wall => '#',
                Space::Empty => '.',
            }
        )
    }
}

fn parse(input: &str) -> (Grid<Space>, VecDeque<Instr>) {
    // parse map
    let grid = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => Some(Space::Wall),
                    '.' => Some(Space::Empty),
                    ' ' => None,
                    _ => panic!("unknown char"),
                })
                .collect()
        })
        .collect();

    // parse instructions
    let pass = input
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .next()
        .unwrap()
        .to_string();

    let instr_parser = instr_parser();
    let instr = instr_parser.parse(pass).unwrap().into();

    (Grid::from_vec(grid), instr)
}

fn instr_parser() -> impl Parser<char, Vec<Instr>, Error = Simple<char>> {
    choice((
        text::int(10).map(|s: String| Instr::Fwd(s.parse().unwrap())),
        just('L').to(Instr::Turn(Turn::Left)),
        just('R').to(Instr::Turn(Turn::Right)),
    ))
    .repeated()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/22.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(123046, super::one(&input));
    }

    #[test]
    #[ignore]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(0, super::two(&input));
    }
}
