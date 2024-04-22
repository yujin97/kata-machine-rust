use std::cell::RefCell;
use std::fmt::Display;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone)]
pub struct BinaryNode<T> {
    pub value: T,
    pub left: Option<SharedBinaryNode<T>>,
    pub right: Option<SharedBinaryNode<T>>,
}

#[derive(Clone)]
pub struct SharedBinaryNode<T>(Rc<RefCell<BinaryNode<T>>>);

impl<T> Deref for SharedBinaryNode<T> {
    type Target = Rc<RefCell<BinaryNode<T>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> SharedBinaryNode<T>
where
    T: Clone,
{
    pub fn from_binary_node(node: BinaryNode<T>) -> Self {
        SharedBinaryNode(Rc::new(RefCell::new(node)))
    }
}

impl<T> BinaryNode<T>
where
    T: Copy + Display,
{
    pub fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    pub fn add_left_node(&mut self, value: T) {
        let left = Self::new(value);

        self.left = Some(SharedBinaryNode::from_binary_node(left));
    }

    pub fn add_right_node(&mut self, value: T) {
        let right = Self::new(value);

        self.right = Some(SharedBinaryNode::from_binary_node(right));
    }
}
