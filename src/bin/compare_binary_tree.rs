use dsa::binary_tree::{get_test_tree_1, get_test_tree_2, SharedBinaryNode};

fn compare(a: Option<&SharedBinaryNode<i64>>, b: Option<&SharedBinaryNode<i64>>) -> bool {
    if a.is_none() && b.is_none() {
        return true;
    }

    if a.is_none() || b.is_none() {
        return false;
    }

    let a = a.expect("Failed to access a");
    let b = b.expect("Failed to access b");

    if a.borrow().value != b.borrow().value {
        return false;
    }

    compare(a.borrow().left.as_ref(), b.borrow().left.as_ref())
        && compare(a.borrow().right.as_ref(), b.borrow().right.as_ref())
}

fn main() {
    let a = SharedBinaryNode::from_binary_node(get_test_tree_1());
    let b = SharedBinaryNode::from_binary_node(get_test_tree_1());
    let c = SharedBinaryNode::from_binary_node(get_test_tree_2());

    println!("a and b are identical: {}", compare(Some(&a), Some(&b)));
    println!("a and c are identical: {}", compare(Some(&a), Some(&c)));
}
