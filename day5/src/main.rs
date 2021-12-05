use std::cmp::Ordering::*;
use std::fmt;

fn main() {
    let s = include_str!("input.txt");

    let lines: Vec<Line> = s
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(Line::from)
        // .filter(|line| line.is_straight()) // remove for part 2
        .collect();

    println!("lines count {}", lines.len());

    let max_x_from = lines.iter().max_by(|x, y| x.from.0.cmp(&y.from.0)).unwrap();
    let max_x_to = lines.iter().max_by(|x, y| x.to.0.cmp(&y.to.0)).unwrap();
    let max_y_from = lines.iter().max_by(|x, y| x.from.1.cmp(&y.from.1)).unwrap();
    let max_y_to = lines.iter().max_by(|x, y| x.to.1.cmp(&y.to.1)).unwrap();

    let max_x = max_x_from.from.0.max(max_x_to.to.0);
    let max_y = max_y_from.from.1.max(max_y_to.to.1);

    let mut overlaps = Overlaps::create(max_x, max_y);

    for line in lines {
        overlaps.draw_line(line);
    }

    let overlaps = overlaps
        .data
        .iter()
        .map(|row| {
            row.iter()
                .filter(|&&overlap_point| overlap_point >= 2)
                .collect::<Vec<&isize>>()
        })
        .flatten()
        .count();

    println!("answer={}", overlaps);

    // println!("overlaps\n{}", overlaps);
}

#[derive(Debug)]
struct Overlaps {
    pub data: Vec<Vec<isize>>,
}

impl Overlaps {
    pub fn create(max_x: isize, max_y: isize) -> Self {
        let mut vec: Vec<Vec<isize>> = vec![];
        (0..=max_y).for_each(|_| vec.push(vec![0; (max_x + 1) as usize]));
        Self { data: vec }
    }

    pub fn draw_line(&mut self, line: Line) {
        if line.from.0 == line.to.0 {
            let mut y_range = vec![line.from.1, line.to.1];
            y_range.sort_unstable();
            let mut start = *y_range.first().unwrap();
            let end = *y_range.last().unwrap();
            while start <= end {
                self.mark_point_updated(line.from.0, start);
                start += 1;
            }
        } else if line.from.1 == line.to.1 {
            let mut x_range = vec![line.from.0, line.to.0];
            x_range.sort_unstable();
            let mut start = *x_range.first().unwrap();
            let end = *x_range.last().unwrap();
            while start <= end {
                self.mark_point_updated(start, line.from.1);
                start += 1;
            }
        } else {
            let x_op: isize = match line.from.0.cmp(&line.to.0) {
                Less => 1,
                Greater => -1,
                Equal => panic!("should not be equal: {:?}", line),
            };
            let y_op: isize = match line.from.1.cmp(&line.to.1) {
                Less => 1,
                Greater => -1,
                Equal => panic!("should not be equal: {:?}", line),
            };

            let mut each_point = line.from;

            while each_point.0 != line.to.0 {
                self.mark_point_updated(each_point.0, each_point.1);
                each_point = (each_point.0 + x_op, each_point.1 + y_op);
            }

            self.mark_point_updated(each_point.0, each_point.1);
        }
    }

    fn mark_point_updated(&mut self, x: isize, y: isize) {
        self.data.get_mut(y as usize).map(|row| {
            row.get_mut(x as usize).map(|x| {
                *x += 1;
            })
        });
    }
}

impl fmt::Display for Overlaps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.data.iter() {
            for point in row {
                if *point == 0 {
                    write!(f, ".")?;
                } else {
                    write!(f, "{}", point)?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

#[derive(Debug)]
struct Line {
    pub from: (isize, isize),
    pub to: (isize, isize),
}

impl Line {
    pub fn from(input: &str) -> Self {
        let points: Vec<isize> = input
            .split(" -> ")
            .map(|point| {
                point
                    .split(',')
                    .map(|x| x.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>()
            })
            .flatten()
            .collect();

        Self {
            from: (*points.get(0).unwrap(), *points.get(1).unwrap()),
            to: (*points.get(2).unwrap(), *points.get(3).unwrap()),
        }
    }

    pub fn is_straight(&self) -> bool {
        self.from.0 == self.to.0 || self.from.1 == self.to.1
    }
}
