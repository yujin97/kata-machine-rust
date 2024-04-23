use dsa::binary_tree::{BinaryNode, SharedBinaryNode};
use dsa::queue::Queue;
use std::rc::Rc;

fn bfs(head: &SharedBinaryNode<i64>, needle: i64) -> bool {
    let mut q = Queue::new();

    q.enqueue(Rc::clone(head));

    while q.length > 0 {
        let curr = q.deque().expect("Failed to deque Binary Node");

        // search
        if curr.borrow().value == needle {
            return true;
        }

        let borrowed_curr = curr.borrow();

        if let Some(left_node) = borrowed_curr.left.as_ref() {
            q.enqueue(Rc::clone(left_node));
        }

        if let Some(right_node) = borrowed_curr.right.as_ref() {
            q.enqueue(Rc::clone(right_node));
        }
    }

    false
}

fn main() {
    let head = SharedBinaryNode::from_binary_node(BinaryNode::new(69));

    head.borrow_mut().add_left_node(420);
    head.borrow_mut().add_right_node(1234);

    head.borrow()
        .left
        .as_ref()
        .expect("Failed to access left")
        .borrow_mut()
        .add_left_node(4321);

    println!("there is 420 in the tree: {:?}", bfs(&head, 420));
    println!("there is 567 in the tree: {:?}", bfs(&head, 567));
}

#[cfg(test)]
mod tests {
    use super::*;
    use dsa::binary_tree::get_test_tree_1;

    #[test]
    fn bt_bfs_works() {
        let tree1 = SharedBinaryNode::from_binary_node(get_test_tree_1());

        assert_eq!(bfs(&tree1, 45), true);
        assert_eq!(bfs(&tree1, 7), true);
        assert_eq!(bfs(&tree1, 69), false);
    }
}

