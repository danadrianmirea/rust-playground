use std::collections::VecDeque;

#[derive(Clone)]
struct TreeNode {
    value: i32,
    children: Vec<Box<TreeNode>>,
}

fn main() {
    let root = generate_example_tree();
    
    println!("Breadth First Traversal:");
    breadth_first_traversal(root.clone());
    
    println!("Depth First Traversal:");
    depth_first_traversal(root);
}

// Breadth First Traversal (level-order)
fn breadth_first_traversal(root: Option<Box<TreeNode>>) {
    if let Some(node) = root {
        let mut queue = VecDeque::new();
        queue.push_back(node);

        while let Some(node) = queue.pop_front() {
            println!("{}", node.value);

            // Push all children into the queue
            for child in node.children.iter() {
                queue.push_back(child.clone());
            }
        }
    }
}

// Depth First Traversal (preorder)
fn depth_first_traversal(root: Option<Box<TreeNode>>) {
    if let Some(node) = root {
        println!("{}", node.value);

        for child in node.children.iter() {
            depth_first_traversal(Some(child.clone()));
        }
    }
}

// Generate an example generic tree
//          1
//       /  |  \
//      2   3   4
//     / \
//    5   6
fn generate_example_tree() -> Option<Box<TreeNode>> {
    let node5 = Box::new(TreeNode { value: 5, children: vec![] });
    let node6 = Box::new(TreeNode { value: 6, children: vec![] });
    let node2 = Box::new(TreeNode { value: 2, children: vec![node5, node6] });
    let node3 = Box::new(TreeNode { value: 3, children: vec![] });
    let node4 = Box::new(TreeNode { value: 4, children: vec![] });

    let root = Box::new(TreeNode { value: 1, children: vec![node2, node3, node4] });
    Some(root)
}