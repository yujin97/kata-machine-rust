use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

struct Node<T> {
    value: T,
    prev: Option<SharedNode<T>>,
    next: Option<SharedNode<T>>,
}

type SharedNode<T> = Rc<RefCell<Node<T>>>;

struct DoublyLinkedList<T> {
    length: usize,
    head: Option<SharedNode<T>>,
}

impl<T> DoublyLinkedList<T>
where
    T: Copy + Display,
{
    fn new() -> Self {
        Self {
            length: 0,
            head: None,
        }
    }

    fn prepend(&mut self, item: T) {
        let old_head = &self.head;

        let new_node = Node {
            value: item,
            prev: None,
            next: None,
        };

        let shared_new_node = Rc::new(RefCell::new(new_node));

        if let Some(old_head) = old_head {
            let next = Rc::clone(old_head);

            shared_new_node.borrow_mut().next = Some(next);

            old_head.borrow_mut().prev = Some(Rc::clone(&shared_new_node));
        }

        self.head = Some(shared_new_node);
        self.length += 1;
    }

    fn insert_at(&mut self, item: T, idx: usize) -> bool {
        if idx > self.length {
            return false;
        }

        if idx == 0 {
            self.prepend(item);
            return true;
        }

        let old_head = &self.head;

        let new_node = Node {
            value: item,
            prev: None,
            next: None,
        };

        let shared_new_node = Rc::new(RefCell::new(new_node));

        // we always have a head at this point
        // computation is delegated to prepend
        // when idx == 0
        // length >= idx > 0
        // self.length is at least 1
        if let Some(old_head) = old_head {
            let mut current_pointer = Rc::clone(old_head);

            for _ in 0..({
                match idx {
                    0 => 0,
                    _ => idx - 1,
                }
            }) {
                let current = Rc::clone(&current_pointer);
                let borrowed_current = current.borrow();

                let next = borrowed_current.next.as_ref();

                if let Some(next) = next {
                    current_pointer = Rc::clone(next);
                }
            }

            let node_before_idx = Rc::clone(&current_pointer);

            let borrowed_node_before_idx = node_before_idx.borrow();

            let next = borrowed_node_before_idx.next.as_ref();

            if let Some(next) = next {
                let next = Rc::clone(next);

                shared_new_node.borrow_mut().next = Some(next);
            }

            let prev = Rc::clone(&node_before_idx);
            shared_new_node.borrow_mut().prev = Some(prev);

            let mut borrowed_node_before_idx = node_before_idx.borrow_mut();
            borrowed_node_before_idx.next = Some(Rc::clone(&shared_new_node));

            let borrowed_new_node = shared_new_node.borrow();
            let next = borrowed_new_node.next.as_ref();

            if let Some(next) = next {
                next.borrow_mut().prev = Some(Rc::clone(&shared_new_node));
            }
        }

        true
    }
}

fn main() {}
