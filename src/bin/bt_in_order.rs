use std::fmt::Display;

use dsa::binary_tree::{BinaryNode, SharedBinaryNode};

fn walk<T: Display + Copy>(curr: Option<&SharedBinaryNode<T>>, mut path: Vec<T>) -> Vec<T> {
    match curr {
        Some(curr) => {
            // pre
            // recurse
            path = walk(curr.borrow().left.as_ref(), path);
            path.push(curr.borrow().value);
            path = walk(curr.borrow().right.as_ref(), path);
            // post
            path
        }
        None => path,
    }
}

fn in_order_walk<T: Display + Copy>(head: &SharedBinaryNode<T>) -> Vec<T> {
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

    let path = in_order_walk(&SharedBinaryNode::from_binary_node(head));

    println!("{:?}", path);
}

#[cfg(test)]
mod tests {
    use super::*;
    use dsa::binary_tree::get_test_tree_1;

    #[test]
    fn in_order_works() {
        let tree = SharedBinaryNode::from_binary_node(get_test_tree_1());

        assert_eq!(
            in_order_walk(&tree),
            vec![5, 7, 10, 15, 20, 29, 30, 45, 50, 100]
        )
    }
}
