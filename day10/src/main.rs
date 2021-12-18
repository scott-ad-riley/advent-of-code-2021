use std::collections::VecDeque;

fn main() {
    let s = include_str!("input.txt");

    let lines: Vec<&str> = s.split('\n').collect();

    let illegal_chars: Vec<Vec<char>> = lines
        .iter()
        .filter_map(|line| remaining_required_chars(line))
        .collect();

    let mut scores: Vec<usize> = illegal_chars
        .iter()
        .map(|chars| chars.iter().rfold(0, |acc, char| (acc * 5) + score(*char)))
        .collect();

    scores.sort_unstable();

    println!("result={}", scores[scores.len() / 2])
}

fn score(bracket: char) -> usize {
    match bracket {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("unrecognised bracket {}", bracket),
    }
}

fn remaining_required_chars(line: &str) -> Option<Vec<char>> {
    let mut bracket_stack: VecDeque<char> = VecDeque::new();
    for char in line.chars() {
        if bracket_stack.is_empty() {
            bracket_stack.push_back(char);
        } else if is_closing(char) && !can_close(char, *bracket_stack.back().unwrap()) {
            return None;
        } else if is_closing(char) && can_close(char, *bracket_stack.back().unwrap()) {
            bracket_stack.pop_back();
        } else {
            bracket_stack.push_back(char);
        }
    }

    if bracket_stack.is_empty() {
        return None;
    }

    let mut completion = vec![];

    for bracket in bracket_stack {
        completion.push(matching(bracket));
    }

    Some(completion)
}

fn can_close(bracket: char, target: char) -> bool {
    matches!(
        (bracket, target),
        (']', '[') | (')', '(') | ('>', '<') | ('}', '{')
    )
}

fn matching(bracket: char) -> char {
    match bracket {
        '(' => ')',
        '<' => '>',
        '{' => '}',
        '[' => ']',
        _ => panic!("unkwown bracket type"),
    }
}

fn is_closing(bracket: char) -> bool {
    matches!(bracket, ']' | ')' | '>' | '}')
}
