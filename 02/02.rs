use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Find the 2 ids that differ by exactly one letter, and then return the common letters between the two ids

    // First do a test to ensure we get the right answer
    let test_input = "abcde
    fghij
    klmno
    pqrst
    fguij
    axcye
    wvxyz";
    println!("{} should be 'fgij'", get_common_characters(calc(&test_input)));

    // Then open the puzzle input file and calculate my answer
    let mut file = File::open("input.txt").expect("File 'input.txt' could not be opened.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("File 'input.txt' could not be read.");
    println!("Part 2 Answer: {}", get_common_characters(calc(&contents)));
}

fn get_common_characters((str1, str2): (String, String)) -> String {
    // Given two strings, return a string made of the characters in the two strings that are the same in their positions
    let mut output = String::new();
    for i in 0..str1.len() {
        let c0 = &str1[i..i+1];
        let c1 = &str2[i..i+1];
        if c0 == c1 {
            output.push_str(c0);
        }
    }
    return output;
}

fn calc(input: &str) -> (String, String) {
    // Given an input broken up by newlines, find the two ids that differ by only a single character and print them out

    // This time we'll need to collect into a vector
    let ids: Vec<&str> = input.split("\n").collect();
    for index0 in 0..ids.len() {
        for index1 in index0..ids.len() {
            let id0 = ids[index0].trim().to_string();
            let id1 = ids[index1].trim().to_string();
            if id0 == "" || id1 == "" {
                continue;
            }
            if check(&id0, &id1) {
                return (id0, id1);
            }
        }
    }
    return (String::new(), String::new());
}

fn check(id0: &str, id1: &str) -> bool {
    // Given two id strings, check that they differ by only a single character
    let mut diffs: u32 = 0;
    for i in 0..id0.len() {
        let c0 = &id0[i..i+1];
        let c1 = &id1[i..i+1];
        if c0 != c1 {
            diffs += 1;
        }
    }
    return diffs == 1;
}
