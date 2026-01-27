fn main() {
    println!("Hello, world!");

    let mut root = TreeNode {
        data: "g",
        left_child: None,
        right_child: None,
    };

    println!("{:#?}", root);

    // Test inserting nodes
    root.insert_node("b");
    root.insert_node("z");
    root.insert_node("a");
    println!("{:#?}", root);
}

#[derive(Debug)]
struct TreeNode<'a> {
    data: &'a str,
    left_child: Option<Box<TreeNode<'a>>>,
    right_child: Option<Box<TreeNode<'a>>>,
}

impl<'a> TreeNode<'a> {
    pub fn insert_node(&mut self, data: &'a str) {
        if self.data == data {
            return;
        }
        let new_node = if data < self.data { &mut self.left_child } else { &mut self.right_child };
        match new_node {
            Some(child) => {
                child.insert_node(data);
            }
            None => {
                let new_tree_node = TreeNode {
                    data,
                    left_child: None,
                    right_child: None,
                };
                if data < self.data {
                    self.left_child = Some(Box::new(new_tree_node));
                } else {
                    self.right_child = Some(Box::new(new_tree_node));
                }
            }
        }
    }
}

// Question 2: Try to run the above code. Does it run? If not, explain why and rewrite the code so it runs.
/*
The code doesn't compile because the struct TreeNode contains references (&str) without a defined lifetime. 
In Rust, references must have a specified lifetime to ensure they are valid for the duration of their use.

*/
