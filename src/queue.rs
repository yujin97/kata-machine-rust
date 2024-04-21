use std::cell::RefCell;
use std::rc::Rc;

pub struct Node<T> {
    pub value: T,
    pub next: Option<SharedNode<T>>,
}

type SharedNode<T> = Rc<RefCell<Node<T>>>;

pub struct Queue<T> {
    pub length: usize,
    pub head: Option<SharedNode<T>>,
    pub tail: Option<SharedNode<T>>,
}

impl<T> Queue<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Queue {
            length: 0,
            head: None,
            tail: None,
        }
    }

    pub fn enqueue(&mut self, item: T) {
        let new_node = Node {
            value: item,
            next: None,
        };

        let shared_new_node = Rc::new(RefCell::new(new_node));

        if self.length == 0 {
            self.head = Some(Rc::clone(&shared_new_node));
            self.tail = Some(Rc::clone(&shared_new_node));
            self.length += 1;
        } else {
            if let Some(tail) = &self.tail {
                tail.borrow_mut().next = Some(Rc::clone(&shared_new_node));
            }

            self.tail = Some(Rc::clone(&shared_new_node));
            self.length += 1;
        }
    }

    pub fn deque(&mut self) -> Option<T> {
        if self.length > 0 {
            if let Some(head) = &self.head {
                let pop_value = head.borrow().value.clone();

                let head = Rc::clone(head);
                let head = head.borrow();
                let next = head.next.as_ref();
                if let Some(next) = next {
                    self.head = Some(Rc::clone(next));
                } else {
                    self.head = None;
                    self.tail = None;
                }

                self.length -= 1;

                return Some(pop_value);
            }
        }
        return None;
    }

    pub fn peek(&self) -> Option<T> {
        if let Some(head) = &self.head {
            let value = head.borrow().value.clone();

            return Some(value);
        }

        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_works_normally() {
        let mut list = Queue::new();

        list.enqueue(5);
        list.enqueue(7);
        list.enqueue(9);

        assert_eq!(list.deque(), Some(5));
        assert_eq!(list.length, 2);

        list.enqueue(11);

        assert_eq!(list.deque(), Some(7));
        assert_eq!(list.deque(), Some(9));
        assert_eq!(list.peek(), Some(11));
        assert_eq!(list.deque(), Some(11));
        assert_eq!(list.deque(), None);
        assert_eq!(list.length, 0);

        list.enqueue(69);
        assert_eq!(list.peek(), Some(69));
        assert_eq!(list.length, 1);
    }
}
