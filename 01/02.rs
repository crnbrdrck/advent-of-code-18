use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Find the first frequency reached twice, potentially looping through the input multiple times
    println!("Answer 2 Tests");
    println!("{} should be 0", calc("+1\n-1"));
    println!("{} should be 10", calc("+3\n+3\n+4\n-2\n-4"));
    println!("{} should be 5", calc("-6\n+3\n+8\n+5\n-6"));
    println!("{} should be 14", calc("+7\n+7\n-2\n-7\n-4"));

    // Then read the input file and get the answer from that
    let mut file = File::open("input.txt").expect("File 'input.txt' could not be opened.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("File 'input.txt' could not be read.");
    println!("Puzzle Answer #2: {}", calc(&contents));
}

fn calc(input: &str) -> i32 {
    // Given an input, find the first frequency that appears twice
    // Use a recursive function that allows for passing in a hashmap
    return search(input, HashSet::new(), 0);
}

fn search(input: &str, mut found: HashSet<i32>, mut freq: i32) -> i32 {
    // Given an input string, split it on newlines and calculate the change in frequency until a duplicate is found
    // If we reach the end of input, recurse with the status of our currently found frequencies and our current frequency
    let diffs = input.split("\n"); // Creates an iterator which is fine by me
    for diff_str in diffs {
        // Trim the string and cast it to an integer
        if diff_str == "" {
            continue;
        }
        let diff = diff_str.trim().to_string().parse::<i32>().unwrap();
        freq += diff;
        // Check if the current frequency is in the HashSet, if it is return the frequency, if not keep going
        if found.contains(&freq) {
            return freq;
        }
        found.insert(freq);
    }
    // Recurse here if we haven't got our answer yet
    return search(input, found, freq);
}
