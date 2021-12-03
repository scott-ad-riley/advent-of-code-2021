fn main() {
    let s = include_str!("input.txt");
    let input: Vec<&str> = s.split('\n').filter(|line| line.len() > 1).collect();

    let mut possible_oxygen_values = input.clone();

    for position in 0..12 {
        // we could also partition and then just take largest, rather than find largest + filter
        let bit = bit_occurrences(
            &possible_oxygen_values,
            position,
            OccurrenceMode::MostCommon,
        );

        possible_oxygen_values =
            filter_lines_with_bit(possible_oxygen_values, position, dbg!(bit).to_string());

        if possible_oxygen_values.len() == 1 {
            break;
        }
    }

    let mut possible_co2_values = input;

    for position in 0..12 {
        // we could also partition and then just take largest, rather than find largest + filter
        let bit = bit_occurrences(&possible_co2_values, position, OccurrenceMode::LeastCommon);
        possible_co2_values =
            filter_lines_with_bit(possible_co2_values, position, dbg!(bit).to_string());
        if possible_co2_values.len() == 1 {
            break;
        }
    }

    println!(
        "{:?}",
        u32::from_str_radix(possible_oxygen_values.first().unwrap(), 2)
    );

    println!(
        "{:?}",
        u32::from_str_radix(possible_co2_values.first().unwrap(), 2)
    );
}

#[derive(Debug)]
enum OccurrenceMode {
    MostCommon,
    LeastCommon,
}

impl OccurrenceMode {
    pub fn for_val(&self, value: i32) -> char {
        use OccurrenceMode::*;
        if value == 0 {
            return match self {
                MostCommon => '1',
                LeastCommon => '0',
            };
        }
        match (self, value > 0) {
            (MostCommon, true) | (LeastCommon, false) => '1',
            (MostCommon, false) | (LeastCommon, true) => '0',
        }
    }
}

fn bit_occurrences(lines: &[&str], position: usize, mode: OccurrenceMode) -> char {
    let mut counter = 0;

    for line in lines {
        match str_at(line, position).unwrap() {
            "0" => {
                counter -= 1;
            }
            "1" => {
                counter += 1;
            }
            not_a_bit => panic!("Expected a 1 or 0, got: {:?}", not_a_bit),
        }
    }

    mode.for_val(counter)
}

// rewrite to avoid extra allocation/don't collect
fn filter_lines_with_bit(lines: Vec<&str>, position: usize, bit: String) -> Vec<&str> {
    lines
        .iter()
        .filter(|line| str_at(line, position) == Some(bit.as_str()))
        .copied()
        .collect()
}

fn str_at(string: &str, pos: usize) -> Option<&str> {
    string.get(pos..(pos + 1))
}
