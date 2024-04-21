use std::rc::Rc;

use dsa::queue::Queue;

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
