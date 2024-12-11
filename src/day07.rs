#![allow(dead_code)]
use crate::util;

pub fn solve1(session_cookie: &str) {
    let url = "https://adventofcode.com/2024/day/7/input";
    match util::get_input(url, session_cookie) {
        Ok(input) => {
            let result = process_input_1(input.trim());
            println!("Sum of valid targets: {}", result);
        }
        Err(err) => eprintln!("Error fetching input for Day 07 - Part 1: {}", err),
    }
}

pub fn solve2(session_cookie: &str) {
    let url = "https://adventofcode.com/2024/day/7/input";
    match util::get_input(url, session_cookie) {
        Ok(input) => {
            let result = process_input_2(input.trim());
            println!("Sum of valid targets: {}", result);
        }
        Err(err) => eprintln!("Error fetching input for Day 07 - Part 2: {}", err),
    }
}

fn process_input_1(input: &str) -> i64 {
    let mut total_sum = 0;

    for line in input.lines() {
        // Split each line into target and numbers
        if let Some((target, numbers)) = line.split_once(':') {
            let target: i64 = target.trim().parse().expect("Invalid target number");
            let numbers: Vec<i64> = numbers
                .trim()
                .split_whitespace()
                .map(|n| n.parse().expect("Invalid number"))
                .collect();

            // Check if any combination of operations matches the target
            if can_evaluate_to_target(&numbers, target, 0, numbers[0]) {
                total_sum += target;
            }
        }
    }

    total_sum
}

fn process_input_2(input: &str) -> i64 {
    let mut total_sum = 0;

    for line in input.lines() {
        // Split each line into target and numbers
        if let Some((target, numbers)) = line.split_once(':') {
            let target: i64 = target.trim().parse().expect("Invalid target number");
            let numbers: Vec<i64> = numbers
                .trim()
                .split_whitespace()
                .map(|n| n.parse().expect("Invalid number"))
                .collect();

            // Check if any combination of operations matches the target
            if can_evaluate_to_target_with_concat(&numbers, target, 0, numbers[0]) {
                total_sum += target;
            }
        }
    }

    total_sum
}

fn can_evaluate_to_target_with_concat(
    numbers: &[i64],
    target: i64,
    index: usize,
    current_value: i64,
) -> bool {
    if index == numbers.len() - 1 {
        // Base case: we've reached the end of the sequence
        return current_value == target;
    }

    let next_value = numbers[index + 1];

    // Try addition
    if let Some(new_value) = current_value.checked_add(next_value) {
        if can_evaluate_to_target_with_concat(numbers, target, index + 1, new_value) {
            return true;
        }
    }

    // Try multiplication
    if let Some(new_value) = current_value.checked_mul(next_value) {
        if can_evaluate_to_target_with_concat(numbers, target, index + 1, new_value) {
            return true;
        }
    }

    // Try concatenation
    if let Some(new_value) = concat_numbers(current_value, next_value) {
        if can_evaluate_to_target_with_concat(numbers, target, index + 1, new_value) {
            return true;
        }
    }

    false
}

fn concat_numbers(left: i64, right: i64) -> Option<i64> {
    let right_as_string = right.to_string();
    let concatenated = format!("{left}{right_as_string}");
    concatenated.parse::<i64>().ok()
}

fn can_evaluate_to_target(numbers: &[i64], target: i64, index: usize, current_value: i64) -> bool {
    if index == numbers.len() - 1 {
        // Base case: we've reached the end of the sequence
        return current_value == target;
    }

    let next_value = numbers[index + 1];

    // Try addition, ensuring no overflow
    if let Some(new_value) = current_value.checked_add(next_value) {
        if can_evaluate_to_target(numbers, target, index + 1, new_value) {
            return true;
        }
    }

    // Try multiplication, ensuring no overflow
    if let Some(new_value) = current_value.checked_mul(next_value) {
        if can_evaluate_to_target(numbers, target, index + 1, new_value) {
            return true;
        }
    }

    false
}
