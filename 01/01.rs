use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Calculate frequency based on the inputs, but first try it with a test
    println!("Answer 1 tests");
    println!("{} should be 3", calc("+1\n+1\n+1"));
    println!("{} should be 0", calc(&"+1\n+1\n-2"));
    println!("{} should be -6", calc(&"-1\n-2\n-3"));

    // Lastly, read from the input file and run the calc function on that
    let mut file = File::open("input.txt").expect("File 'input.txt' could not be opened.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("File 'input.txt' could not be read.");
    println!("Puzzle Answer #1: {}", calc(&contents));
}

fn calc(input: &str) -> i32 {
    // Given a string that is comma separated values, calculate the resulting frequency
    let mut freq = 0;
    let diffs = input.split("\n"); // Creates an iterator which is fine by me
    for diff_str in diffs {
        // Trim the string and cast it to an integer
        if diff_str == "" {
            continue;
        }
        let diff = diff_str.trim().to_string().parse::<i32>().unwrap();
        freq += diff
    }
    return freq
}
