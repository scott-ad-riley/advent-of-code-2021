use std::collections::VecDeque;

fn main() {
    let s = include_str!("input.txt");

    let lines: Vec<&str> = s.split('\n').collect();

    let illegal_chars: Vec<char> = lines
        .iter()
        .filter_map(|line| first_illegal_char(line))
        .collect();

    println!("result={:?}", illegal_chars);

    let score: usize = illegal_chars.into_iter().map(score).sum();
    println!("score={}", score)
}

fn score(bracket: char) -> usize {
    match bracket {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("unrecognised bracket {}", bracket),
    }
}

fn first_illegal_char(line: &str) -> Option<char> {
    let mut bracket_stack: VecDeque<char> = VecDeque::new();
    for char in line.chars() {
        if bracket_stack.is_empty() {
            bracket_stack.push_back(char);
        } else if is_closing(char) && !can_close(char, *bracket_stack.back().unwrap()) {
            return Some(char);
        } else if is_closing(char) && can_close(char, *bracket_stack.back().unwrap()) {
            bracket_stack.pop_back();
        } else {
            bracket_stack.push_back(char);
        }
    }

    None
}

fn can_close(bracket: char, target: char) -> bool {
    matches!(
        (bracket, target),
        (']', '[') | (')', '(') | ('>', '<') | ('}', '{')
    )
}

fn is_closing(bracket: char) -> bool {
    matches!(bracket, ']' | ')' | '>' | '}')
}
