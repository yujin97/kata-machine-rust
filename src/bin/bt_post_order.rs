use std::fmt::Display;

use dsa::binary_tree::{BinaryNode, SharedBinaryNode};

fn walk<T: Display + Copy>(curr: Option<&SharedBinaryNode<T>>, mut path: Vec<T>) -> Vec<T> {
    match curr {
        Some(curr) => {
            // pre
            // recurse
            path = walk(curr.borrow().left.as_ref(), path);
            path = walk(curr.borrow().right.as_ref(), path);
            // post
            path.push(curr.borrow().value);
            path
        }
        None => path,
    }
}

fn post_order_walk<T: Display + Copy>(head: &SharedBinaryNode<T>) -> Vec<T> {
    walk(Some(head), Vec::new())
}

fn main() {
    let mut head = BinaryNode::new(69);

    head.add_left_node(420);
    head.add_right_node(1234);

    head.left
        .as_ref()
        .expect("Failed to access left")
        .borrow_mut()
        .add_left_node(4321);

    let path = post_order_walk(&SharedBinaryNode::from_binary_node(head));

    println!("{:?}", path);
}

#[cfg(test)]
mod tests {
    use super::*;
    use dsa::binary_tree::get_test_tree_1;

    #[test]
    fn post_order_works() {
        let tree = SharedBinaryNode::from_binary_node(get_test_tree_1());

        assert_eq!(
            post_order_walk(&tree),
            vec![7, 5, 15, 10, 29, 45, 30, 100, 50, 20]
        )
    }
}
