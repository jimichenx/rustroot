//! One node, two parents. Each parent wants to own the child.
//! Uncomment `right` to get error[E0382]: use of moved value: `child`.

struct Node {
    name: String,
    children: Vec<Node>,
}

fn main() {
    let child = Node { name: String::from("shared"), children: Vec::new() };
    let left = Node { name: String::from("left"), children: vec![child] };
    // let right = Node { name: String::from("right"), children: vec![child] };
    println!("{} has {}", left.name, left.children[0].name);
}
