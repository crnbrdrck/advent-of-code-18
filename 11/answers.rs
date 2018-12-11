fn main() {
    // Given a grid serial number, find the (x, y) position of the top left corner of the 3x3 square with the largest power sum
    println!("{:?} should equal 33,45", calc(18));
    println!("{:?} should equal 21,61", calc(42));

    let puzzle_input = 9306;
    println!("Puzzle Answer #1: {:?}", calc(puzzle_input));
}

fn calc(input: i32) -> String {
    // Find the X,Y coordinate of the top left corner of the 3x3 square with the largest sum of power

    // Make a 300x300 matrix
    let mut matrix: Vec<Vec<i32>> = generate_matrix();

    // Go through the cells and calculate their power levels
    calculate_power(input, &mut matrix);

    // Now search the grid for the square with the largest sum and return it
    let (x, y) = find_largest(&matrix);

    return format!("{},{}", x, y);
}

fn find_largest(matrix: &Vec<Vec<i32>>) -> (u32, u32) {
    // Find the top left coord of the 3x3 square with the biggest sum
    let mut max_sum = std::i32::MIN;
    let mut x = 0;
    let mut y = 0;
    // Iterate only up to 297 to avoid index out of bounds issues
    for i in 0..297 {
        for j in 0..297 {
            let sum = calculate_sum(matrix, i, j);
            if sum > max_sum {
                max_sum = sum;
                x = i;
                y = j;
            }
        }
    }
    return (x + 1, y + 1);
}

fn calculate_sum(matrix: &Vec<Vec<i32>>, x: u32, y: u32) -> i32 {
    // Given a matrix and the coords of the top left cell of a 3x3 grid, return the sum of that 3x3 grid
    let mut values: Vec<i32> = Vec::new();
    for i in x..x+3 {
        for j in y..y+3 {
            values.push(matrix[i as usize][j as usize]);
        }
    }
    return values.into_iter().sum();
}

fn calculate_power(input: i32, matrix: &mut Vec<Vec<i32>>) {
    // Given the grid serial number as input, calculate the power of all the cells in the supplied matrix

    // Power is determined as follows;
    // 1. rackID = X + 10 (x and y are indexed from 1, not 0)
    // 2. initial power level = rackID * Y
    // 3. Increase power level by `input`
    // 4. Multiply power level by rackID
    // 5. Take the hundreds digit from this number
    // 6. Subtract 5 to get the power level

    for x in 0..300 {
        for y in 0..300 {
            // Don't forget to add 1 to x and y when using them for calculations
            let rack_id = x + 11;
            let mut power = rack_id * (y + 1);
            power += input;
            power *= rack_id;
            power = get_hundreds(power);
            power -= 5;
            matrix[x as usize].push(power);
        }
    }
}

fn get_hundreds(num: i32) -> i32 {
    // Given a number, return its hundreds digit or 0 if it has none
    let string = num.to_string();
    if string.len() < 3 {
        return 0;
    }
    for i in 0..string.len() {
        if string[i..].len() == 3 {
            return string[i..i+1].chars().next().unwrap().to_string().parse::<i32>().unwrap();
        }
    }
    // If we got to this point, return 0
    return 0;
}

fn generate_matrix() -> Vec<Vec<i32>> {
    // Generate a 300x300 matrix
    let mut matrix: Vec<Vec<i32>> = Vec::with_capacity(300);
    for _ in 0..300 {
        matrix.push(Vec::with_capacity(300));
    }
    return matrix;
}
