use crate::utils::grid::{Direction, Grid, Point2};

#[allow(dead_code)]
pub fn one(input: &str) -> u32 {
    let grid = parse_grid(input);
    grid.map
        .keys()
        .map(|p| is_visible(*p, &grid))
        .filter(|b| *b)
        .count() as u32
}

#[allow(dead_code)]
pub fn two(input: &str) -> u32 {
    let grid = parse_grid(input);
    grid.map
        .keys()
        .map(|p| scenic_score(*p, &grid))
        .max()
        .unwrap()
}

fn scenic_score(p: Point2, grid: &Grid<u32>) -> u32 {
    let height = grid.map.get(&p).unwrap();
    Direction::cardinals()
        .map(|d| {
            let mut cnt = 0;
            for tree_height in grid.line_starting_from(p, d).skip(1).copied() {
                cnt += 1;
                if tree_height >= *height {
                    break;
                }
            }
            cnt
        })
        .into_iter()
        .fold(1, |acc, x| acc * x)
}

fn is_visible(p: Point2, grid: &Grid<u32>) -> bool {
    let height = grid.map.get(&p).unwrap();
    Direction::cardinals()
        .map(|d| grid.line_starting_from(p, d).skip(1).all(|n| n < height))
        .into_iter()
        .any(|b| b)
}

fn parse_grid(input: &str) -> Grid<u32> {
    Grid::from_vec(
        input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10)).collect())
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "src/input/08.txt";

    #[test]
    fn one() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(1717, super::one(&input));
    }

    #[test]
    fn two() {
        let input = std::fs::read_to_string(INPUT).unwrap();
        assert_eq!(321975, super::two(&input));
    }
}
