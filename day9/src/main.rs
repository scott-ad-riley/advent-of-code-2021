use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt,
};

const DIRECTIONS: &[Direction] = &[
    Direction::Above,
    Direction::Below,
    Direction::Left,
    Direction::Right,
];

fn main() {
    let s = include_str!("input.txt");

    let lines: Vec<Vec<usize>> = s
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.chars()
                .map(|x| x.to_digit(10).unwrap())
                .map(|x| x as usize)
                .collect::<Vec<usize>>()
        })
        .collect();
    let mut map = Map::new(lines);
    map.create_basins();

    let mut basins: Vec<HashSet<Point>> = map
        .basins
        .into_inner()
        .into_iter()
        .map(|(_, points)| points)
        .collect();

    basins.sort_by_key(|a| a.len());
    basins.reverse();

    println!("{}", basins[0].len() * basins[1].len() * basins[2].len());
}

type Point = (usize, usize);
type BasinId = i32;
#[derive(Debug)]
enum Direction {
    Above,
    Below,
    Left,
    Right,
}

impl Direction {
    fn transform(&self) -> (isize, isize) {
        match self {
            Direction::Above => (-1, 0),
            Direction::Below => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn apply(&self, point: Point, max_x: isize, max_y: isize) -> Option<Point> {
        let new_x = point.0 as isize + self.transform().0;
        let new_y = point.1 as isize + self.transform().1;

        if new_x < 0 || new_y < 0 || new_x > max_x || new_y > max_y {
            None
        } else {
            Some((new_x as usize, new_y as usize))
        }
    }
}

#[derive(Debug)]
struct Map {
    data: Vec<Vec<usize>>,
    basins: RefCell<HashMap<BasinId, HashSet<Point>>>,
    in_basin: RefCell<Vec<Vec<Option<BasinId>>>>,
}

impl Map {
    pub fn new(input: Vec<Vec<usize>>) -> Self {
        let in_basin: Vec<Vec<Option<BasinId>>> = input
            .iter()
            .map(|row| row.iter().map(|_| None).collect::<Vec<Option<BasinId>>>())
            .collect();
        Self {
            in_basin: RefCell::new(in_basin),
            data: input,
            basins: RefCell::new(HashMap::new()),
        }
    }

    // scan every node, if it's adjacent to a node in a basin, put it in that basin
    // if two of its adjacents are in different basins, merge those two basins into one
    // (and add it to that basin)
    pub fn create_basins(&mut self) {
        let mut basin_id: BasinId = 0;
        for (row_pos, row) in self.data.iter().enumerate() {
            for (col_pos, value) in row.iter().enumerate() {
                if *value != 9 {
                    let adjacents = self.adjacents_in_basins((row_pos, col_pos));
                    match adjacents.len() {
                        0 => {
                            self.basins.borrow_mut().insert(
                                basin_id,
                                HashSet::from_iter(vec![(row_pos, col_pos)].into_iter()),
                            );
                            self.in_basin.borrow_mut()[row_pos][col_pos] = Some(basin_id);
                            basin_id += 1;
                        }
                        1 => {
                            // we're in a basin already, either to the left or above
                            let basin_id = adjacents.first().unwrap().1;
                            self.basins
                                .borrow_mut()
                                .entry(basin_id)
                                .and_modify(|entry| {
                                    entry.insert((row_pos, col_pos));
                                });
                            self.in_basin.borrow_mut()[row_pos][col_pos] = Some(basin_id);
                        }
                        2 => {
                            // we may have just hit another basin, check if they're both the same - if so merge basins
                            let basin_id_to_merge_into = adjacents.first().unwrap().1;
                            let basin_id_to_merge_from = adjacents.get(1).unwrap().1;

                            if basin_id_to_merge_from == basin_id_to_merge_into {
                                self.basins
                                    .borrow_mut()
                                    .entry(basin_id_to_merge_into)
                                    .and_modify(|entry| {
                                        entry.insert((row_pos, col_pos));
                                    });
                                self.in_basin.borrow_mut()[row_pos][col_pos] =
                                    Some(basin_id_to_merge_into);
                            } else {
                                // merge basins
                                self.merge_basins(basin_id_to_merge_into, basin_id_to_merge_from);
                                self.basins
                                    .borrow_mut()
                                    .entry(basin_id_to_merge_into)
                                    .and_modify(|entry| {
                                        entry.insert((row_pos, col_pos));
                                    });
                                self.in_basin.borrow_mut()[row_pos][col_pos] =
                                    Some(basin_id_to_merge_into);
                            }
                        }
                        _ => panic!(
                            "We've got more than 2 adjacents in basins - shouldn't be possible"
                        ),
                    }
                }
            }
        }
    }

    fn merge_basins(&self, target: BasinId, source: BasinId) {
        let points: Vec<(usize, usize)> = self
            .basins
            .borrow()
            .get(&source)
            .unwrap()
            .iter()
            .copied()
            .collect();

        self.basins
            .borrow_mut()
            .entry(target)
            .and_modify(|target_basin| {
                for point in points.iter() {
                    target_basin.insert(*point);
                }
            });

        for point in points {
            self.in_basin.borrow_mut()[point.0][point.1] = Some(target)
        }

        self.basins.borrow_mut().remove(&source);
    }

    fn adjacents_in_basins(&self, point: Point) -> Vec<(Point, BasinId)> {
        let mut adjacents: Vec<(Point, BasinId)> = DIRECTIONS
            .iter()
            .map(|direction| self.basin_at(point, direction))
            .filter(|x| matches!(x, (Some(_), Some(_))))
            .map(|(x, y)| (x.unwrap(), y.unwrap()))
            .collect();

        adjacents.sort_by(|(_, a), (_, b)| a.cmp(b));

        adjacents
    }

    fn basin_at(&self, point: Point, direction: &Direction) -> (Option<Point>, Option<BasinId>) {
        let max_rows = (self.data.len() - 1) as isize;
        let max_cols = (self.data.first().unwrap().len() - 1) as isize;

        match direction.apply(point, max_rows, max_cols) {
            Some(new_point) => (
                Some(new_point),
                self.in_basin.borrow()[new_point.0][new_point.1],
            ),
            None => (None, None),
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for basin_ids in self.in_basin.borrow().iter() {
            for col in basin_ids {
                match col {
                    Some(basin_id) => write!(f, "{:3}", basin_id)?,
                    None => write!(f, "{:3}", "  *")?,
                }
            }
            writeln!(f)?;
        }

        writeln!(f)
    }
}
