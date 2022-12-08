#[allow(dead_code)]
pub mod grid {
    use std::{
        collections::HashMap,
        fmt::Display,
        ops::{Add, Mul},
    };

    #[derive(Clone, Debug)]
    pub struct Grid<V> {
        pub map: HashMap<Point2, V>,
        pub width: usize,
        pub height: usize,
    }

    #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
    pub struct Point2 {
        pub x: isize,
        pub y: isize,
    }

    impl Add<Point2> for Point2 {
        type Output = Point2;

        fn add(self, rhs: Point2) -> Self::Output {
            Point2 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl Mul<isize> for Point2 {
        type Output = Point2;

        fn mul(self, rhs: isize) -> Self::Output {
            Point2 {
                x: self.x * rhs,
                y: self.y * rhs,
            }
        }
    }

    impl Mul<i32> for Point2 {
        type Output = Point2;

        fn mul(self, rhs: i32) -> Self::Output {
            self * (rhs as isize)
        }
    }

    #[derive(Clone, Copy)]
    pub enum Direction {
        North,
        NorthEast,
        East,
        SouthEast,
        South,
        SouthWest,
        West,
        NorthWest,
    }

    impl Direction {
        pub fn cardinals() -> [Direction; 4] {
            [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ]
        }

        pub fn ordinals() -> [Direction; 4] {
            [
                Direction::NorthEast,
                Direction::SouthEast,
                Direction::SouthWest,
                Direction::NorthWest,
            ]
        }

        pub fn all() -> [Direction; 8] {
            [
                Direction::North,
                Direction::NorthEast,
                Direction::East,
                Direction::SouthEast,
                Direction::South,
                Direction::SouthWest,
                Direction::West,
                Direction::NorthWest,
            ]
        }

        pub fn offset(&self) -> Point2 {
            let (x, y) = match self {
                Direction::North => (-1, 0),
                Direction::East => (0, 1),
                Direction::South => (1, 0),
                Direction::West => (0, -1),
                Direction::NorthEast => (1, 1),
                Direction::SouthEast => (1, -1),
                Direction::SouthWest => (-1, -1),
                Direction::NorthWest => (-1, 1),
            };
            Point2 { x, y }
        }
    }

    impl<V: Copy> Grid<V> {
        pub fn from_vec(vec: Vec<Vec<V>>) -> Self {
            let rows = vec.len();
            let cols = vec.iter().map(|l| l.len()).max().unwrap_or_default();

            let mut map = HashMap::with_capacity(rows * cols);
            for y in 0..rows {
                for x in 0..cols {
                    map.insert(
                        Point2 {
                            x: x as isize,
                            y: y as isize,
                        },
                        vec[y][x],
                    );
                }
            }

            Self {
                map,
                height: rows,
                width: cols,
            }
        }

        pub fn line_starting_from<'a>(
            &'a self,
            p: Point2,
            dir: Direction,
        ) -> impl Iterator<Item = &V> + 'a {
            let offset = dir.offset();
            (0..)
                .into_iter()
                .map(move |n| p + (offset * n))
                .map(|coord| self.map.get(&coord))
                .take_while(|x| x.is_some())
                .map(|x| x.unwrap())
        }
    }

    impl<V: Display> Display for Grid<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let map = &self.map;
            if map.is_empty() {
                writeln!(f, "map is empty")?;
                return Ok(());
            }

            let keys: Vec<_> = map.keys().collect();
            let min_y = keys.iter().min_by_key(|f| f.x).map(|f| f.x).unwrap();
            let min_x = keys.iter().min_by_key(|f| f.y).map(|f| f.y).unwrap();
            let max_y = keys.iter().max_by_key(|f| f.x).map(|f| f.x).unwrap() + 1;
            let max_x = keys.iter().max_by_key(|f| f.y).map(|f| f.y).unwrap() + 1;
            let longest_v = map.values().map(|v| v.to_string().len()).max().unwrap();
            let empty = core::iter::repeat(' ').take(longest_v).collect::<String>();

            for y in min_y..max_y {
                let mut out = String::new();
                let row: Vec<_> = (min_x..max_x)
                    .map(|x| map.get(&Point2 { x, y }).map(|v| v.to_string()))
                    .collect();

                for el in row.iter() {
                    let str = if let Some(s) = el { &s } else { &empty };
                    out.push_str(format!("{:>width$} ", str, width = longest_v).as_ref());
                }

                writeln!(f, "{}", out)?;
            }

            Ok(())
        }
    }
}
