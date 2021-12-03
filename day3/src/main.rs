use std::collections::HashMap;

type BitValue = isize;

const TWELVE_BIT_NUMBER: [i16; 12] = [2048, 1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1];

fn main() {
    let s = include_str!("input.txt");
    let input: Vec<&str> = s.split('\n').filter(|line| line.len() > 1).collect();

    let mut bit_counts: HashMap<BitValue, isize> = HashMap::new();

    for line in input {
        for (bit, value) in line.chars().zip(TWELVE_BIT_NUMBER.iter()) {
            match bit {
                '0' => {
                    *bit_counts.entry((*value).into()).or_insert(-1) -= 1;
                }
                '1' => {
                    *bit_counts.entry((*value).into()).or_insert(1) += 1;
                }
                not_a_bit => panic!("Expected a 1 or 0, got: {:?}", not_a_bit),
            }
        }
    }

    let mut x: Vec<(&BitValue, &isize)> = bit_counts.iter().collect();

    x.sort_by(|(x, _), (y, _)| y.partial_cmp(x).unwrap());

    let mut gamma = String::new();
    let mut epsilon = String::new();

    x.iter().for_each(|(_, count)| {
        gamma.push(to_gamma_bit(**count));
        epsilon.push(to_epsilon_bit(**count));
    });

    let gamma = u32::from_str_radix(gamma.as_str(), 2).unwrap();
    let epsilon = u32::from_str_radix(epsilon.as_str(), 2).unwrap();

    println!(
        "gamma={} epsilon={} result={}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}

fn to_gamma_bit(val: isize) -> char {
    match val > 0 {
        true => '1',
        false => '0',
    }
}

fn to_epsilon_bit(val: isize) -> char {
    match val > 0 {
        true => '0',
        false => '1',
    }
}
