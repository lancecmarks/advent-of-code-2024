#![allow(dead_code)]
use crate::util;
use std::collections::HashMap;

pub fn solve1(session_cookie: &str) {
    let url = "https://adventofcode.com/2024/day/1/input";
    match crate::util::get_input(url, session_cookie) {
        Ok(input) => {
            let (column1, column2, differences, sum_of_differences) = process_input_1(&input);

            println!("Column 1 (sorted): {:?}", column1);
            println!("Column 2 (sorted): {:?}", column2);
            println!("Absolute Differences: {:?}", differences);
            println!("Sum of Differences: {}", sum_of_differences);

            // Additional logic for part 1
        }
        Err(err) => eprintln!("Error fetching input for Day 01 - Part 1: {}", err),
    }
}

pub fn solve2(session_cookie: &str) {
    let url = "https://adventofcode.com/2024/day/1/input";
    match util::get_input(url, session_cookie) {
        Ok(input) => {
            let (column1, column2, similarity_scores, sum_of_scores) = process_input_2(&input);

            println!("Column 1 (sorted): {:?}", column1);
            println!("Column 2 (sorted): {:?}", column2);
            println!("Similarity Scores: {:?}", similarity_scores);
            println!("Sum of Differences: {}", sum_of_scores);

            // Additional logic for part 1
        }
        Err(err) => eprintln!("Error fetching input for Day 01 - Part 2: {}", err),
    }
}

pub fn process_input_1(input: &str) -> (Vec<i32>, Vec<i32>, Vec<i32>, i32) {
    let mut column1 = Vec::new();
    let mut column2 = Vec::new();

    println!("Input data:\n{}", input);

    // Parse input into two columns
    for line in input.lines() {
        println!("Processing line: '{}'", line);
        if let Some((c1, c2)) = line.split_once(char::is_whitespace) {
            let c1 = c1.trim();
            let c2 = c2.trim();
            if let (Ok(num1), Ok(num2)) = (c1.parse::<i32>(), c2.parse::<i32>()) {
                println!("Parsed numbers: {} and {}", num1, num2);
                column1.push(num1);
                column2.push(num2);
            } else {
                println!("Failed to parse numbers: '{}' and '{}'", c1, c2);
            }
        } else {
            println!("Failed to split line: '{}'", line);
        }
    }

    // Sort columns
    column1.sort_unstable();
    column2.sort_unstable();

    // Calculate absolute differences
    let differences: Vec<i32> = column1
        .iter()
        .zip(column2.iter())
        .map(|(a, b)| (a - b).abs())
        .collect();

    // Calculate the sum of differences
    let sum_of_differences: i32 = differences.iter().sum();

    (column1, column2, differences, sum_of_differences)
}

pub fn process_input_2(input: &str) -> (Vec<i32>, Vec<i32>, Vec<i32>, i32) {
    let mut column1 = Vec::new();
    let mut column2 = Vec::new();

    println!("Input data:\n{}", input);

    // Parse input into two columns
    for line in input.lines() {
        println!("Processing line: '{}'", line);
        if let Some((c1, c2)) = line.split_once(char::is_whitespace) {
            let c1 = c1.trim();
            let c2 = c2.trim();
            if let (Ok(num1), Ok(num2)) = (c1.parse::<i32>(), c2.parse::<i32>()) {
                println!("Parsed numbers: {} and {}", num1, num2);
                column1.push(num1);
                column2.push(num2);
            } else {
                println!("Failed to parse numbers: '{}' and '{}'", c1, c2);
            }
        } else {
            println!("Failed to split line: '{}'", line);
        }
    }

    // Sort columns
    column1.sort_unstable();
    column2.sort_unstable();

    // Count occurrences in column2
    let mut count_map = HashMap::new();
    for &num in &column2 {
        *count_map.entry(num).or_insert(0) += 1;
    }

    // Calculate similarity scores
    let similarity_scores: Vec<i32> = column1
        .iter()
        .map(|&num| {
            let count = count_map.get(&num).copied().unwrap_or(0);
            num * count
        })
        .collect();

    // Calculate the sum of similarity_scores
    let sum_of_scores: i32 = similarity_scores.iter().sum();

    (column1, column2, similarity_scores, sum_of_scores)
}

