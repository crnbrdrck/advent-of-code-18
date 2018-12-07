use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::prelude::*;

// In order to use the BinaryHeap as a min Heap, I should implement a Task struct with Ord and PartialOrd flipped for the strings
#[derive(Clone, Debug, Eq, PartialEq)]
struct Task {
    name: String,
}

impl Ord for Task {
    fn cmp(&self, other: &Task) -> Ordering {
        // Compare other task name with this task name to have a min heap
        return other.name.cmp(&self.name);
    }
}

// Also need to implement PartialOrd
impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Task) -> Option<Ordering> {
        // Careful not to reverse here also, as that would break our min heap
        return Some(self.cmp(other));
    }
}

fn main() {
    let test = "Step C must be finished before step A can begin.
    Step C must be finished before step F can begin.
    Step A must be finished before step B can begin.
    Step A must be finished before step D can begin.
    Step B must be finished before step E can begin.
    Step D must be finished before step E can begin.
    Step F must be finished before step E can begin.";
    println!("{} should be CABDFE", calc(test));

    // Now do with the actual file
    let mut file = File::open("input.txt").expect("File 'input.txt' could not be opened.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("File 'input.txt' could not be read.");
    // Trim the file to avoid having that error again
    contents = contents.trim().to_string();
    println!("Puzzle #1 Answer: {:?}", calc(&contents));
}

fn calc(input: &str) -> String {
    // Given an input specifying the order jobs must be finished in, return a string of the job order

    // First, build up the task order list and the task name list
    let (mut task_map, mut task_names) = parse_task_list(&input);

    // Now to search through the generated map and find the jobs to do in which order
    return parse_task_map(&mut task_map, &mut task_names);
}

fn parse_task_list(input: &str) -> (HashMap<String, Vec<&str>>, HashSet<&str>) {
    // Given the string task list, parse it and return a mapping of jobs to the jobs that require them, as well as a Set of all the names to use later
    let mut task_map: HashMap<String, Vec<&str>> = HashMap::new();
    let mut task_names : HashSet<&str> = HashSet::new();
    for line in input.split("\n") {
        // Split on spaces and just get the task names
        let pred = &line.trim()[5..6];
        let succ = &line.trim()[36..37];

        task_names.insert(pred);
        task_names.insert(succ);

        // Check if pred is already a key in the hash map
        if !task_map.contains_key(&pred.to_string()) {
            task_map.insert(pred.to_string(), Vec::new());
        }
        // Now add the succ to the vector, and then sort it
        (*task_map.get_mut(pred).unwrap()).push(succ);
        (*task_map.get_mut(pred).unwrap()).sort_unstable();
    }

    return (task_map, task_names);
}

fn parse_task_map(tasks: &mut HashMap<String, Vec<&str>>, task_names: &mut HashSet<&str>) -> String {
    // Given a task map indicating job priorities, return a string of the jobs in order that they should be done

    // Get the initial choices from the map; the job(s) that have no predecessors, in alpha order
    let mut choices: BinaryHeap<Task> = get_initial_choices(tasks.clone(), task_names);
    let mut output: Vec<String> = Vec::new();

    while !choices.is_empty() {
        // Get the top Task's name, add the tasks that rely on it to the heap and add the name to the output
        let task = choices.pop().unwrap().name;
        output.push(task.clone());

        // Check if the hash map contains the task as a key
        if !tasks.contains_key(&task) {
            continue;
        }
        for name in tasks.get(&task).unwrap() {
            // Ensure that the task has no other dependencies (this is so inefficient ahh)
            let mut ready = true;
            for (parent, reliants) in tasks.clone() {
                if parent != task && reliants.contains(&name) {
                    ready = false;
                    break;
                }
            }
            if !ready {
                continue;
            }
            choices.push(Task {name: name.to_string()});
        }
        // Remove the entry for the task from the tasks hash
        tasks.remove(&task);
    }

    return output.join("");
}

fn get_initial_choices(tasks: HashMap<String, Vec<&str>>, names: &mut HashSet<&str>) -> BinaryHeap<Task> {
    // Get the initial choices from the tasks hash map and make a heap of task structs of the tasks that are available to choose from the start
    let mut choices: BinaryHeap<Task> = BinaryHeap::new();

    // Remove from the set any task in a vec from the hashmap
    for (_, invalid_tasks) in tasks {
        for inv in invalid_tasks {
            names.remove(inv);
        }
    }

    // Add to the Binary Heap Task structs from the remaining names
    for task_name in names.iter() {
        choices.push(Task {name: task_name.to_string()});
    }

    return choices;
}
