use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Calculate the number of squares that are used in more than one of the supplied claims
    let test = "#1 @ 1,3: 4x4
    #2 @ 3,1: 4x4
    #3 @ 5,5: 2x2";
    let (count, id) = calc(&test);
    println!("{}, {} should be 4, 3", count, id);

    // Lastly, read from the input file and run the calc function on that
    let mut file = File::open("input.txt").expect("File 'input.txt' could not be opened.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("File 'input.txt' could not be read.");
    let (count, id) = calc(&contents);
    println!("Puzzle Answers: {}, {}", count, id);
}

fn calc(input: &str) -> (i32, i32) {
    // Given a list of claims, calculate the number of squares that are used in multiple claims

    // x co-ord => y co-ord => number of claims its used in
    let mut squares: HashMap<i32, HashMap<i32, (i32, i32)>> = HashMap::new();
    let mut ids: HashSet<i32> = HashSet::new();

    // Iterate through the claims
    let claims = input.split("\n");
    for mut claim in claims {
        if claim == "" {
            continue;
        }
        // Strip the string and parse it
        let (id, x, y, width, height) = parse_claim(&claim.trim().to_string().replace(":", ""));
        ids.insert(id);
        // Given an x and y coordinate and a width and height, all the cells that should be filled
        fill_cells(id, x, y, width, height, &mut squares, &mut ids);
    }

    // Loop through all the hashmaps and find the amount of innermost values that are higher than 1
    let mut count = 0;
    for (_, internal) in squares {
        // Iterate through the internal and check the values
        for (_, claims) in internal {
            let (claim_count, _) = claims;
            if claim_count > 1 {
                count += 1;
            }
        }
    }
    // Get the single (hopefully) remaining id
    for id in ids {
        return (count, id);
    }
    return (0, 0);
}

fn parse_claim(claim: &str) -> (i32, i32, i32, i32, i32) {
    // Given a claim in string form, parse and return the x, y coords and the width and height
    // Format; #<id> @ x,y: wxh
    let vec: Vec<&str> = claim.split(" ").collect();
    // 0 => #<id>, 1 => @, 2 => x,y, 3 => wxh
    let id = vec[0].replace("#", "").parse::<i32>().expect("Could not parse id");
    let coords: Vec<&str> = vec[2].split(",").collect();
    let x = coords[0].parse::<i32>().expect("Could not parse x value to int");
    let y = coords[1].parse::<i32>().expect("Could not parse y value to int");
    let dimensions: Vec<&str> = vec[3].split("x").collect();
    let w = dimensions[0].parse::<i32>().expect("Could not parse w value to int");
    let h = dimensions[1].parse::<i32>().expect("Could not parse h value to int");
    return (id, x, y, w, h);
}

fn fill_cells(id: i32, x: i32, y: i32, width: i32, height: i32, cells: &mut HashMap<i32, HashMap<i32, (i32, i32)>>, ids: &mut HashSet<i32>) {
    // Given the x and y and a width and height, fill in the cells of the supplied hash map for all the cells taken up
    for i in x..(x + width) {
        for j in y..(y + height) {
            // Cell i, j is taken up by the claim
            if !cells.contains_key(&i) {
                cells.insert(i, HashMap::new());
            }
            if !cells[&i].contains_key(&j) {
                cells.get_mut(&i).unwrap().insert(j, (1, id));
            }
            else {
                let (count, old_id) = cells.get_mut(&i).unwrap().get_mut(&j).unwrap();
                *count += 1;
                // Remove the old id from the set
                ids.remove(&*old_id);
                // Also remove the id we're currently checking
                ids.remove(&id);
                *old_id = id;
            }
        }
    }
}
