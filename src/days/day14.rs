use std::{
    collections::HashSet,
    fmt::{Display, Write},
    str::FromStr,
};

use itertools::Itertools;

use crate::utils::grid::{Grid, Point2};

#[allow(dead_code)]
pub fn one(input: &str) -> u32 {
    let (mut grid, floor_y) = parse(input);
    let mut i = 0;
    loop {
        match simulate_sand_corn(&mut grid, floor_y, |grid, p| grid.map.get(&p).is_some()) {
            SandResult::Abyss => break,
            SandResult::Settled => i += 1,
            _ => {}
        };
    }

    i
}

#[allow(dead_code)]
pub fn two(input: &str) -> u32 {
    let (mut grid, floor_y) = parse(input);
    let mut i = 0;
    loop {
        match simulate_sand_corn(&mut grid, floor_y, |grid, p| {
            if let Some(_) = grid.map.get(&p) {
                true
            } else {
                p.y == floor_y
            }
        }) {
            SandResult::Abyss => panic!("can't happen"),
            SandResult::Settled => i += 1,
            SandResult::Congested => break,
        }
    }

    i
}

enum SandResult {
    Abyss,
    Settled,
    Congested,
}

fn simulate_sand_corn<F>(grid: &mut Grid<Tile>, floor_y: isize, obstacle: F) -> SandResult
where
    F: Fn(&Grid<Tile>, Point2) -> bool,
{
    let mut sand = Point2 { x: 500, y: 0 };
    let (down, down_left, down_right) = (
        Point2 { x: 0, y: 1 },
        Point2 { x: -1, y: 1 },
        Point2 { x: 1, y: 1 },
    );
    loop {
        if !obstacle(&grid, sand + down) {
            sand = sand + down;
            if sand.y > floor_y {
                break SandResult::Abyss;
            }
        } else if !obstacle(&grid, sand + down_left) {
            sand = sand + down_left;
        } else if !obstacle(&grid, sand + down_right) {
            sand = sand + down_right;
        } else {
            if obstacle(&grid, sand) {
                break SandResult::Congested;
            }

            grid.map.insert(sand, Tile::Sand);
            break SandResult::Settled;
        }
    }
}

#[derive(Clone, Debug)]
struct Path(Vec<Point2>);

impl FromStr for Path {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points = s
            .split(" -> ")
            .map(|s| {
                s.split_once(',')
                    .map(|(a, b)| Point2 {
                        x: a.parse().unwrap(),
                        y: b.parse().unwrap(),
                    })
                    .unwrap()
            })
            .collect();
        Ok(Path(points))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Rock,
    Sand,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Tile::Rock => '#',
            Tile::Sand => 'o',
        })
    }
}

fn parse(input: &str) -> (Grid<Tile>, isize) {
    let paths = input
        .lines()
        .map(|l| l.parse::<Path>().unwrap())
        .collect_vec();

    let mut all_points = HashSet::new();
    for path in paths {
        for points in path.0.windows(2) {
            let (p1, p2) = (points[0], points[1]);
            all_points.insert(p1);
            all_points.insert(p2);
            all_points.extend(points_between(p1, p2));
        }
    }

    let mut g = Grid::new();
    g.map
        .extend(all_points.into_iter().map(|x| (x, Tile::Rock)));

    let floor = floor_y(&g);

    (g, floor)
}

fn points_between(p1: Point2, p2: Point2) -> Vec<Point2> {
    let mut points = Vec::new();
    let mut p = p1;
    while p != p2 {
        p.x = p.x + (p2.x - p1.x).signum();
        p.y = p.y + (p2.y - p1.y).signum();
        points.push(p);
    }
    points
}

fn floor_y(grid: &Grid<Tile>) -> isize {
    grid.map
        .iter()
        .filter(|(_, v)| **v == Tile::Rock)
        .map(|(p, _)| p.y)
        .max()
        .unwrap()
        + 2
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/14.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(817, super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(23416, super::two(&input));
    }
}
