use std::io::{self, Write};

fn main() {
    let s = include_str!("input.txt");

    let mut fish_counts: Vec<usize> = s
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|line| {
            line.split(',')
                .map(|value| value.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .flatten()
        .collect();

    for day in 0..80 {
        let new_fish_counts = progress_fish(fish_counts);
        println!("day={} new_fish_counts={}", day, new_fish_counts.len());
        io::stdout().flush().unwrap();
        fish_counts = new_fish_counts;
    }

    println!("answer={}", fish_counts.len())
}

#[allow(clippy::needless_collect)]
fn progress_fish(counts: Vec<usize>) -> Vec<usize> {
    let mut newborn_fish_count = 0;
    let older_fish_counts = counts
        .iter()
        .map(|&current| match current {
            0 => {
                newborn_fish_count += 1;
                6
            }
            val => val - 1,
        })
        .collect::<Vec<usize>>();

    older_fish_counts
        .into_iter()
        .chain(vec![8; newborn_fish_count].into_iter())
        .collect()
}
