use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

struct Node<T> {
    value: T,
    next: Option<SharedNode<T>>,
}

type SharedNode<T> = Rc<RefCell<Node<T>>>;

struct Queue<T> {
    length: usize,
    head: Option<SharedNode<T>>,
    tail: Option<SharedNode<T>>,
}

impl<T> Queue<T>
where
    T: Copy + Display,
{
    fn new() -> Self {
        Queue {
            length: 0,
            head: None,
            tail: None,
        }
    }

    fn enqueue(&mut self, item: T) {
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

    fn deque(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        } else if self.length == 1 {
            if let Some(pop_value_ref) = &self.head {
                let pop_value = pop_value_ref.borrow().value;

                self.head = None;
                self.tail = None;
                self.length -= 1;

                return Some(pop_value);
            }
            return None;
        } else {
            if let Some(pop_value_ref) = &self.tail {
                let pop_value = pop_value_ref.borrow().value;
                if let Some(head) = &self.head {
                    let mut temp = Rc::clone(&head);
                    for i in 0..self.length - 1 {
                        let next = Rc::clone(
                            temp.borrow()
                                .next
                                .as_ref()
                                .expect("Failed to traverse queue"),
                        );
                        if i < self.length - 2 {
                            temp = next;
                        }
                    }

                    temp.borrow_mut().next = None;

                    self.tail = Some(temp);
                    self.length -= 1;
                }

                return Some(pop_value);
            }
            return None;
        }
    }

    fn peek(&self) -> Option<T> {
        if let Some(head) = &self.head {
            let value = head.borrow().value;

            return Some(value);
        }

        return None;
    }
}

fn main() {
    let mut queue = Queue::new();

    queue.enqueue(123);
    queue.enqueue(69);
    queue.enqueue(420);

    println!("head: {:?}", queue.peek());

    queue.deque();

    let mut head = queue.head.clone();
    while let Some(node) = head {
        println!("value: {}", node.borrow().value);

        if let Some(next) = &node.borrow().next {
            let next = Rc::clone(next);

            head = Some(next);
        } else {
            head = None;
        }
    }
}
