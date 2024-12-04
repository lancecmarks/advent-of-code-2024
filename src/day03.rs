#![allow(dead_code)]
use crate::util;
use regex::Regex;

pub fn solve1(session_cookie: &str) {
    let url = "https://adventofcode.com/2024/day/3/input";
    match util::get_input(url, session_cookie) {
        Ok(input) => {
            println!("Fetched input: {}", input);
            let result = process_input_1(&input);

            println!("Sum of all products: {}", result);
        }
        Err(err) => eprintln!("Error fetching input for Day 03 - Part 1: {}", err),
    }
}

pub fn solve2(session_cookie: &str) {
    let url = "https://adventofcode.com/2024/day/3/input";
    match util::get_input(url, session_cookie) {
        Ok(input) => {
            println!("Fetched input: {}", input);
            let result = process_input_2(&input);

            println!("Sum of all products: {}", result);
        }
        Err(err) => eprintln!("Error fetching input for Day 03 - Part 2: {}", err),
    }
}

pub fn process_input_1(input: &str) -> i32 {
    // Regex to match "mul(3d,3d)" where 3d are 1-to-3 digit numbers
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    // Initialize the sum
    let mut total_sum = 0;

    // Find all matches in the input string
    for cap in re.captures_iter(input) {
        // Capture group 1 and 2 are the two numbers
        let num1: i32 = cap[1].parse().unwrap();
        let num2: i32 = cap[2].parse().unwrap();

        // Multiply and add to the sum
        total_sum += num1 * num2;
    }

    total_sum
}

pub fn process_input_2(input: &str) -> i32 {
    // Combined regex to match mul(3d,3d), do(), and don't()
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    let mut total_sum = 0;
    let mut do_multiplication = true; // Start with default `do()` logic

    for cap in re.captures_iter(input) {
        if let Some(mul_match) = cap.get(1) {
            // If it's a mul(3d,3d) match, get the numbers
            let num1: i32 = mul_match.as_str().parse().unwrap();
            let num2: i32 = cap.get(2).unwrap().as_str().parse().unwrap();

            if do_multiplication {
                let product = num1 * num2;
                total_sum += product;
                println!("Adding product of {} and {}: {}", num1, num2, product);
            } else {
                println!("Skipping product of {} and {}", num1, num2);
            }
        } else if let Some(toggle) = cap.get(0) {
            // If it's a do() or don't(), toggle the switch
            match toggle.as_str() {
                "do()" => {
                    do_multiplication = true;
                    println!("Toggled to do()");
                }
                "don't()" => {
                    do_multiplication = false;
                    println!("Toggled to don't()");
                }
                _ => {}
            }
        }
    }

    total_sum
}
