use std::{cell::RefCell, collections::HashMap, fmt::Debug, hash::Hash, ops::Deref, ptr, rc::Rc};

#[allow(unused)]
struct Node<T> {
    value: T,
    next: Option<SharedNode<T>>,
    prev: Option<SharedNode<T>>,
}

struct SharedNode<T>(Rc<RefCell<Node<T>>>);

impl<T> Deref for SharedNode<T> {
    type Target = Rc<RefCell<Node<T>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> PartialEq for SharedNode<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(self, other)
    }
}

impl<T> Eq for SharedNode<T> {}

impl<T> Hash for SharedNode<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ptr::hash(&**self, state)
    }
}

impl<T> Clone for SharedNode<T> {
    fn clone(&self) -> Self {
        SharedNode(Rc::clone(&self.0))
    }
}

#[allow(unused)]
impl<T> SharedNode<T> {
    fn new(value: T) -> SharedNode<T> {
        SharedNode(Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: None,
        })))
    }
}

#[allow(unused)]
struct LRU<K, V> {
    length: usize,
    capacity: usize,
    head: Option<SharedNode<V>>,
    tail: Option<SharedNode<V>>,

    lookup: HashMap<K, SharedNode<V>>,
    reverse_lookup: HashMap<SharedNode<V>, K>,
}

#[allow(unused)]
impl<K, V> LRU<K, V>
where
    K: Copy + Eq + PartialEq + Hash + Debug,
    V: Copy,
{
    fn new(capacity: usize) -> Self {
        Self {
            length: 0,
            capacity,
            head: None,
            tail: None,
            lookup: HashMap::new(),
            reverse_lookup: HashMap::new(),
        }
    }

    fn update(&mut self, key: K, value: V) {
        // does it exist?
        let node = self.lookup.get(&key);
        match node {
            None => {
                // if it doesn't we need to insert
                //  - check capacity and evict if over
                let node = SharedNode::new(value);
                self.length += 1;
                self.prepend(node.clone());
                self.trim_cache();

                self.lookup.insert(key, node.clone());
                self.reverse_lookup.insert(node.clone(), key);
            }
            Some(node) => {
                // if it does, we need to update to the front of the list
                // and update the value
                node.borrow_mut().value = value;

                let detach_node = node.clone();
                let prepend_node = node.clone();

                self.detach(detach_node);
                self.prepend(prepend_node);
            }
        }
    }

    fn get(&mut self, key: K) -> Option<V> {
        // check the cache for existence
        let node = self.lookup.get(&key);
        match node {
            None => return None,
            Some(node) => {
                // update the value we found and move it to the front
                let detach_node = node.clone();
                let prepend_node = node.clone();

                let v = node.borrow().value;

                self.detach(detach_node);
                self.prepend(prepend_node);

                // return out the value found
                Some(v)
            }
        }
    }

    fn detach(&mut self, node: SharedNode<V>) {
        if let Some(prev) = node.borrow().prev.as_ref() {
            prev.borrow_mut().next = match node.borrow().next.as_ref() {
                Some(next) => Some(next.clone()),
                None => None,
            }
        }

        if let Some(next) = node.borrow().next.as_ref() {
            next.borrow_mut().prev = match node.borrow().prev.as_ref() {
                Some(prev) => Some(prev.clone()),
                None => None,
            }
        }

        if let Some(head) = self.head.as_ref() {
            let head = head.clone();
            if head == node {
                self.head = match &head.borrow().next {
                    Some(next) => Some(next.clone()),
                    None => None,
                }
            }
        }
        if let Some(tail) = self.tail.as_ref() {
            let tail = tail.clone();
            if tail == node {
                self.tail = match &tail.borrow().prev {
                    Some(prev) => Some(prev.clone()),
                    None => None,
                }
            }
        }
    }

    fn prepend(&mut self, node: SharedNode<V>) {
        if self.head.is_none() {
            self.head = Some(node.clone());
            self.tail = Some(node.clone());

            return;
        }

        node.borrow_mut().next = self.head.clone();
        self.head.as_ref().unwrap().borrow_mut().prev = Some(node.clone());
        self.head = Some(node);
    }

    fn trim_cache(&mut self) {
        if self.length <= self.capacity {
            return;
        }

        let tail = self.tail.as_ref().unwrap().clone();
        self.detach(tail);

        let tail = self.tail.as_ref().unwrap().clone();
        let key = self.reverse_lookup.get(&tail).expect("Failed to find key");
        self.lookup.remove(&key);
        self.reverse_lookup.remove(&tail);
        self.length -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lru_works() {
        let mut lru: LRU<&str, u64> = LRU::new(3);

        assert_eq!(lru.get("foo"), None);
        lru.update("foo", 69);
        assert_eq!(lru.get("foo"), Some(69));

        lru.update("bar", 420);
        assert_eq!(lru.get("bar"), Some(420));

        lru.update("baz", 1337);
        assert_eq!(lru.get("baz"), Some(1337));

        lru.update("ball", 69420);
        assert_eq!(lru.get("ball"), Some(69420));
    }
}
