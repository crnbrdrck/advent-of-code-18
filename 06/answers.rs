use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Given a set of coords, determine the largest non-infinite area of closest cells in a infinite grid
    let test = "1, 1
    1, 6
    8, 3
    3, 4
    5, 5
    8, 9";
    // We won't know how big the view into the grid needs to be until we read all the coords
    println!("{} should be 17", calc(test));

    // Now do the input file
    let mut file = File::open("input.txt").expect("File 'input.txt' could not be opened.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("File 'input.txt' could not be read.");
    contents = contents.trim().to_string();
    println!("Puzzle Answer #1: {}", calc(&contents));

    println!("{} should be 16", calc2(test, 32));
    println!("Puzzle Answer #2: {}", calc2(&contents, 10000));
}

fn calc(input: &str) -> u32 {
    // Given the input which is a collection of coords, one per line, make up the visible space on an infinite grid and return the largest non infinte area
    // Since the grid we make is a finite view into an infinite grid, any coordinate that has a cell at the edge can be discounted before calculation

    // First, convert the string input into a vec of i32 tuples
    let (coords, max_x, max_y) = get_coords(input);

    // Loop from 0,0 to max_x,max_y and find which specified coord is the closest to each
    let mut disqualified: HashSet<i32> = HashSet::new();
    let mut cell_counts: HashMap<i32, i32> = HashMap::new();

    // Initialize the cell count hashmap
    for i in 0..coords.len() {
        cell_counts.insert(i as i32, 0);
    }

    // Loop through the grid and get the closest specified coord to the cell
    for x in 0..=max_x {
        for y in 0..=max_y {
            // Check if we need to remember to not count whatever specified location is returned
            let to_disqualify = x == 0 || x == max_x || y == 0 || y == max_y;

            // Get the closest coord to the current cell
            if let Some(ref coord) = get_closest((x, y), &coords) {
                // At this point (I think) we know that coord has the value of the index to give the cell to
                *cell_counts.get_mut(&*coord).unwrap() += 1;

                // Also check if we should disqualify this coord index
                if to_disqualify {
                    disqualified.insert(*coord);
                }
            }
        }
    }

    // Go through the non-disqualified keys and find the largest area
    let mut largest = 0;
    for (k, count) in cell_counts {
        if disqualified.contains(&k) {
            continue;
        }
        if count > largest {
            largest = count;
        }
    }
    return largest as u32;
}

fn calc2(input: &str, max_dist: u32) -> u32 {
    // Alternatively now we want to find the area of the region where all the cells are at most `max_dist` from all of the input coords
    // The max_dist is the maximum that the sum of the distances from a cell to all the specified coords can be

    // Start by getting the grid size and coords properly
    let (coords, max_x, max_y) = get_coords(input);

    // For each cell, get the sum of the distances to each of the coords and check against the allowed max
    let mut cells = 0;

    for x in 0..=max_x {
        for y in 0..=max_y {
            let dist_sum: u32 = (&coords).into_iter().map(|c| manhattan((x, y), *c)).sum();
            if dist_sum < max_dist {
                cells += 1;
            }
        }
    }

    return cells as u32;
}

fn get_coords(input: &str) -> (Vec<(i32, i32)>, i32, i32) {
    // Given the string kind of input, return a vector of i32 tuples representing the coords, as well as the largest x and y values found
    let mut vec: Vec<(i32, i32)> = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for mut line in input.split("\n") {
        let str_coords:Vec<&str> = line.trim().split(", ").collect();
        // Split on ", " and parse the two numbers
        let x = str_coords[1].parse::<i32>().unwrap();
        let y = str_coords[0].parse::<i32>().unwrap();
        vec.push((x, y));
        if x > max_x {
            max_x = x;
        }
        if y > max_y {
            max_y = y;
        }
    }
    return (vec, max_x, max_y);
}

fn get_closest((x, y): (i32, i32), coords: &Vec<(i32, i32)>) -> Option<i32> {
    // Given a specified (x,y) cell, find the closest coord pair in coords to it and return its index
    // If theres a tie, return nil
    let mut closest_dist: u32 = std::u32::MAX;
    let mut closest_count = 0;
    let mut closest_index = 0;
    for i in 0..coords.len() {
        let coord = coords[i];
        let dist = manhattan((x, y), coord);
        // If dist is the same as the closest, we have a tie
        if dist == closest_dist {
            closest_count += 1;
        }
        if dist < closest_dist {
            closest_dist = dist;
            closest_index = i;
            closest_count = 1;
        }
    }
    if closest_count == 1 {
        return Some(closest_index as i32);
    }
    else {
        return None;
    }
}

fn manhattan((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> u32 {
    // Given two coordinate pairs, return the manhattan distance between them
    return ((x1 - x2).abs() + (y1 - y2).abs()) as u32;
}
