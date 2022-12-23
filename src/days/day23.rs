use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::utils::grid::{BoundingBox2, Direction, Grid, Point2};

#[allow(dead_code)]
pub fn one(input: &str) -> usize {
    let mut map = parse(input);
    let _ = rounds(&mut map, Some(10));
    let bounding_box = BoundingBox2::from_points(map.iter().copied());
    bounding_box.area() - map.len()
}

#[allow(dead_code)]
pub fn two(input: &str) -> usize {
    let mut map = parse(input);
    let rounds_done = rounds(&mut map, None);
    rounds_done
}

fn rounds(map: &mut HashSet<Point2>, nrounds: Option<usize>) -> usize {
    let mut dir_props = [
        [Direction::North, Direction::NorthEast, Direction::NorthWest],
        [Direction::South, Direction::SouthEast, Direction::SouthWest],
        [Direction::West, Direction::NorthWest, Direction::SouthWest],
        [Direction::East, Direction::NorthEast, Direction::SouthEast],
    ];

    let mut i = 0;
    loop {
        if let Some(n) = nrounds {
            if i >= n {
                break;
            }
        }

        i += 1;

        let mut propositions: HashMap<Point2, Vec<Point2>> = HashMap::new();

        // propose moves - 1st half
        'inner: for p in map.iter() {
            let dirs = Direction::all().map(|d| d.offset() + *p);
            let adjacents = dirs
                .iter()
                .filter(|p| map.contains(p))
                .collect::<HashSet<_>>();
            if adjacents.len() == 0 {
                continue 'inner;
            }

            if let Some(proposed_dir) = dir_props
                .iter()
                .filter_map(|dirs| {
                    let any_occupied = dirs
                        .map(|d| d.offset() + *p)
                        .iter()
                        .any(|p| adjacents.contains(p));

                    if any_occupied {
                        None
                    } else {
                        Some(dirs[0])
                    }
                })
                .next()
            {
                let proposed_tile = proposed_dir.offset() + *p;
                (*propositions.entry(proposed_tile).or_default()).push(*p);
            }
        }

        // move - 2nd half
        let moves = propositions
            .iter()
            .filter_map(|(p, elves)| {
                if elves.len() == 1 {
                    Some((elves[0], *p))
                } else {
                    None
                }
            })
            .collect_vec();

        if moves.is_empty() {
            break;
        }

        for (from, to) in moves {
            map.remove(&from);
            map.insert(to);
        }

        // rotate directions
        dir_props.rotate_left(1);
    }

    i
}

fn parse(input: &str) -> HashSet<Point2> {
    let v = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => Some(()),
                    _ => None,
                })
                .collect_vec()
        })
        .collect_vec();

    let grid = Grid::from_vec(v);
    grid.map.keys().copied().collect::<HashSet<_>>()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/23.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(4000, super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(1040, super::two(&input));
    }
}
