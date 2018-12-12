use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Calculate the sum of the indices of the pots that have plants in them at the end of 20 generations
    let test = "initial state: #..#.#..##......###...###

    ...## => #
    ..#.. => #
    .#... => #
    .#.#. => #
    .#.## => #
    .##.. => #
    .#### => #
    #.#.# => #
    #.### => #
    ##.#. => #
    ##.## => #
    ###.. => #
    ###.# => #
    ####. => #";
    println!("{:?} should equal 325", calc(test));

    // Now do with the actual file
    let mut file = File::open("input.txt").expect("File 'input.txt' could not be opened.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("File 'input.txt' could not be read.");
    // Trim the file to avoid having that error again
    contents = contents.trim().to_string();
    println!("Puzzle #1 Answer: {:?}", calc(&contents));
}

fn calc(input: &str) -> i32 {
    // Given an initial state for pots and the patterns that lead to the creation of new plants, get the sum of the pot indices that have plants after 20 generations
    return 0;
}

fn parse_input(input: &str) -> (Vec<String>, HashMap<String, String>) {
    // Parse the input into a vector of plant pots, and a vector of patterns that lead to new plants being grown

    // Initial state is contained in the first line
    let split: Vec<&str> = input.split("\n").map(|s| s.trim()).collect();

    let state: Vec<String> = split[0][15..].split("").map(|s| s.to_string()).filter(|s| s.to_string() != String::from("")).collect();

    let mut patterns: HashMap<String, String> = HashMap::new();

    // Patterns are stored in line 2 onwards
    for i in 2..split.len() {
        let line: Vec<String> = split[i].split(" ").map(|s| s.to_string()).collect();
        patterns.insert(line[0].clone(), line[2].clone());
    }
    return (state, patterns);
}
