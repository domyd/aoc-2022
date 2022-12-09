use scan_fmt::scan_fmt;

use crate::utils::grid::{Direction, Grid, Point2};

#[allow(dead_code)]
pub fn one(input: &str) -> u32 {
    let mut board = Board::new(2);
    for (dir, n) in parse_moves(input) {
        for _ in 0..n {
            board.update(dir);
        }
    }
    board.count_tail_visited()
}

#[allow(dead_code)]
pub fn two(input: &str) -> u32 {
    let mut board = Board::new(10);
    for (dir, n) in parse_moves(input) {
        for _ in 0..n {
            board.update(dir);
        }
    }
    board.count_tail_visited()
}

fn parse_moves(input: &str) -> Vec<(Direction, usize)> {
    input
        .lines()
        .map(|l| scan_fmt!(l, "{} {}", char, usize).unwrap())
        .map(|(c, n)| {
            (
                match c {
                    'L' => Direction::West,
                    'R' => Direction::East,
                    'D' => Direction::South,
                    'U' => Direction::North,
                    _ => panic!("wrong char"),
                },
                n,
            )
        })
        .collect()
}

struct Board {
    grid: Grid<bool>,
    head: Point2,
    tail: Vec<Point2>,
}

impl Board {
    pub fn new(length: usize) -> Self {
        let mut grid = Grid::new();
        grid.map.insert(Point2::zero(), true);
        Board {
            grid,
            head: Point2::zero(),
            tail: (0..length - 1).map(|_| Point2::zero()).collect(),
        }
    }

    pub fn update(&mut self, dir: Direction) {
        // update head
        self.head = self.head + dir.offset();

        // update tail
        let mut head = self.head;
        for tail in self.tail.iter_mut() {
            if tail.x.abs_diff(head.x) > 1 || tail.y.abs_diff(head.y) > 1 {
                *tail = head + shorten_point(*tail - head);
            }
            head = *tail;
        }

        *(self.grid.map.entry(head).or_default()) = true;

        fn shorten_point(p: Point2) -> Point2 {
            // Shortens `p` (which we treat as a vector here) by one unit on each axis
            Point2 {
                x: p.x + (-p.x.signum()),
                y: p.y + (-p.y.signum()),
            }
        }
    }

    pub fn count_tail_visited(&self) -> u32 {
        self.grid.map.values().filter(|v| **v).count() as u32
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/09.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(6339, super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(2541, super::two(&input));
    }
}
