#![allow(dead_code)]
use crate::util;

pub fn solve1(session_cookie: &str) {
    let url = "https://adventofcode.com/2024/day/4/input";
    match util::get_input(url, session_cookie) {
        Ok(input) => {
            let result = process_input_1(input.trim());
            println!("Total XMAS found: {}", result.1);
        
            println!("Annotated grid:\n{}", result.0);
        }
        Err(err) => eprintln!("Error fetching input for Day 01 - Part 1: {}", err),
    }
}

pub fn solve2(session_cookie: &str) {
    let url = "https://adventofcode.com/2024/day/4/input";
    match util::get_input(url, session_cookie) {
        Ok(input) => {
            let result = process_input_2(input.trim());
            println!("Total XMAS found: {}", result.1);
        
            println!("Annotated grid:\n{}", result.0);
        }
        Err(err) => eprintln!("Error fetching input for Day 01 - Part 1: {}", err),
    }
}

pub fn process_input_1(input: &str) -> (String, usize) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut annotated_grid = vec![vec!['.'; grid[0].len()]; grid.len()];
    let directions = [
        (0, 1),   // Horizontal right
        (0, -1),  // Horizontal left
        (1, 0),   // Vertical down
        (-1, 0),  // Vertical up
        (1, 1),   // Diagonal down-right
        (-1, -1), // Diagonal up-left
        (1, -1),  // Diagonal down-left
        (-1, 1),  // Diagonal up-right
    ];

    let mut count = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            for &(di, dj) in &directions {
                if matches_pattern(&grid, i as isize, j as isize, di, dj, "XMAS") {
                    count += 1;

                    // Annotate the grid
                    for k in 0..4 {
                        let ni = (i as isize + k as isize * di) as usize;
                        let nj = (j as isize + k as isize * dj) as usize;
                        annotated_grid[ni][nj] = grid[ni][nj];
                    }
                }
            }
        }
    }

    let annotated_output: String = annotated_grid
        .into_iter()
        .map(|row| row.into_iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");

    (annotated_output, count)
}

pub fn process_input_2(input: &str) -> (String, usize) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut annotated_grid = vec![vec!['.'; grid[0].len()]; grid.len()];
    let mut count = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if matches_mas(&grid, i, j) {
                count += 1;

                // Annotate the grid
                annotated_grid[i][j] = grid[i][j]; // Center (A)
                if i > 0 && j > 0 {
                    annotated_grid[i - 1][j - 1] = grid[i - 1][j - 1]; // Top-left (M)
                }
                if i > 0 && j + 1 < grid[0].len() {
                    annotated_grid[i - 1][j + 1] = grid[i - 1][j + 1]; // Top-right (M)
                }
                if i + 1 < grid.len() && j > 0 {
                    annotated_grid[i + 1][j - 1] = grid[i + 1][j - 1]; // Bottom-left (S)
                }
                if i + 1 < grid.len() && j + 1 < grid[0].len() {
                    annotated_grid[i + 1][j + 1] = grid[i + 1][j + 1]; // Bottom-right (S)
                }
            }
        }
    }

    let annotated_output: String = annotated_grid
        .into_iter()
        .map(|row| row.into_iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");

    (annotated_output, count)
}


fn matches_pattern(
    grid: &[Vec<char>],
    mut i: isize,
    mut j: isize,
    di: isize,
    dj: isize,
    pattern: &str,
) -> bool {
    for ch in pattern.chars() {
        if i < 0 || i >= grid.len() as isize || j < 0 || j >= grid[0].len() as isize {
            return false;
        }

        if grid[i as usize][j as usize] != ch {
            return false;
        }

        i += di;
        j += dj;
    }

    true
}

fn matches_mas(grid: &[Vec<char>], i: usize, j: usize) -> bool {
    // Define the four rotations of the "MAS" pattern
    let patterns = [
        [(0, 0, 'A'), (-1, -1, 'M'), (-1, 1, 'M'), (1, -1, 'S'), (1, 1, 'S')], // Original
        [(0, 0, 'A'), (-1, -1, 'S'), (-1, 1, 'S'), (1, -1, 'M'), (1, 1, 'M')], // Rotated 90
        [(0, 0, 'A'), (-1, -1, 'M'), (-1, 1, 'S'), (1, -1, 'M'), (1, 1, 'S')], // Rotated 180
        [(0, 0, 'A'), (-1, -1, 'S'), (-1, 1, 'M'), (1, -1, 'S'), (1, 1, 'M')], // Rotated 270
    ];

    for pattern in patterns.iter() {
        if pattern.iter().all(|&(di, dj, ch)| {
            let ni = i as isize + di;
            let nj = j as isize + dj;
            ni >= 0
                && ni < grid.len() as isize
                && nj >= 0
                && nj < grid[0].len() as isize
                && grid[ni as usize][nj as usize] == ch
        }) {
            return true;
        }
    }

    false
}

