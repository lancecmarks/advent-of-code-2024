#![allow(dead_code)]
use crate::util;

pub fn solve1(session_cookie: &str) {
    let url = "https://adventofcode.com/2024/day/2/input";
    match util::get_input(url, session_cookie) {
        Ok(input) => {
            let count = process_input_1(&input);

            println!("Number of safe rows: {}", count);
        }
        Err(err) => eprintln!("Error fetching input for Day 02 - Part 1: {}", err),
    }
}

pub fn solve2(session_cookie: &str) {
    let url = "https://adventofcode.com/2024/day/2/input";
    match util::get_input(url, session_cookie) {
        Ok(input) => {
            let count = process_input_2(&input);

            println!("Number of safe rows (with removal): {}", count);
        }
        Err(err) => eprintln!("Error fetching input for Day 02 - Part 2: {}", err),
    }
}

pub fn process_input_1(input: &str) -> usize {
    // Parse the input into rows of numbers
    let rows: Vec<Vec<i32>> = input
        .lines()
        .filter_map(|line| {
            let numbers: Option<Vec<i32>> = line
                .split_whitespace()
                .map(|n| n.parse::<i32>().ok())
                .collect();
            numbers
        })
        .collect();

    // Count rows that are consistently increasing or decreasing
    let mut safe_row_count = 0;

    for row in rows {
        if is_consistent(&row) {
            safe_row_count += 1;
        }
    }

    safe_row_count
}

pub fn process_input_2(input: &str) -> usize {
    // Parse the input into rows of numbers
    let rows: Vec<Vec<i32>> = input
        .lines()
        .filter_map(|line| {
            let numbers: Option<Vec<i32>> = line
                .split_whitespace()
                .map(|n| n.parse::<i32>().ok())
                .collect();
            numbers
        })
        .collect();

    // Count rows that are safe, considering removal of one number
    let mut safe_row_count = 0;

    for row in rows {
        if is_consistent(&row) || is_consistent_with_removal(&row) {
            safe_row_count += 1;
        }
    }

    safe_row_count
}

fn is_consistent(row: &[i32]) -> bool {
    let mut differences = vec![];

    for pair in row.windows(2) {
        differences.push(pair[1] - pair[0]);
    }

    let all_increasing = differences.iter().all(|&diff| diff >= 1 && diff <= 3);
    let all_decreasing = differences.iter().all(|&diff| diff <= -1 && diff >= -3);

    all_increasing || all_decreasing
}

fn is_consistent_with_removal(row: &[i32]) -> bool {
    for i in 0..row.len() {
        // Create a new row without the i-th element
        let mut modified_row = row.to_vec();
        modified_row.remove(i);

        if is_consistent(&modified_row) {
            return true;
        }
    }

    false
}
