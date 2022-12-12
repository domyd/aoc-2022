use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

use crate::utils::grid::{Direction, Grid, Point2};

#[derive(Clone, Copy, Debug)]
struct Position {
    elevation: u8,
    start: bool,
    end: bool,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.elevation)
    }
}

#[derive(Clone, Copy, Debug)]
struct Path {
    prev: Point2,
    cur: Point2,
    count: usize,
}

#[allow(dead_code)]
pub fn one(input: &str) -> usize {
    let (grid, start, _) = parse(input);
    find_trail(&grid, start).unwrap()
}

#[allow(dead_code)]
pub fn two(input: &str) -> usize {
    let (grid, _, _) = parse(input);
    let mut steps = usize::MAX;
    let starting_squares: Vec<Point2> = grid
        .map
        .iter()
        .filter(|p| p.1.elevation == 0)
        .map(|p| *p.0)
        .collect();

    for s in starting_squares {
        if let Some(trail_steps) = find_trail(&grid, s) {
            steps = steps.min(trail_steps);
        }
    }

    steps
}

fn find_trail(grid: &Grid<Position>, start: Point2) -> Option<usize> {
    let offsets = Direction::cardinals().map(|d| d.offset());
    let mut queue = VecDeque::new();
    for o in offsets.clone() {
        queue.push_back(Path {
            prev: start,
            cur: start + o,
            count: 1,
        });
    }

    let mut visited = HashSet::new();
    visited.insert(start);

    while let Some(path) = queue.pop_front() {
        if visited.contains(&path.cur) {
            continue;
        }
        let prev = match grid.map.get(&path.prev) {
            Some(x) => x,
            None => continue,
        };
        let cur = match grid.map.get(&path.cur) {
            Some(x) => x,
            None => continue,
        };
        if !(cur.elevation == prev.elevation + 1 || cur.elevation <= prev.elevation) {
            continue;
        }
        visited.insert(path.cur);

        if cur.end {
            return Some(path.count);
        }

        for o in offsets.clone() {
            queue.push_back(Path {
                prev: path.cur,
                cur: path.cur + o,
                count: path.count + 1,
            });
        }
    }

    None
}

fn parse(input: &str) -> (Grid<Position>, Point2, Point2) {
    let input: Vec<Vec<Position>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| Position {
                    start: c == 'S',
                    end: c == 'E',
                    elevation: match c {
                        'a'..='z' => c as u8 - 97,
                        'S' => b'a' - 97,
                        'E' => b'z' - 97,
                        _ => panic!("input"),
                    },
                })
                .collect()
        })
        .collect();

    let grid = Grid::from_vec(input);
    let (start, _) = grid.find(|x| x.start).unwrap();
    let (end, _) = grid.find(|x| x.end).unwrap();

    (grid, start, end)
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/12.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(350, super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(349, super::two(&input));
    }
}
