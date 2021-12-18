const KNOWN_LENGTHS: &[usize] = &[2, 4, 3, 7];

fn main() {
    let s = include_str!("input.txt");

    let lines: Vec<(&str, Vec<&str>)> = s
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.split('|').collect::<Vec<&str>>())
        .map(|line| {
            let mut iter = line.iter();
            (*iter.next().unwrap(), *iter.next().unwrap())
        })
        .map(|(signals, output)| {
            (
                signals,
                output
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .collect::<Vec<&str>>(),
            )
        })
        .collect();

    let x: usize = lines
        .iter()
        .map(|line| &line.1)
        .map(|output| {
            output
                .iter()
                .filter(|digit_str| KNOWN_LENGTHS.contains(&digit_str.len()))
                .count()
        })
        .sum();

    println!("{:?}", x);
}
