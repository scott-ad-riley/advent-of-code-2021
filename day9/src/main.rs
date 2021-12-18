use std::collections::HashSet;

fn main() {
    let s = include_str!("test_input.txt");

    let lines: Vec<Vec<u32>> = s
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();
    let map = Map::new(lines);

    dbg!(map.calculate_lowest());
}

struct Map {
    data: Vec<Vec<u32>>,
}

impl Map {
    pub fn new(input: Vec<Vec<u32>>) -> Self {
        Self { data: input }
    }

    pub fn calculate_lowest(&self) -> usize {
        let mut horizontal_positions: HashSet<(usize, usize)> = HashSet::new();
        let mut final_values: Vec<usize> = Vec::new();

        for (row_pos, row) in self.data.iter().enumerate() {
            let mut col_pos: usize = 0;
            if row[0] < row[1] {
                horizontal_positions.insert((row_pos, col_pos));
            }
            col_pos += 1;
            for window in row.windows(3) {
                if window[0] > window[1] && window[2] > window[1] {
                    horizontal_positions.insert((row_pos, col_pos));
                }
                col_pos += 1;
            }
            if row[col_pos] < row[col_pos - 1] {
                horizontal_positions.insert((row_pos, col_pos));
            }
        }

        for (x, y) in horizontal_positions {
            let above = if x == 0 {
                &u32::MAX
            } else {
                self.data
                    .get(x - 1)
                    .map(|row| row.get(y))
                    .flatten()
                    .unwrap()
            };

            let below = self
                .data
                .get(x + 1)
                .map(|row| row.get(y))
                .flatten()
                .unwrap_or(&u32::MAX);
            let value = self.data.get(x).unwrap().get(y).unwrap();
            if above > value && below > value {
                final_values.push(*value as usize);
            }
        }

        final_values.iter().sum::<usize>() + final_values.len()
    }
}
