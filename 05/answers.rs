use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Given some input, remove touching characters with opposing polarities (x and X), keep going until all removed and return the length
    println!("Tests for Answer 1");
    println!("{} should be 0", calc(&mut String::from("aA")));
    println!("{} should be 0", calc(&mut String::from("abBA")));
    println!("{} should be 4", calc(&mut String::from("abAB")));
    println!("{} should be 6", calc(&mut String::from("aabAAB")));
    println!("{} should be 10", calc(&mut String::from("dabAcCaCBAcCcaDA")));

    // Finally, load the test input and get the answer
    let mut file = File::open("input.txt").expect("File 'input.txt' could not be opened.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("File 'input.txt' could not be read.");
    println!("Puzzle Answer #1: {}", calc(&mut contents));

    // For part 2, iterate through the string, remove each of the characters and calc the len, find the smallest
    println!("Tests for Answer 2");
    println!("{} should be 4", calc2(&mut String::from("dabAcCaCBAcCcaDA")));
    println!("Puzzle Answer #2: {}", calc2(&mut contents));
}

fn calc(input: &mut String) -> u32 {
    // Given an input, keep removing touching opposing polarity characters until there has been no change, then return the length
    let mut found = false;
    let mut checked = String::from(input.clone());
    while !found {
        let curr = remove_polarities(&mut checked).to_string();
        found = curr.len() == checked.len();
        checked = curr;
    }
    return checked.trim().len() as u32;
}

fn calc2(input: &mut String) -> u32 {
    // Using the existing calc function, find the smallest possible collapsed polymer by removing a unit and collapsing the polymer
    let mut checked: HashSet<String> = HashSet::new();
    let mut shortest: u32 = std::u32::MAX;
    for c in input.chars() {
        if checked.contains(&c.to_lowercase().to_string()) {
            continue;
        }
        // Clone the contents, remove the character and check the polymer length
        checked.insert(c.to_lowercase().to_string());
        let mut clone = input.replace(&c.to_lowercase().to_string(), "").replace(&c.to_uppercase().to_string(), "");
        let length = calc(&mut clone);
        if length < shortest {
            shortest = length;
        }
    }
    return shortest;
}

fn remove_polarities(input: &mut String) -> &mut String {
    // If we iterate from end to start, we can splice out the indices as we meet them
    for i in (1..input.len()).rev() {
        // Check that we haven't gone out of bounds with i + 1;
        if i + 1 >= input.len() {
            continue;
        }
        let c0 = &input[i..i+1].chars().next().unwrap();
        let c1 = &input[i-1..i].chars().next().unwrap();
        if (c0.is_lowercase() && c1.is_uppercase()) || (c0.is_uppercase() && c1.is_lowercase()) {
            if c0.to_lowercase().next().unwrap() == c1.to_lowercase().next().unwrap() {
                // Remove the characters at position i and i - 1
                input.remove(i);
                input.remove(i - 1);
            }
        }
    }
    return input;
}
