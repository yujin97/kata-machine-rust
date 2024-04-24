use dsa::binary_tree::SharedBinaryNode;

#[allow(dead_code)]
fn search(curr: Option<&SharedBinaryNode<i64>>, needle: i64) -> bool {
    if curr.is_none() {
        return false;
    }

    let curr = curr.expect("Failed to access curr");

    let value = curr.borrow().value;

    if value == needle {
        return true;
    }

    if value < needle {
        return search(curr.borrow().right.as_ref(), needle);
    } else {
        return search(curr.borrow().left.as_ref(), needle);
    }
}

#[allow(dead_code)]
fn dfs(head: &SharedBinaryNode<i64>, needle: i64) -> bool {
    search(Some(head), needle)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use dsa::binary_tree::get_test_tree_1;

    #[test]
    fn it_works_on_bst() {
        let tree = SharedBinaryNode::from_binary_node(get_test_tree_1());

        assert_eq!(dfs(&tree, 45), true);
        assert_eq!(dfs(&tree, 7), true);
        assert_eq!(dfs(&tree, 69), false);
    }
}
