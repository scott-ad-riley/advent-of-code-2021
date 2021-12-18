use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let s = include_str!("input.txt");

    let mut lines: Vec<DigitDisplay> = s
        .split('\n')
        .filter(|line| !line.is_empty())
        // .map(|input| str::replace(input, "|", ""))
        .map(|signals| {
            signals
                .split(' ')
                .filter(|x| !x.is_empty())
                .filter(|&x| x != "|")
                .collect::<Vec<&str>>()
        })
        .map(DigitDisplay::new)
        .collect();

    let x: usize = lines
        .iter_mut()
        .map(|display| display.determine_output())
        .sum();
    println!("result={:?}", x);
}

#[derive(Debug, Clone)]
struct DigitDisplay {
    readings: Vec<String>,
    // this could be a bimap https://docs.rs/bimap/latest/bimap/ but cba
    known_from_pattern: HashMap<String, usize>,
    known_from_value: HashMap<usize, String>,
}

impl DigitDisplay {
    pub fn new(readings: Vec<&str>) -> Self {
        let readings: Vec<String> = readings
            .iter()
            .map(|reading| reading.chars().sorted().collect::<String>())
            .collect();
        Self {
            readings,
            known_from_pattern: HashMap::new(),
            known_from_value: HashMap::new(),
        }
    }

    fn determine_output(&mut self) -> usize {
        let numbers: &[usize] = &[1, 4, 7, 8, 9, 6, 0, 3, 2, 5];
        for number in numbers {
            match number {
                1 => self.determine_one(),
                4 => self.determine_four(),
                7 => self.determine_seven(),
                8 => self.determine_eight(),
                6 => self.determine_six(),
                9 => self.determine_nine(),
                0 => self.determine_zero(),
                3 => self.determine_three(),
                2 => self.determine_two(),
                5 => self.determine_five(),

                x => panic!("number not handled yet {}", x),
            };
        }

        self.calculate()
    }

    fn calculate(&self) -> usize {
        let num = self
            .readings
            .iter()
            .skip(10)
            .map(|pattern| {
                self.known_from_pattern
                    .get(pattern)
                    .unwrap_or_else(|| panic!("{:?} not found", pattern))
                    .to_string()
            })
            .collect::<String>();

        num.parse::<usize>().unwrap()
    }

    fn charset_for_value(&self, value: usize) -> HashSet<char> {
        HashSet::from_iter(self.known_from_value.get(&value).unwrap().chars())
    }

    fn pattern_for(&self, match_pattern: &str, target: usize) -> bool {
        let target_pattern = self.known_from_value.get(&target).unwrap();
        *target_pattern == match_pattern
    }

    fn determine_one(&mut self) {
        let value = self.readings.iter().find(|value| value.len() == 2).unwrap();
        self.known_from_pattern.insert(value.clone(), 1);
        self.known_from_value.insert(1, value.clone());
    }

    fn determine_four(&mut self) {
        let value = self.readings.iter().find(|value| value.len() == 4).unwrap();
        self.known_from_pattern.insert(value.clone(), 4);
        self.known_from_value.insert(4, value.clone());
    }

    fn determine_seven(&mut self) {
        let value = self.readings.iter().find(|value| value.len() == 3).unwrap();
        self.known_from_pattern.insert(value.clone(), 7);
        self.known_from_value.insert(7, value.clone());
    }

    fn determine_eight(&mut self) {
        let value = self.readings.iter().find(|value| value.len() == 7).unwrap();
        self.known_from_pattern.insert(value.clone(), 8);
        self.known_from_value.insert(8, value.clone());
    }

    // requires (4)
    fn determine_nine(&mut self) {
        // get all patterns of len 6, find the one which is a superset of (4)
        let four_chars: HashSet<char> = self.charset_for_value(4);
        let nine = self
            .readings
            .iter()
            .filter(|x| x.len() == 6)
            .find(|possible_nine| {
                // (0) is not a superset of (4) but (9) is
                let charset = HashSet::from_iter(possible_nine.chars());
                charset.is_superset(&four_chars)
            })
            .unwrap();

        self.known_from_pattern.insert(nine.clone(), 9);
        self.known_from_value.insert(9, nine.clone());
    }

    fn determine_six(&mut self) {
        // get all patterns of len 6 and then it's the only one that's not a superset of (1)
        let one_chars: HashSet<char> = self.charset_for_value(1);

        let six = self
            .readings
            .iter()
            .filter(|value| value.len() == 6)
            .filter(|pattern| !self.pattern_for(pattern, 9))
            .find(|possible_six| {
                let charset = HashSet::from_iter(possible_six.chars());
                !charset.is_superset(&one_chars)
            })
            .unwrap();

        self.known_from_pattern.insert(six.clone(), 6);
        self.known_from_value.insert(6, six.clone());
    }

    // requires (6), (9)
    fn determine_zero(&mut self) {
        // get all patterns of len 6, remove (6) and (9) and it's what's left
        let zero = self
            .readings
            .iter()
            .filter(|x| x.len() == 6)
            .filter(|pattern| !self.pattern_for(pattern, 6))
            .find(|pattern| !self.pattern_for(pattern, 9))
            .unwrap();

        self.known_from_pattern.insert(zero.clone(), 0);
        self.known_from_value.insert(0, zero.clone());
    }

    // requires (1)
    fn determine_three(&mut self) {
        // get all patterns of len 5, find the one that's a superset of (1)
        let one_chars = self.charset_for_value(1);

        let three = self
            .readings
            .iter()
            .filter(|value| value.len() == 5)
            .find(|two_three_or_five| {
                let charset = HashSet::from_iter(two_three_or_five.chars());

                charset.is_superset(&one_chars)
            })
            .unwrap();

        self.known_from_pattern.insert(three.clone(), 3);
        self.known_from_value.insert(3, three.clone());
    }

    // requires (6)
    fn determine_two(&mut self) {
        // get all patterns of len 5, find the one with a set diff with (6) of len 2 (that is not 3)
        let six_chars = self.charset_for_value(6);
        let two = self
            .readings
            .iter()
            .filter(|value| value.len() == 5)
            .filter(|pattern| !self.pattern_for(pattern, 3))
            .find(|pattern| {
                let charset = HashSet::from_iter(pattern.chars());
                six_chars.difference(&charset).count() == 2
            })
            .unwrap();
        self.known_from_pattern.insert(two.clone(), 2);
        self.known_from_value.insert(2, two.clone());
    }

    // requires (6)
    fn determine_five(&mut self) {
        // get all patterns of len 5, where it's not the pattern for 3 or 2f
        let five = self
            .readings
            .iter()
            .filter(|value| value.len() == 5)
            .filter(|pattern| !self.pattern_for(pattern, 3))
            .find(|pattern| !self.pattern_for(pattern, 2))
            .unwrap();
        self.known_from_pattern.insert(five.clone(), 5);
        self.known_from_value.insert(5, five.clone());
    }
}
