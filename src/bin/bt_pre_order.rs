use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

struct BinaryNode<T> {
    value: T,
    left: Option<SharedBinaryNode<T>>,
    right: Option<SharedBinaryNode<T>>,
}

type SharedBinaryNode<T> = Rc<RefCell<BinaryNode<T>>>;

fn walk<T: Display + Copy>(curr: Option<&SharedBinaryNode<T>>, mut path: Vec<T>) -> Vec<T> {
    match curr {
        Some(curr) => {
            // pre
            path.push(curr.borrow().value);
            // recurse
            path = walk(curr.borrow().left.as_ref(), path);
            path = walk(curr.borrow().right.as_ref(), path);
            // post
            path
        }
        None => path,
    }
}

fn pre_order_walk<T: Display + Copy>(head: &SharedBinaryNode<T>) -> Vec<T> {
    walk(Some(head), Vec::new())
}

fn main() {}
