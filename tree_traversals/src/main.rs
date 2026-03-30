use std::collections::VecDeque;

#[derive(Clone)]
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn main() {
    let root = generate_example_tree();
    println!("Breadth First Traversal:");
    breadth_first_traversal(root.clone());
    println!("Depth First Traversal:");
    depth_first_traversal(root);
}

// Breadth First Traversal
fn breadth_first_traversal(root: Option<Box<TreeNode>>) {
    if let Some(node) = root {
        let mut queue = VecDeque::new();
        queue.push_back(node);
        while let Some(node) = queue.pop_front() {
            println!("{}", node.value);
            if let Some(left) = node.left {
                queue.push_back(left);
            }
            if let Some(right) = node.right {
                queue.push_back(right);
            }
        }
    }
}

// Depth First Traversal (preorder)
fn depth_first_traversal(root: Option<Box<TreeNode>>) {
    if let Some(node) = root {
        println!("{}", node.value);
        depth_first_traversal(node.left);
        depth_first_traversal(node.right);
    }
}

// Generate an example tree:
//        1
//       / \
//      2   3
//     / \
//    4   5
fn generate_example_tree() -> Option<Box<TreeNode>> {
    let node4 = Some(Box::new(TreeNode { value: 4, left: None, right: None }));
    let node5 = Some(Box::new(TreeNode { value: 5, left: None, right: None }));
    let node2 = Some(Box::new(TreeNode { value: 2, left: node4, right: node5 }));
    let node3 = Some(Box::new(TreeNode { value: 3, left: None, right: None }));
    let root = Some(Box::new(TreeNode { value: 1, left: node2, right: node3 }));
    root
}