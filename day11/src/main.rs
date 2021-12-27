use colored::Colorize;
use std::{collections::HashMap, slice::Iter};

fn main() {
    let s = include_str!("input.txt");

    let lines: Vec<Vec<Octopus>> = s
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|row| {
            row.chars()
                .map(|x| Octopus::Dormant(x.to_digit(10).unwrap()))
                .collect::<Vec<Octopus>>()
        })
        .collect();

    let mut map: Map = Map::new(lines);

    for step in 0..5 {
        map.run();

        let flash_count = map.reset_flashes();

        println!("flash count={:?} step={}", flash_count, step + 1);

        println!("{}", map);
        // if flash_count == 100 {
        //     break;
        // }
    }
}

type Point = (usize, usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Octopus {
    Dormant(u32),
    Flashing,
}

#[derive(Debug)]
struct Map {
    points: HashMap<Point, Octopus>,
    max_row: usize,
    max_col: usize,
}

impl Map {
    pub fn new(data: Vec<Vec<Octopus>>) -> Self {
        let mut points = HashMap::new();
        let max_row = data.len();
        let max_col = data.first().unwrap().len();
        data.into_iter().enumerate().for_each(|(row_pos, row)| {
            row.into_iter().enumerate().for_each(|(col_pos, item)| {
                points.insert((row_pos, col_pos), item);
            })
        });

        Self {
            points,
            max_row,
            max_col,
        }
    }
    fn value_at(&mut self, point: Point) -> Option<Octopus> {
        self.points.get(&point).cloned()
    }
    fn update_to(&mut self, point: Point, octopus: Octopus) {
        self.points.entry(point).and_modify(|oct| {
            *oct = octopus;
        });
    }

    fn run(&mut self) {
        for row_pos in 0..self.max_row {
            for col_pos in 0..self.max_col {
                self.run_step((row_pos, col_pos));
            }
        }
    }

    fn run_step(&mut self, point: Point) {
        use Octopus::*;
        let mut octopuses = vec![point];

        while !octopuses.is_empty() {
            println!("{:?}", octopuses);
            let current_point = octopuses.pop().unwrap();

            let current_octopus = self.value_at(current_point).unwrap();

            match current_octopus {
                Dormant(count) if count < 9 => {
                    self.update_to(current_point, Dormant(count + 1));
                }
                Dormant(9) => {
                    self.update_to(current_point, Flashing);

                    let adjacents = adjacents_of(current_point)
                        .iter()
                        .filter_map(|adj| self.value_at(*adj).zip(Some(adj)))
                        .filter(|(oct, _)| !matches!(oct, Flashing))
                        .map(|(oct, adj)| (*adj, oct))
                        .collect::<Vec<(Point, Octopus)>>();
                    for adj in adjacents {
                        octopuses.push(adj.0);
                    }
                }
                Dormant(_) => panic!("Octopus was above 9"),
                Flashing => {}
            };
        }
    }

    fn reset_flashes(&mut self) -> usize {
        let mut flashes = 0;
        for row_pos in 0..self.max_row {
            for col_pos in 0..self.max_col {
                if matches!(
                    self.value_at((row_pos, col_pos)).unwrap(),
                    Octopus::Flashing
                ) {
                    self.update_to((row_pos, col_pos), Octopus::Dormant(0));
                    flashes += 1;
                }
            }
        }
        flashes
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Octopus::*;
        let flash = " F".blue();
        for x in 0..self.max_row {
            for y in 0..self.max_col {
                match self.points.get(&(x, y)).unwrap() {
                    Dormant(count) => write!(f, "{:2}", count)?,
                    Flashing => write!(f, "{}", flash)?,
                };
            }
            writeln!(f)?
        }
        writeln!(f)
    }
}

fn adjacents_of(point: Point) -> Vec<Point> {
    RelativePosition::iterator()
        .map(|pos| pos.to_value(point))
        .filter_map(|(x, y)| x.zip(y))
        .collect()
}

enum RelativePosition {
    Above,
    Left,
    Right,
    Below,
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

impl RelativePosition {
    pub fn iterator() -> Iter<'static, RelativePosition> {
        use RelativePosition::*;
        static POSITIONS: [RelativePosition; 8] = [
            Above,
            Left,
            Right,
            Below,
            TopRight,
            TopLeft,
            BottomRight,
            BottomLeft,
        ];
        POSITIONS.iter()
    }

    fn to_value(&self, point: Point) -> (Option<usize>, Option<usize>) {
        use RelativePosition::*;
        match self {
            Above => (point.0.checked_sub(1), Some(point.1)),
            Left => (Some(point.0), point.1.checked_sub(1)),
            Right => (Some(point.0), Some(point.1 + 1)),
            Below => (Some(point.0 + 1), Some(point.1)),
            TopRight => (point.0.checked_sub(1), Some(point.1 + 1)),
            TopLeft => (point.0.checked_sub(1), point.1.checked_sub(1)),
            BottomRight => (Some(point.0 + 1), Some(point.1 + 1)),
            BottomLeft => (Some(point.0 + 1), point.1.checked_sub(1)),
        }
    }
}
