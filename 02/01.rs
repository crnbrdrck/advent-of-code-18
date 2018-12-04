use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Find the number of ids that contain exactly 2 of any letter, and exactly 3 of any letter
    // Multiply these counts by eachother to get the checksum

    // First do a test to ensure we get the right answer
    let test_input = "abcdef
    bababc
    abbcde
    abcccd
    aabcdd
    abcdee
    ababab";
    println!("{} should equal 12", calc(&test_input));

    // Then open the puzzle input file and calculate my answer
    let mut file = File::open("input.txt").expect("File 'input.txt' could not be opened.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("File 'input.txt' could not be read.");
    println!("Puzzle Answer #1: {}", calc(&contents));
}

fn calc(input: &str) -> i32 {
    // Given an input broken up by newlines, calculate the numbers of 2 and 3 char ids and get the checksum
    let mut two_count = 0;
    let mut three_count = 0;
    let ids = input.split("\n");
    for id in ids {
        // Check does it count for 2 and / or 3 characters
        if check_id(&id, 2) {
            two_count += 1;
        }
        if check_id(&id, 3) {
            three_count += 1;
        }
    }
    return two_count * three_count;
}

fn check_id(id: &str, count: i32) -> bool {
    // Check if the supplied `id` has a letter that appears exactly `count` times
    for char in id.chars() {
        if id.matches(char).count() as i32 == count {
            return true;
        }
    }
    return false;
}
