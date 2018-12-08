

// Define a struct for the nodes
#[derive(Clone, Eq, PartialEq)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<i32>,
}

fn main() {
    // Take in a string of space separated numbers and build nodes from them. Each node is two numbers, number of children and number of metadata entries
    // Probably will parse recursively
    let test = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
    println!("{:?} should equal 138", calc(test));
}

fn calc(input: &str) -> i32 {
    // Convert the input into a vector of i32s
    let vec: Vec<i32> = input.split(" ").map(|s| s.trim().parse::<i32>().unwrap()).collect();

    // Given a space separated list of numbers, parse them as nodes and then sum the metadatas
    let (root, _) = parse_nodes(vec, &mut 0);

    // Iterate through the node, summing all the metadata
    let meta = parse_meta(root);

    return meta;
}

fn parse_nodes(input: Vec<i32>, i: &mut i32) -> (Node, i32) {
    // Parse a node at position `i` of `vec` and return the node and the new value for `i` after parsing
    while i < input.len() {
        let node = Node {children: Vec::new(), metadata: Vec::new()};
        // i is the number of children, i + 1 is the number of metadata items
        let num_children = input[i..i+1];
        i += 1;
        let num_meta = input[i..i+1];
        i += 1;

        // i is now at the first child node position
        for _ in 0..num_children {
            let (child, i) = parse_nodes(input, i);
            node.children.push(child);
        }

        // i *should* now be at the first metadata position
        for _ in 0..num_meta {
            node.metadata.push(input[i]);
            i += 1;
        }

        // Increment i once more after parsing the node
        i += 1;
    }

    return (Node {children: Vec::new(), metadata: Vec::new()}, i);
}

fn parse_meta(node: Node) -> i32 {
    // Given a node, return the sum of the metadata for itself and its children
    let mut meta_sum: i32 = 0;

    // First, add on the metadata from this node specifically
    for meta in node.metadata {
        meta_sum += meta;
    }

    // Then recursively call this method on the node's children
    for child in node.children {
        meta_sum += parse_meta(child);
    }

    return meta_sum;
}
