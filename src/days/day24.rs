use std::fmt::Display;

use itertools::Itertools;

use crate::utils::grid::{BoundingBox2, Direction, Grid, Point2};

#[allow(dead_code)]
pub fn one(input: &str) -> u32 {
    let grid = parse(input);
    let start = Point2 { x: 1, y: 0 };
    let end = Point2 {
        x: grid.width as isize - 2,
        y: grid.height as isize - 1,
    };

    let (node, _) = find_path(
        &grid,
        Node {
            minute: 0,
            point: start,
        },
        end,
    );

    node.minute as u32
}

#[allow(dead_code)]
pub fn two(input: &str) -> u32 {
    let mut grid = parse(input);
    let mut start = Point2 { x: 1, y: 0 };
    let mut end = Point2 {
        x: grid.width as isize - 2,
        y: grid.height as isize - 1,
    };

    let mut nodes = Vec::new();
    for _ in 0..3 {
        let (node, end_grid) = find_path(
            &grid,
            Node {
                minute: 0,
                point: start,
            },
            end,
        );

        grid = end_grid;
        std::mem::swap(&mut start, &mut end);

        nodes.push(node);
    }

    nodes.into_iter().map(|n| n.minute).sum::<usize>() as u32
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Node {
    minute: usize,
    point: Point2,
}

fn find_path(grid: &Grid<Tile>, start: Node, end: Point2) -> (Node, Grid<Tile>) {
    use pathfinding::prelude::*;

    let mut grids = Vec::new();
    grids.push(grid.clone());

    let bounding_box = BoundingBox2 {
        lowest: Point2 { x: 0, y: 0 },
        highest: Point2 {
            x: grid.width as isize - 1,
            y: grid.height as isize - 1,
        },
    };

    let path = bfs(
        &start,
        |n| {
            let minute = n.minute + 1;

            // get the grid for the current `minute`, advancing it from the previous minute if
            // necessary
            let grid = match grids.get(minute) {
                Some(g) => g,
                None => {
                    let prev_g = grids.get(minute - 1).unwrap();
                    let g = advance_blizzards(&prev_g);
                    grids.push(g);
                    grids.get(minute).unwrap()
                }
            };

            // evaluate successor states
            [(0, 0), (1, 0), (0, 1), (-1, 0), (0, -1)]
                .map(|(x, y)| n.point + Point2 { x, y })
                .into_iter()
                .filter(|p| {
                    bounding_box.contains(p)
                        && match grid.map.get(p) {
                            Some(_) => false,
                            None => true,
                        }
                })
                .map(|p| Node { point: p, minute })
                .collect_vec()
        },
        |n| n.point == end,
    )
    .unwrap();

    let node = path.last().unwrap();
    (*node, grids[node.minute].clone())
}

fn advance_blizzards(grid: &Grid<Tile>) -> Grid<Tile> {
    let mut target = grid.clone();
    target.map.retain(|_, t| matches!(t, Tile::Wall));

    for (point, blizzard) in grid
        .map
        .iter()
        .filter_map(|(p, b)| match b {
            Tile::Wall => None,
            Tile::Blizzard(blizzards) => Some((p, blizzards)),
        })
        .flat_map(|(p, blizzards)| blizzards.into_iter().map(|b| (*p, *b)))
    {
        let target_point = blizzard.offset() + point;

        // set target for blizzard
        let target_point = match grid.map.get(&target_point) {
            Some(Tile::Wall) => {
                // wrap around
                match blizzard {
                    Direction::North => Point2 {
                        y: grid.height as isize - 2,
                        ..target_point
                    },
                    Direction::East => Point2 {
                        x: 1,
                        ..target_point
                    },
                    Direction::South => Point2 {
                        y: 1,
                        ..target_point
                    },
                    Direction::West => Point2 {
                        x: grid.width as isize - 2,
                        ..target_point
                    },
                    _ => panic!("invalid dir"),
                }
            }
            _ => target_point,
        };

        // move blizzard
        target
            .map
            .entry(target_point)
            .and_modify(|t| match t {
                Tile::Blizzard(bs) => bs.push(blizzard),
                _ => panic!("target map hit wall"),
            })
            .or_insert(Tile::Blizzard(vec![blizzard]));
    }

    target
}

#[derive(Clone, Debug)]
enum Tile {
    Wall,
    Blizzard(Vec<Direction>),
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Tile::Wall => "#".to_string(),
            Tile::Blizzard(dirs) => match &dirs[..] {
                [d] => match d {
                    Direction::North => "^".to_string(),
                    Direction::East => ">".to_string(),
                    Direction::South => "v".to_string(),
                    Direction::West => "<".to_string(),
                    _ => panic!("invalid dirs"),
                },
                _ => dirs.len().to_string(),
            },
        };

        write!(f, "{}", s)
    }
}

fn parse(input: &str) -> Grid<Tile> {
    Grid::from_vec(
        input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '#' => Some(Tile::Wall),
                        '<' => Some(Tile::Blizzard(vec![Direction::West])),
                        'v' => Some(Tile::Blizzard(vec![Direction::South])),
                        '>' => Some(Tile::Blizzard(vec![Direction::East])),
                        '^' => Some(Tile::Blizzard(vec![Direction::North])),
                        _ => None,
                    })
                    .collect()
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/24.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(322, super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(974, super::two(&input));
    }
}
