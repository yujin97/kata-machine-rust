use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

struct Node<T> {
    value: T,
    next: Option<SharedNode<T>>,
    prev: Option<SharedNode<T>>,
}

type SharedNode<T> = Rc<RefCell<Node<T>>>;

impl<T> Node<T> {
    fn new_shared_node(value: T) -> SharedNode<T> {
        Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: None,
        }))
    }
}

struct LRU<K, V> {
    length: usize,
    head: Option<SharedNode<V>>,
    tail: Option<SharedNode<V>>,

    lookup: HashMap<K, SharedNode<V>>,
    reverse_lookup: HashMap<SharedNode<V>, K>,
}

impl<K, V> LRU<K, V>
where
    K: Eq + PartialEq + Hash,
    V: Copy,
{
    fn new(capacity: usize) -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
            lookup: HashMap::new(),
            reverse_lookup: HashMap::new(),
        }
    }

    fn update(key: K, value: V) {
        // does it exist?
        //
        // if it doesn't we need to insert
        //  - check capacity and evict if over
        // if it does, we need to update to the front of the list
        // and update the value
        todo!()
    }

    fn get(&mut self, key: K) -> Option<V> {
        // check the cache for existence
        let node = self.lookup.get(&key);
        match node {
            None => return None,
            Some(node) => {
                // update the value we found and move it to the front
                let detach_node = Rc::clone(&node);
                let prepend_node = Rc::clone(&node);

                let v = node.borrow().value;

                self.detach(detach_node);
                self.prepend(prepend_node);

                // return out the value found
                Some(v)
            }
        }
    }

    fn detach(&mut self, node: SharedNode<V>) {}

    fn prepend(&mut self, node: SharedNode<V>) {}
}
