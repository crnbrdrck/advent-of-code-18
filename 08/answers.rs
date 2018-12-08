
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
    let nodes: Vec<i32> = input.split(" ").map(|s| s.trim().parse::<i32>().unwrap()).collect();

    // Given a space separated list of numbers, parse them as nodes and then sum the metadatas
    let root = parse_nodes(&mut nodes.into_iter());

    // Iterate through the node, summing all the metadata
    let meta = parse_meta(root);

    return meta;
}

fn parse_nodes(input: &mut Iterator<Item=i32>) -> Node {
    // Parse a node at position `i` of `vec` and return the node and the new value for `i` after parsing
    // Parsing child nodes will be recursive since child nodes can themselves have children
    // Node: (num_children, num_meta, children, meta)

    let mut node = Node {children: Vec::new(), metadata: Vec::new()};

    // No need to loop as there will be a single root node
    // The first two next calls will get the num children and num metadata from the iterator
    let num_children = input.next().unwrap();
    let num_meta = input.next().unwrap();

    // Loop through the children and parse them
    for _ in 0..num_children {
        node.children.push(parse_nodes(input));
    }

    // Loop through the number of metadata entries and add those to the meta array
    for _ in 0..num_meta {
        node.metadata.push(input.next().unwrap());
    }
    return node;
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
