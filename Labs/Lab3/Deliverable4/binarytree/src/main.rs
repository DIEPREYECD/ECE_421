fn main() {
    println!("Hello, world!");

    let mut tree = Tree::new();
    tree.insert_node(5);
    tree.insert_node(3);
    tree.insert_node(7);
    println!("{:#?}", tree);
}

#[derive(Debug)]
enum Tree<T: Ord> {
    Node {
        data: T,
        left_child: Box<Tree<T>>,
        right_child: Box<Tree<T>>,
    },
    Empty,
}

impl<T: Ord> Tree<T> {
    pub fn new() -> Self {
        Tree::Empty
    }

    pub fn insert_node(&mut self, node_data: T) {
        match self {
            Tree::Empty => {
                *self = Tree::Node {
                    data: node_data,
                    left_child: Box::new(Tree::Empty),
                    right_child: Box::new(Tree::Empty),
                };
            }
            Tree::Node { data, left_child, right_child } => {
                if node_data < *data {
                    left_child.insert_node(node_data);
                } else {
                    right_child.insert_node(node_data);
                }
            }
        }
    }
}

/*
What is the purpose of Empty? 
We need Empty to represent the absence of a node in the binary tree.

Which solution (struct-based or enum-based) is better?
The enum-based solution is better because it allows us to represent both the presence and absence of nodes 
directly within the tree structure. 
This makes it easier to manage tree operations like insertion and traversal without needing additional constructs 
like Option types.
*/
