fn main() {
    let s = include_str!("input.txt");

    let fish_counts: Vec<usize> = s
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|line| {
            line.split(',')
                .map(|value| value.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .flatten()
        .collect();
    // keep a 9 item array (i.e. all possible ages of fish) with a count of each fish at each stage
    // rotate the array left, each time you pull off the start, add the count into idx 6 (timer of a fish that's just reproduced)
    // and then also add that count into idx 8 (timer of a fish that's just been born) N.B second part doesn't need to be done, it's handled by rotation

    let mut counts_of_fishes_at_age = vec![0; 9];
    (0..9).for_each(|age| {
        counts_of_fishes_at_age[age] = fish_counts.iter().filter(|&&x| x == age).count();
    });

    let total = 256;

    for _day in 0..total {
        let reproduce_count = counts_of_fishes_at_age[0];
        counts_of_fishes_at_age.rotate_left(1);
        counts_of_fishes_at_age[6] += reproduce_count;
    }

    println!(
        "rotate_answer={}",
        counts_of_fishes_at_age.iter().sum::<usize>()
    );
    // 80 days should eq 383160
}
