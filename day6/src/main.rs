use std::collections::HashMap;

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

    // for each possible age, run for all the days and insert the end result into a hashmap
    // loop through the input data and look the value up in the hashmap, add it to the total

    // V2
    // Run through 128 days for a single age
    // Cache the result
    // Produce the vec of fish ages after 128 days
    // iterate throught that, for each fish add it's age to the total

    let mut lookups: HashMap<usize, usize> = HashMap::new();
    let ages = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    let total = 256;

    for start_age in ages {
        lookups.insert(start_age, run_for_day_count(start_age, total / 2));
    }

    println!("{:?}", lookups);

    for _day in 0..(total / 2) {
        fish_counts = progress_fish(fish_counts);
    }

    // we have a 50% done array of fish, now run to completion by iterating through and looking up each value
    let mut faster_count = 0;

    for fish in fish_counts {
        faster_count += lookups.get(&fish).unwrap();
    }

    println!("{}", faster_count);
    // 80 days should eq 383160
}

fn run_for_day_count(start: usize, day_count: usize) -> usize {
    let mut initial = vec![start];
    for day in 0..day_count {
        println!("Day({}/{}) -> {}", day, day_count, initial.len());
        let new_fish_counts = progress_fish(initial);
        initial = new_fish_counts;
    }

    initial.len()
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
