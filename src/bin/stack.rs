use std::fmt::Display;
use std::{cell::RefCell, rc::Rc};

struct Node<T> {
    value: T,
    prev: Option<SharedNode<T>>,
}

type SharedNode<T> = Rc<RefCell<Node<T>>>;

struct Stack<T> {
    head: Option<SharedNode<T>>,
    length: usize,
}

impl<T> Stack<T>
where
    T: Copy + Display,
{
    fn new() -> Self {
        Stack {
            head: None,
            length: 0,
        }
    }

    fn push(&mut self, item: T) {
        let mut new_node = Node {
            value: item,
            prev: None,
        };

        if let Some(old_head) = &self.head {
            let old_head = Rc::clone(&old_head);

            new_node = Node {
                prev: Some(old_head),
                ..new_node
            };
        }

        let new_node = Rc::new(RefCell::new(new_node));
        self.head = Some(new_node);

        self.length += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.length > 0 {
            if let Some(head) = &self.head {
                let pop_value = head.borrow().value;

                let current = Rc::clone(head);

                if let Some(prev) = &current.borrow().prev {
                    self.head = Some(Rc::clone(prev));
                } else {
                    self.head = None;
                }

                self.length -= 1;

                return Some(pop_value);
            }
        }
        None
    }

    fn peek(&self) -> Option<T> {
        if let Some(head) = &self.head {
            Some(head.borrow().value)
        } else {
            None
        }
    }
}

fn main() {
    let mut list = Stack::new();

    list.push(5);
    list.push(7);
    list.push(9);

    println!("head of list: {:?}", list.peek());

    list.pop();

    let head = list.head.as_ref();
    let head = Rc::clone(head.expect("Failed to access head"));
    let mut current = Some(head);

    while current.is_some() {
        let temp = Rc::clone(current.as_ref().expect("Failed to access some node"));

        println!("node value: {}", temp.borrow().value);

        let borrowed_temp = temp.borrow();
        let prev = borrowed_temp.prev.as_ref();

        if let Some(prev) = prev {
            let prev = Rc::clone(prev);
            current = Some(prev);
        } else {
            current = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_works_normally() {
        let mut list = Stack::new();

        list.push(5);
        list.push(7);
        list.push(9);

        assert_eq!(list.pop(), Some(9));
        assert_eq!(list.length, 2);

        list.push(11);
        assert_eq!(list.pop(), Some(11));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.peek(), Some(5));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        list.push(69);
        assert_eq!(list.peek(), Some(69));
        assert_eq!(list.length, 1);
    }
}
