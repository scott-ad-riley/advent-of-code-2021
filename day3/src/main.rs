use std::cmp::Ordering;

fn main() {
    let s = include_str!("input.txt");
    let input: Vec<&str> = s.split('\n').filter(|line| line.len() > 1).collect();

    let mut possible_oxygen_values = input.clone();
    let oxygen_mode = OccurrenceMode::MostCommon;

    for position in 0..12 {
        possible_oxygen_values = oxygen_mode.filter_by_position(&possible_oxygen_values, position);

        if possible_oxygen_values.len() == 1 {
            break;
        }
    }

    let mut possible_co2_values = input;
    let co2_mode = OccurrenceMode::LeastCommon;

    for position in 0..12 {
        possible_co2_values = co2_mode.filter_by_position(&possible_co2_values, position);

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
    fn default(&self) -> &str {
        use OccurrenceMode::*;
        match self {
            MostCommon => "1",
            LeastCommon => "0",
        }
    }

    pub fn filter_by_position<'a>(&self, lines: &[&'a str], position: usize) -> Vec<&'a str> {
        let partitioned: (Vec<&str>, Vec<&str>) = lines
            .iter()
            .partition(|line| str_at(line, position) == Some(self.default()));

        use OccurrenceMode::*;
        use Ordering::*;
        // return the partition which is largest, defaulting to the first if the same length
        match (self, partitioned.0.len().cmp(&partitioned.1.len())) {
            (_, Equal) => partitioned.0,
            (MostCommon, Greater) | (LeastCommon, Less) => partitioned.0,
            (MostCommon, Less) | (LeastCommon, Greater) => partitioned.1,
        }
    }
}

fn str_at(string: &str, pos: usize) -> Option<&str> {
    string.get(pos..(pos + 1))
}
