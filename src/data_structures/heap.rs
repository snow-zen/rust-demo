use std::cmp::Ordering;
use std::cmp::Ordering::Less;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> Option<Ordering>,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> Option<Ordering>) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, item: T) {
        self.count += 1;
        self.items.push(item);

        let mut idx = self.count;
        while self.parent_idx(idx) > 0 {
            let parent_idx = self.parent_idx(idx);
            if (self.comparator)(&self.items[idx], &self.items[parent_idx]) == Some(Less) {
                self.items.swap(idx, parent_idx);
            }
            idx = parent_idx;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }

        self.count -= 1;
        let item = self.items.swap_remove(1);

        if self.count > 0 {
            let mut idx = 1;
            while self.children_present(idx) {
                let cdx = self.smallest_child_idx(idx);
                if (self.comparator)(&self.items[idx], &self.items[cdx]) != Some(Less) {
                    self.items.swap(idx, cdx);
                }
                idx = cdx
            }
        }
        Some(item)
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        if self.right_child_idx(idx) > self.count {
            self.left_child_idx(idx)
        } else {
            let ldx = self.left_child_idx(idx);
            let rdx = self.right_child_idx(idx);
            if (self.comparator)(&self.items[ldx], &self.items[rdx]) == Some(Less) {
                ldx
            } else {
                rdx
            }
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    pub fn new_min() -> Self {
        Self::new(|it1, it2| it1.partial_cmp(it2))
    }

    pub fn new_max() -> Self {
        Self::new(|it1, it2| it1.partial_cmp(it2).map(|res| res.reverse()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_heap() {
        let mut heap: Heap<i32> = Heap::new_min();
        assert!(heap.is_empty());
        assert_eq!(heap.len(), 0);
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn min_heap() {
        let mut heap = Heap::new_min();

        heap.push(4);
        heap.push(2);
        heap.push(9);
        heap.push(10);

        assert_eq!(heap.len(), 4);
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(9));
        assert_eq!(heap.pop(), Some(10));

        heap.push(1);
        assert_eq!(heap.pop(), Some(1));
    }

    #[test]
    fn max_heap() {
        let mut heap = Heap::new_max();

        heap.push(4);
        heap.push(2);
        heap.push(9);
        heap.push(10);

        assert_eq!(heap.len(), 4);
        assert_eq!(heap.pop(), Some(10));
        assert_eq!(heap.pop(), Some(9));
        assert_eq!(heap.pop(), Some(4));

        heap.push(1);
        assert_eq!(heap.pop(), Some(2));
    }

    struct Point(i32, i32);

    impl Default for Point {
        fn default() -> Self {
            Point(0, 0)
        }
    }

    #[test]
    fn custom_comparator_heap() {
        let mut heap: Heap<Point> = Heap::new(|p1, p2| p1.0.partial_cmp(&p2.0));

        heap.push(Point(1, 5));
        heap.push(Point(3, 10));
        heap.push(Point(-2, 4));

        assert_eq!(heap.len(), 3);
        assert_eq!(heap.pop().unwrap().0, -2);
        assert_eq!(heap.pop().unwrap().0, 1);

        heap.push(Point(50, 34));
        assert_eq!(heap.pop().unwrap().0, 3);
    }
}
