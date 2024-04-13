use std::cell::RefCell;
use std::fmt::Write as _;
use std::fmt::{Debug, Display};
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
    T: Copy + Display + PartialEq,
{
    pub fn new() -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
        }
    }

    pub fn prepend(&mut self, item: T) {
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

    pub fn insert_at(&mut self, item: T, idx: usize) -> bool {
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

        let new_node = Node {
            value: item,
            prev: None,
            next: None,
        };

        let shared_new_node = Rc::new(RefCell::new(new_node));

        let target_node = self
            .get_at(idx)
            .expect("Failed to get node at insert position");

        let prev = match target_node.borrow().prev.as_ref() {
            Some(prev) => Some(Rc::clone(prev)),
            None => None,
        };

        if let Some(prev) = prev {
            prev.borrow_mut().next = Some(Rc::clone(&shared_new_node));
            shared_new_node.borrow_mut().prev = Some(prev);
        }

        let next = Rc::clone(&target_node);
        next.borrow_mut().prev = Some(Rc::clone(&shared_new_node));

        shared_new_node.borrow_mut().next = Some(next);

        true
    }

    pub fn append(&mut self, item: T) {
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

    pub fn remove(&mut self, item: T) -> Option<T> {
        if self.length == 0 {
            return None;
        }

        let mut current_pointer = match &self.head {
            Some(head) => Some(Rc::clone(head)),
            None => None,
        };

        for _ in 0..self.length {
            if let Some(some_current_pointer) = current_pointer.as_ref() {
                let current = Rc::clone(some_current_pointer);
                if current.borrow().value == item {
                    break;
                }

                let borrowed_current = current.borrow();
                current_pointer = match &borrowed_current.next {
                    Some(next) => Some(Rc::clone(next)),
                    _ => None,
                };
            }
        }

        if current_pointer.is_none() {
            return None;
        }

        match current_pointer {
            Some(node) => Some(self.remove_node(node)),
            None => None,
        }
    }

    pub fn get(&self, idx: usize) -> Option<T> {
        match self.get_at(idx) {
            Some(node) => Some(node.borrow().value),
            None => None,
        }
    }

    pub fn remove_at(&mut self, idx: usize) -> Option<T> {
        let node = self.get_at(idx);

        if let Some(node) = node {
            return Some(self.remove_node(node));
        } else {
            return None;
        }
    }

    fn get_at(&self, idx: usize) -> Option<SharedNode<T>> {
        if idx >= self.length || self.length == 0 {
            return None;
        }

        let head = Rc::clone(self.head.as_ref().expect("Failed to access head"));

        let mut current_pointer = Rc::clone(&head);

        for _ in 0..idx {
            let current = Rc::clone(&current_pointer);
            let borrowed_current = current.borrow();

            let next = borrowed_current.next.as_ref();

            let next = next.expect("Failed to access next node");

            current_pointer = Rc::clone(next);
        }

        Some(current_pointer)
    }

    fn remove_node(&mut self, node: SharedNode<T>) -> T {
        self.length -= 1;

        if self.length == 0 {
            self.head = None;
            self.tail = None;

            return node.borrow().value;
        }

        let borrowed_node = node.borrow();
        let prev = &borrowed_node.prev;
        let next = &borrowed_node.next;

        if let Some(prev) = prev {
            match next {
                Some(next) => prev.borrow_mut().next = Some(Rc::clone(next)),
                // target is tail
                None => {
                    prev.borrow_mut().next = None;

                    self.tail = Some(Rc::clone(&prev));
                }
            }
        };

        if let Some(next) = next {
            match prev {
                Some(prev) => next.borrow_mut().prev = Some(Rc::clone(prev)),
                // target is head
                None => {
                    next.borrow_mut().prev = None;

                    self.head = Some(Rc::clone(&next));
                }
            }
        };

        borrowed_node.value
    }
}

impl<T> Debug for DoublyLinkedList<T>
where
    T: Copy + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_owned();

        let head = &self.head;

        if let Some(head) = head {
            let mut current_pointer = Some(Rc::clone(head));

            for _ in 0..self.length {
                if let Some(current) = &current_pointer {
                    let current = Rc::clone(&current);

                    let borrowed_current = current.borrow();
                    let value = borrowed_current.value;

                    write!(&mut output, "{} => ", value)?;

                    let next = borrowed_current.next.as_ref();

                    if let Some(next) = next {
                        current_pointer = Some(Rc::clone(next));
                    } else {
                        write!(&mut output, "None")?;
                        current_pointer = None;
                    }
                }
            }
        } else {
            write!(&mut output, "Empty list")?;
        }

        write!(f, "{}", output)
    }
}

fn main() {
    let mut list = DoublyLinkedList::new();

    list.append(5);
    list.append(7);
    list.append(9);

    println!("{:?}", list);

    list.remove_at(1);

    println!("{:?}", list);

    list.append(11);
    list.remove_at(1);
    list.remove(9);
    list.remove_at(0);
    list.remove_at(0);

    println!("{:?}", list);

    list.prepend(5);
    list.prepend(7);
    list.prepend(9);

    println!("{:?}", list);

    list.remove(9);

    println!("{:?}", list);
    list.insert_at(8, 1);
    println!("{:?}", list);

    println!(
        "node at position 2: {}",
        list.get(2).expect("Failed to access node at 2")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linked_list_works() {
        let mut list = DoublyLinkedList::new();

        list.append(5);
        list.append(7);
        list.append(9);

        assert_eq!(list.get(2), Some(9));
        assert_eq!(list.remove_at(1), Some(7));
        assert_eq!(list.length, 2);

        list.append(11);
        assert_eq!(list.remove_at(1), Some(9));
        assert_eq!(list.remove(9), None);
        assert_eq!(list.remove_at(0), Some(5));
        assert_eq!(list.remove_at(0), Some(11));
        assert_eq!(list.length, 0);

        list.prepend(5);
        list.prepend(7);
        list.prepend(9);

        assert_eq!(list.get(2), Some(5));
        assert_eq!(list.get(0), Some(9));
        assert_eq!(list.remove(9), Some(9));
        assert_eq!(list.length, 2);
        assert_eq!(list.get(0), Some(7));

        list.insert_at(8, 1);
        assert_eq!(list.get(1), Some(8));
    }
}
