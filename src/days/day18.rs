use std::collections::{HashMap, HashSet};

use crate::utils::grid::{BoundingBox3, Point3};

#[allow(dead_code)]
pub fn one(input: &str) -> usize {
    let points = parse_input(input);
    let points = HashSet::<_>::from_iter(points);
    surface_area(&points)
}

#[allow(dead_code)]
pub fn two(input: &str) -> usize {
    let points = parse_input(input);
    let bb = BoundingBox3::from_points(&points);
    let inverted = invert_points(&points, bb);

    // find clusters of points in inverted set
    let mut clusters = connected_points(&inverted);

    // isolate those that don't touch the bounding box's border -> pockets of air
    clusters.retain(|cluster| cluster.iter().all(|p| !is_point_on_border(*p, bb)));

    // subtract the surface area of the air pockets
    let total_surface_area = surface_area(&HashSet::from_iter(points.clone()));
    let air_pockets_area = clusters.iter().map(|c| surface_area(c)).sum::<usize>();
    let area = total_surface_area - air_pockets_area;

    area
}

fn surface_area(points: &HashSet<Point3>) -> usize {
    let mut neighbors = HashMap::new();

    for p in points {
        let nps = neighbors_of(*p);
        let mut sides = 6;
        for np in nps {
            if points.contains(&np) {
                sides -= 1;
            }
        }
        neighbors.insert(p, sides);
    }

    neighbors.values().sum()
}

fn is_point_on_border(p: Point3, bb: BoundingBox3) -> bool {
    p.x == bb.lowest.x
        || p.x == bb.highest.x
        || p.y == bb.lowest.y
        || p.y == bb.highest.y
        || p.z == bb.lowest.z
        || p.z == bb.highest.z
}

fn invert_points(points: &[Point3], bb: BoundingBox3) -> Vec<Point3> {
    let grid = all_points_in_box(bb);
    let points = HashSet::<_>::from_iter(points.iter().copied());
    grid.difference(&points).copied().collect()
}

fn all_points_in_box(bb: BoundingBox3) -> HashSet<Point3> {
    let mut v = HashSet::with_capacity(bb.volume());
    for x in bb.lowest.x..=bb.highest.x {
        for y in bb.lowest.y..=bb.highest.y {
            for z in bb.lowest.z..=bb.highest.z {
                v.insert(Point3 { x, y, z });
            }
        }
    }
    v
}

fn connected_points(points: &[Point3]) -> Vec<HashSet<Point3>> {
    use pathfinding::prelude::*;

    let ps = HashSet::<_>::from_iter(points.iter().copied());

    connected_components(points, |p| {
        let nps = HashSet::<_>::from_iter(neighbors_of(*p));
        nps.intersection(&ps).copied().collect::<HashSet<_>>()
    })
}

fn neighbors_of(p: Point3) -> [Point3; 6] {
    [
        Point3 { x: 1, y: 0, z: 0 },
        Point3 { x: -1, y: 0, z: 0 },
        Point3 { x: 0, y: 1, z: 0 },
        Point3 { x: 0, y: -1, z: 0 },
        Point3 { x: 0, y: 0, z: 1 },
        Point3 { x: 0, y: 0, z: -1 },
    ]
    .map(|o| p + o)
}

fn parse_input(input: &str) -> Vec<Point3> {
    use scan_fmt::scan_fmt;

    input
        .lines()
        .map(|l| {
            let (x, y, z) = scan_fmt!(l, "{},{},{}", isize, isize, isize).unwrap();
            Point3 { x, y, z }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/18.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(3498, super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(2008, super::two(&input));
    }
}
