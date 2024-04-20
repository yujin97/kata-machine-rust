use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

struct BinaryNode<T> {
    value: T,
    left: Option<SharedBinaryNode<T>>,
    right: Option<SharedBinaryNode<T>>,
}

type SharedBinaryNode<T> = Rc<RefCell<BinaryNode<T>>>;

impl<T> BinaryNode<T>
where
    T: Copy + Display,
{
    fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    fn add_left_node(&mut self, value: T) {
        let left = Self::new(value);

        self.left = Some(Rc::new(RefCell::new(left)));
    }

    fn add_right_node(&mut self, value: T) {
        let right = Self::new(value);

        self.right = Some(Rc::new(RefCell::new(right)));
    }
}

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

    let path = in_order_walk(&Rc::new(RefCell::new(head)));

    println!("{:?}", path);
}