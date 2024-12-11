#![allow(dead_code)]
use crate::util;

pub fn solve1(session_cookie: &str) {
    let url = "https://adventofcode.com/2024/day/5/input";
    match util::get_input(url, session_cookie) {
        Ok(input) => {
            let result = process_input_1(input.trim());
            println!("Sum of middle numbers in valid lists: {}", result);
        }
        Err(err) => eprintln!("Error fetching input for Day 05 - Part 1: {}", err),
    }
}

pub fn solve2(session_cookie: &str) {
    let url = "https://adventofcode.com/2024/day/5/input";
    match util::get_input(url, session_cookie) {
        Ok(input) => {
            let result = process_input_2(input.trim());
            println!("Sum of middle numbers in invalid corrected lists: {}", result);
        }
        Err(err) => eprintln!("Error fetching input for Day 05 - Part 2: {}", err),
    }
}

pub fn process_input_2(input: &str) -> i32 {
    let mut sections = input.split("\n\n");

    // Parse rules
    let rules: Vec<(i32, i32)> = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let parts: Vec<i32> = line.split('|').map(|s| s.parse().unwrap()).collect();
            (parts[0], parts[1])
        })
        .collect();

    // Parse lists
    let lists: Vec<Vec<i32>> = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    let mut sum_of_middle_numbers = 0;

    for list in &lists {
        if !is_valid_list(list, &rules) {
            let corrected_list = correct_order(list.clone(), &rules);
            sum_of_middle_numbers += corrected_list[list.len()/2];
        }
    }

    sum_of_middle_numbers
}

pub fn process_input_1(input: &str) -> i32 {
    println!("Input data:\n{}", input);
    let mut sections = input.split("\n\n");

    // Parse rules
    let rules: Vec<(i32, i32)> = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let parts: Vec<i32> = line.split('|').map(|s| s.parse().unwrap()).collect();
            (parts[0], parts[1])
        })
        .collect();

    // Parse lists
    let lists: Vec<Vec<i32>> = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    let mut sum_of_middle_numbers = 0;

    for list in &lists {
        if is_valid_list(list, &rules) {
            //println!("valid list:\n{:#?}", list);
            //println!("middle is: {}", list.len()/2);
            sum_of_middle_numbers += list[list.len()/2];
        }
    }

    sum_of_middle_numbers
}

fn is_valid_list(list: &[i32], rules: &[(i32, i32)]) -> bool {
    for &(a, b) in rules {
        let pos_a = list.iter().position(|&x| x == a);
        let pos_b = list.iter().position(|&x| x == b);

        if let (Some(index_a), Some(index_b)) = (pos_a, pos_b) {
            if index_a > index_b {
                return false; // Rule violated
            }
        }
    }
    true
}

fn correct_order(mut list: Vec<i32>, rules: &[(i32, i32)]) -> Vec<i32> {
    list.sort_by(|&a, &b| {
        // Compare each pair of numbers based on the rules
        for &(rule_a, rule_b) in rules {
            if a == rule_a && b == rule_b {
                return std::cmp::Ordering::Less;
            } else if a == rule_b && b == rule_a {
                return std::cmp::Ordering::Greater;
            }
        }
        std::cmp::Ordering::Equal // If no rules apply, leave order unchanged
    });

    list
}
