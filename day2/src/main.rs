fn main() {
    let s = include_str!("input.txt");
    let mut depth: isize = 0;
    let mut horizontal: isize = 0;
    let mut aim: isize = 0;
    s.split('\n')
        .filter(|line| line.len() > 1)
        .for_each(|line| {
            let parts: Vec<&str> = line.split(' ').collect();

            match (parts.get(0), parts.get(1)) {
                (Some(direction), Some(magnitude)) => {
                    let magnitude = magnitude.parse::<isize>().unwrap();
                    match *direction {
                        "forward" => {
                            horizontal += magnitude;
                            depth += aim * magnitude;
                        }
                        "down" => {
                            aim += magnitude;
                        }
                        "up" => {
                            aim -= magnitude;
                        }
                        direction => panic!("got an unrecognised direction {}", direction),
                    }
                }
                _ => panic!("failed to parse {:?}", parts),
            };
        });

    println!("{:?}", horizontal * depth)
}
