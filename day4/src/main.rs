use std::fmt;

use colored::*;

fn main() {
    let s = include_str!("input.txt");
    let scores: Vec<usize> = s
        .split('\n')
        .next()
        .map(|row| {
            row.split(',')
                .map(|score| score.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .unwrap();

    let board_strings: Vec<&str> = s
        .split('\n')
        .skip(2)
        .map(|x| {
            x.split(' ')
                .filter(|y| !y.is_empty())
                .collect::<Vec<&str>>()
        })
        .flatten()
        .collect();

    let mut boards: Vec<Board> = board_strings.chunks(25).map(Board::create).collect();
    let mut board_counter = 1;

    for score in scores {
        for board in boards.iter_mut() {
            if board.already_won() {
                continue;
            }
            board.score_tile(score);
        }

        for board in boards.iter_mut() {
            if !board.already_won() && board.board_won() {
                board.record_win(board_counter);
                if board_counter == 100 {
                    println!(
                        "Board Won!: \n{}\nScore: {} on number {}",
                        board,
                        board.score_board(score),
                        score
                    );
                }
                board_counter += 1;
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Tile {
    value: usize,
    checked: bool,
}

impl Tile {
    pub fn from_str(tile_str: &str) -> Self {
        Self {
            value: tile_str.parse().unwrap(),
            checked: false,
        }
    }

    // this could return a boolean to allow the board.has_won() fn to only check when a tile actually gets updated
    fn score(&mut self, value: usize) {
        if self.value == value && !self.checked {
            self.checked = true
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.checked {
            write!(f, "{:3}", self.value.to_string().green())
        } else {
            write!(f, "{:3}", self.value.to_string().red())
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Board {
    winner_position: Option<usize>,
    rows: Vec<Vec<Tile>>,
}

impl Board {
    pub fn create(input: &[&str]) -> Self {
        Self {
            winner_position: None,
            rows: input
                .chunks(5)
                .map(|board_row| {
                    board_row
                        .iter()
                        .map(|tile_str| Tile::from_str(tile_str))
                        .collect()
                })
                .collect(),
        }
    }

    fn score_tile(&mut self, number: usize) {
        self.rows
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|tile| tile.score(number)))
    }

    fn board_won(&self) -> bool {
        let row_complete = self
            .rows
            .iter()
            .any(|row| row.iter().all(|tile| tile.checked));

        if row_complete {
            return true;
        }

        for column in 0..5 {
            let column_count = self
                .rows
                .iter()
                .map(|row| row.get(column).unwrap())
                .filter(|tile| tile.checked)
                .count();

            if column_count == 5 {
                return true;
            }
        }

        false
    }

    fn record_win(&mut self, position: usize) {
        self.winner_position = Some(position);
    }

    fn already_won(&self) -> bool {
        self.winner_position.is_some()
    }

    fn score_board(&self, winning_number: usize) -> usize {
        let unchecked_tiles_score: usize = self
            .rows
            .iter()
            .flatten()
            .filter(|tile| !tile.checked)
            .inspect(|tile| print!("{} ", tile))
            .map(|tile| tile.value)
            .sum();

        winning_number * unchecked_tiles_score
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Board(position={:?})", self.winner_position)?;

        for row in self.rows.iter() {
            for tile in row {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}
