#![allow(dead_code)]
use crate::util;
use std::collections::HashSet;

pub fn solve1(session_cookie: &str) {
    let url = "https://adventofcode.com/2024/day/6/input";
    match util::get_input(url, session_cookie) {
        Ok(input) => {
            let result = process_input_1(input.trim());
            println!("Total Xs: {}", result.1);
        
            println!("Annotated map:\n{}", result.0);
        }
        Err(err) => eprintln!("Error fetching input for Day 06 - Part 1: {}", err),
    }
}

pub fn solve2(session_cookie: &str) {
    let url = "https://adventofcode.com/2024/day/6/input";
    match util::get_input(url, session_cookie) {
        Ok(input) => {
            let result = process_input_2(input.trim());
            println!("Total Os: {}", result.1);
        
            println!("Annotated map:\n{}", result.0);
        }
        Err(err) => eprintln!("Error fetching input for Day 06 - Part 2: {}", err),
    }
}

pub fn process_input_1(input: &str) -> (String, usize) {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // Find the starting position of the player
    let (mut x, mut y, mut direction) = find_player(&grid);

    let mut count_xs = 0;

    loop {
        // Mark current position with X
        if grid[x][y] != 'X' {
            grid[x][y] = 'X';
            count_xs += 1;
        }

        // Move in the current direction
        let (dx, dy) = get_direction_vector(direction);
        let (nx, ny) = (x as isize + dx, y as isize + dy);

        if nx < 0
            || ny < 0
            || nx >= grid.len() as isize
            || ny >= grid[0].len() as isize
            || grid[nx as usize][ny as usize] == '#'
        {
            // Turn right and stay in the same position
            direction = turn_right(direction);
        } else {
            // Update position to new valid position
            x = nx as usize;
            y = ny as usize;
        }

        // Check if we've gone off the map
        if nx < 0 || ny < 0 || nx >= grid.len() as isize || ny >= grid[0].len() as isize {
            break;
        }
    }

    // Create the annotated map
    let annotated_map: String = grid
        .into_iter()
        .map(|row| row.into_iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");

    (annotated_map, count_xs)
}

pub fn process_input_2(input: &str) -> (String, usize) {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let player_path = track_player_path(&grid);

    let mut cycle_positions = 0;

    for &(x, y) in &player_path {
        if grid[x][y] == '.' && causes_cycle(&mut grid, x, y) {
            grid[x][y] = 'O'; // Mark as a cycle-causing obstacle
            cycle_positions += 1;
        }
    }

    let annotated_map: String = grid
        .into_iter()
        .map(|row| row.into_iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");

    (annotated_map, cycle_positions)
}

fn track_player_path(grid: &[Vec<char>]) -> HashSet<(usize, usize)> {
    let (mut x, mut y, mut direction) = find_player(grid);
    let mut path = HashSet::new();

    loop {
        path.insert((x, y));

        let (dx, dy) = get_direction_vector(direction);
        let (nx, ny) = (x as isize + dx, y as isize + dy);

        if nx < 0
            || ny < 0
            || nx >= grid.len() as isize
            || ny >= grid[0].len() as isize
            || grid[nx as usize][ny as usize] == '#'
        {
            direction = turn_right(direction); // Turn right at an obstacle
        } else {
            x = nx as usize;
            y = ny as usize; // Move forward
        }

        if nx < 0 || ny < 0 || nx >= grid.len() as isize || ny >= grid[0].len() as isize {
            break; // Stop if out of bounds
        }
    }

    path
}

fn causes_cycle(grid: &mut [Vec<char>], x: usize, y: usize) -> bool {
    // Temporarily place an obstacle at (x, y)
    grid[x][y] = '#';

    let (mut px, mut py, mut direction) = find_player(grid);
    let mut visited = HashSet::new();

    loop {
        if !visited.insert((px, py, direction)) {
            // Cycle detected: revisiting the same position with the same direction
            grid[x][y] = '.'; // Restore the original state
            return true;
        }

        let (dx, dy) = get_direction_vector(direction);
        let (nx, ny) = (px as isize + dx, py as isize + dy);

        if nx < 0
            || ny < 0
            || nx >= grid.len() as isize
            || ny >= grid[0].len() as isize
            || grid[nx as usize][ny as usize] == '#'
        {
            direction = turn_right(direction); // Turn right at an obstacle
        } else {
            px = nx as usize;
            py = ny as usize; // Move forward
        }

        if nx < 0 || ny < 0 || nx >= grid.len() as isize || ny >= grid[0].len() as isize {
            break; // Stop if out of bounds
        }
    }

    grid[x][y] = '.'; // Restore the original state
    false
}

fn find_player(grid: &[Vec<char>]) -> (usize, usize, char) {
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if "^>v<".contains(cell) {
                return (i, j, cell);
            }
        }
    }
    panic!("Player not found in the grid!");
}

fn get_direction_vector(direction: char) -> (isize, isize) {
    match direction {
        '^' => (-1, 0), // Up
        '>' => (0, 1),  // Right
        'v' => (1, 0),  // Down
        '<' => (0, -1), // Left
        _ => panic!("Invalid direction!"),
    }
}

fn turn_right(direction: char) -> char {
    match direction {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => panic!("Invalid direction!"),
    }
}
