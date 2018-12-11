
// Make the Circle a struct to make life a little easier
#[derive(Debug)]
struct Circle {
    items: Vec<i32>
}

impl Circle {
    fn get_clockwise_index(&self, pos: i32, places: i32) -> usize {
        // Get the position that is `places` spaces clockwise away from pos
        // This method is designed so that it adds last items at the end and not before the first item
        let mut index = pos + places;
        if index > self.items.len() as i32 {
            index %= self.items.len() as i32;
        }
        return index as usize;
    }

    fn get_counter_clockwise_index(&self, pos: i32, places: i32) -> usize {
        // Get the position that is `places` spaces counter-clockwise away from pos
        let mut index = pos - places;
        if index < 0 {
            index += self.items.len() as i32;
        }
        return index as usize;
    }

    fn add_new_marble(&mut self, pos: usize, marble: i32) -> usize {
        // Add a new marble to the circle given the current marble (does not handle the multiple of 23 part) and return the new index
        let index = self.get_clockwise_index(pos as i32, 2);
        self.items.insert(index, marble);
        return index;
    }

    fn remove_scoring_marble(&mut self, pos: usize) -> (i32, usize) {
        // Given a position where a multiple of 23 would have been added, remove the marble 7 items counter clockwise and return the value, as well as the index of the new current marble
        let index = self.get_counter_clockwise_index(pos as i32, 7);
        return (self.items.remove(index), index);
    }
}

fn main() {
    // Play the marble game and return the highest score found
    println!("{:?} should equal 32", calc("9 players; last marble is worth 25 points"));
    println!("{:?} should equal 8317", calc("10 players; last marble is worth 1618 points"));
    println!("{:?} should equal 146373", calc("13 players; last marble is worth 7999 points"));
    println!("{:?} should equal 2764", calc("17 players; last marble is worth 1104 points"));
    println!("{:?} should equal 54718", calc("21 players; last marble is worth 6111 points"));
    println!("{:?} should equal 37305", calc("30 players; last marble is worth 5807 points"));

    // Try the puzzle input
    let puzzle = "424 players; last marble is worth 71144 points";
    println!("Puzzle Answer #1: {:?}", calc(puzzle));
}

fn calc(input: &str) -> i32 {
    // Given a string stating the number of players and marbles, calculate the highest player's score
    let (mut players, num_marbles) = parse_input(&input);

    // Play the game, updating the player vec with scores
    play(&mut players, num_marbles);

    // Return the max of the players vector
    return *players.iter().max().unwrap();
}

fn play(players: &mut Vec<i32>, num_marbles: i32) {
    // Play the game, updating the score vec as necessary

    // Start off with the marbles 0 and 1 in the circle (to avoid weird edge casing)
    let mut circle: Circle = Circle { items: vec![0, 1] };
    let mut current_marble: usize = 1;
    let mut marble_to_add = 2;

    while marble_to_add <= num_marbles {
        // Loop until all the marbles have been used, curr player is (marble_to_add - 1) % players.len()
        let curr_player = ((marble_to_add - 1) % players.len() as i32) as usize;

        // Checking for multiple of 23
        if marble_to_add % 23 == 0 {
            // The current player gets some points
            let (mut score, new_index) = circle.remove_scoring_marble(current_marble);
            current_marble = new_index;
            score += marble_to_add;
            players[curr_player] += score;
        }
        else {
            current_marble = circle.add_new_marble(current_marble, marble_to_add);
        }
        marble_to_add += 1;
    }
}

fn parse_input(input: &str) -> (Vec<i32>, i32) {
    // Parse the puzzle input and return a vector for player scores, and the number of marbles to use in the game
    // split on space; players = 0, marbles = 6
    let split: Vec<&str> = input.split(" ").collect();
    let num_players = split[0].to_string().parse::<i32>().unwrap();
    let num_marbles = split[6].to_string().parse::<i32>().unwrap();

    // Create a vec for the player scores
    let mut players: Vec<i32> = Vec::with_capacity(num_players as usize);
    for _ in 0..num_players {
        players.push(0);
    }

    return (players, num_marbles);
}
