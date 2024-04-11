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
    tail: Option<SharedNode<T>>,
}

impl<T> DoublyLinkedList<T>
where
    T: Copy + Display,
{
    fn new() -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
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

        if self.length == 0 {
            self.head = Some(Rc::clone(&shared_new_node));
            self.tail = Some(shared_new_node);
            self.length += 1;

            return;
        }

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
        } else if idx == 0 {
            self.prepend(item);
            return true;
        } else if idx == self.length {
            self.append(item);
            return true;
        }

        self.length += 1;

        let old_head = &self.head;

        let new_node = Node {
            value: item,
            prev: None,
            next: None,
        };

        let shared_new_node = Rc::new(RefCell::new(new_node));

        if let Some(old_head) = old_head {
            let mut current_pointer = Rc::clone(old_head);

            for _ in 0..idx {
                let current = Rc::clone(&current_pointer);
                let borrowed_current = current.borrow();

                let next = borrowed_current.next.as_ref();

                if let Some(next) = next {
                    current_pointer = Rc::clone(next);
                }
            }

            let target_node = Rc::clone(&current_pointer);

            let borrowed_target_node = target_node.borrow();

            let prev = borrowed_target_node.next.as_ref();

            if let Some(prev) = prev {
                let prev = Rc::clone(prev);

                prev.borrow_mut().next = Some(Rc::clone(&shared_new_node));
                shared_new_node.borrow_mut().next = Some(prev);
            }

            let next = Rc::clone(&target_node);
            next.borrow_mut().prev = Some(Rc::clone(&shared_new_node));

            shared_new_node.borrow_mut().next = Some(next);

            true
        } else {
            false
        }
    }

    fn append(&mut self, item: T) {
        let new_node = Node {
            value: item,
            prev: None,
            next: None,
        };

        let shared_new_node = Rc::new(RefCell::new(new_node));

        if self.length == 0 {
            self.head = Some(Rc::clone(&shared_new_node));
            self.tail = Some(shared_new_node);
            self.length += 1;

            return;
        }

        if let Some(tail) = &self.tail {
            let tail = Rc::clone(&tail);

            shared_new_node.borrow_mut().prev = Some(Rc::clone(&tail));

            tail.borrow_mut().next = Some(Rc::clone(&shared_new_node));
        }

        self.tail = Some(shared_new_node);
        self.length += 1;
    }
}

fn main() {}
