use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Clone)]
pub struct BinaryNode<T> {
    pub value: T,
    pub left: Option<SharedBinaryNode<T>>,
    pub right: Option<SharedBinaryNode<T>>,
}

pub type SharedBinaryNode<T> = Rc<RefCell<BinaryNode<T>>>;

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

        self.left = Some(Rc::new(RefCell::new(left)));
    }

    pub fn add_right_node(&mut self, value: T) {
        let right = Self::new(value);

        self.right = Some(Rc::new(RefCell::new(right)));
    }
}
