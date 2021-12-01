fn main() {
    let s = include_str!("input.txt");
    let input_parsed: Vec<usize> = s
        .split('\n')
        .filter(|line| line.len() > 1)
        .map(|x| {
            x.parse::<usize>()
                .unwrap_or_else(|_| panic!("could not parse into number: {}", x))
        })
        .collect();

    let mut input = input_parsed.windows(3).peekable();
    let mut count = 0;

    // part 1
    // while let Some(current) = input.next() {
    //     if let Some(next) = input.peek() {
    //         if next > &current {
    //             count += 1;
    //         }
    //     }
    // }

    // part 2
    while let Some(current) = input.next() {
        if let Some(next) = input.peek() {
            if next.iter().sum::<usize>() > current.iter().sum() {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
