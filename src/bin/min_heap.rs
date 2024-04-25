use std::fmt::Debug;
use std::fmt::Display;

#[allow(unused)]
struct MinHeap<T> {
    length: usize,
    data: Vec<T>,
}

#[allow(unused)]
impl<T> MinHeap<T>
where
    T: Copy + Clone + Display + Debug + PartialOrd,
{
    pub fn new() -> Self {
        Self {
            length: 0,
            data: Vec::new(),
        }
    }

    pub fn insert(&mut self, value: T) {
        match self.data.get(self.length) {
            Some(_) => self.data[self.length] = value,
            None => self.data.push(value),
        }
        self.heapify_up(self.length);
        self.length += 1;
    }

    pub fn delete(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }

        let out = self.data[0];
        self.length -= 1;

        if self.length == 0 {
            let out = self.data.pop().expect("Failed to pop the only element");

            return Some(out);
        }

        self.data[0] = self.data[self.length];
        self.heapify_down(0);

        return Some(out);
    }

    fn heapify_down(&mut self, idx: usize) {
        if idx >= self.length {
            return;
        }

        let l_idx = Self::left_child(idx);
        let r_idx = Self::right_child(idx);

        if l_idx >= self.length || r_idx >= self.length {
            return;
        }

        let l_v = self.data[l_idx];
        let r_v = self.data[r_idx];
        let v = self.data[idx];

        if l_v > r_v && v > r_v {
            self.data[r_idx] = v;
            self.data[idx] = r_v;

            self.heapify_down(r_idx);
        } else if r_v > l_v && v > l_v {
            self.data[l_idx] = v;
            self.data[idx] = l_v;

            self.heapify_down(l_idx);
        }
    }

    fn heapify_up(&mut self, idx: usize) {
        if idx == 0 {
            return;
        }

        let p = Self::parent(idx);
        let parent_v = self.data[p];
        let v = self.data[idx];

        if parent_v > v {
            self.data[idx] = parent_v;
            self.data[p] = v;

            self.heapify_up(p);
        }
    }

    fn parent(idx: usize) -> usize {
        match idx {
            0 => 0,
            _ => (idx - 1) / 2,
        }
    }

    fn left_child(idx: usize) -> usize {
        idx * 2 + 1
    }

    fn right_child(idx: usize) -> usize {
        idx * 2 + 2
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_heap_works() {
        let mut heap = MinHeap::new();

        assert_eq!(heap.length, 0);

        heap.insert(5);
        heap.insert(3);
        heap.insert(69);
        heap.insert(420);
        heap.insert(4);
        heap.insert(1);
        heap.insert(8);
        heap.insert(7);

        assert_eq!(heap.length, 8);
        assert_eq!(heap.delete(), Some(1));
        assert_eq!(heap.delete(), Some(3));
        assert_eq!(heap.delete(), Some(4));
        assert_eq!(heap.delete(), Some(5));
        assert_eq!(heap.length, 4);
        assert_eq!(heap.delete(), Some(7));
        assert_eq!(heap.delete(), Some(8));
        assert_eq!(heap.delete(), Some(69));
        assert_eq!(heap.delete(), Some(420));
        assert_eq!(heap.length, 0);
    }
}
