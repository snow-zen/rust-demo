use std::ptr::NonNull;

struct Node<T> {
    val: T,
    prev: Link<T>,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Self {
            val,
            prev: None,
            next: None,
        }
    }
}

type Link<T> = Option<NonNull<Node<T>>>;

pub struct LinkedList<T> {
    size: usize,
    head: Link<T>,
    tail: Link<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            head: None,
            tail: None,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn offer_head(&mut self, val: T) {
        let mut node = Box::new(Node::new(val));
        node.next = self.head;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.head {
            None => self.tail = node_ptr,
            Some(old_head_ptr) => unsafe { (*old_head_ptr.as_ptr()).prev = node_ptr },
        }
        self.head = node_ptr;
        self.size += 1;
    }

    pub fn offer_tail(&mut self, val: T) {
        let mut node = Box::new(Node::new(val));
        node.prev = self.tail;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.tail {
            None => self.head = node_ptr,
            Some(old_tail_ptr) => unsafe { (*old_tail_ptr.as_ptr()).next = node_ptr },
        }
        self.tail = node_ptr;
        self.size += 1;
    }

    pub fn poll_head(&mut self) -> Option<T> {
        self.head.map(|old_head_ptr| unsafe {
            let old_head = Box::from_raw(old_head_ptr.as_ptr());
            match old_head.next {
                None => self.tail = None,
                Some(mut next_ptr) => next_ptr.as_mut().prev = None,
            }
            self.head = old_head.next;
            self.size -= 1;
            old_head.val
        })
    }

    pub fn poll_tail(&mut self) -> Option<T> {
        self.tail.map(|old_tail_ptr| unsafe {
            let old_tail = Box::from_raw(old_tail_ptr.as_ptr());
            match old_tail.prev {
                None => self.head = None,
                Some(mut prev_ptr) => prev_ptr.as_mut().next = None,
            }
            self.tail = old_tail.prev;
            self.size -= 1;
            old_tail.val
        })
    }

    pub fn insert(&mut self, index: usize, val: T) {
        if index > self.size {
            panic!("Index out of bounds");
        }

        if index == 0 || self.head.is_none() {
            self.offer_head(val);
            return;
        }

        if index == self.size {
            self.offer_tail(val);
            return;
        }

        if let Some(mut node_ptr) = self.head {
            for _ in 0..index {
                unsafe {
                    match (*node_ptr.as_ptr()).next {
                        None => panic!("Index out of bounds"),
                        Some(next_node_ptr) => node_ptr = next_node_ptr,
                    }
                }
            }

            let mut node = Box::new(Node::new(val));
            unsafe {
                node.prev = (*node_ptr.as_ptr()).prev;
                node.next = Some(node_ptr);
                if let Some(prev_node_ptr) = (*node_ptr.as_ptr()).prev {
                    let new_link = Some(NonNull::new_unchecked(Box::into_raw(node)));
                    (*prev_node_ptr.as_ptr()).next = new_link;
                    (*node_ptr.as_ptr()).prev = new_link;
                    self.size += 1;
                }
            }
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.size {
            panic!("Index out of bounds");
        }

        if index == 0 || self.head.is_none() {
            return self.poll_head();
        }

        if let Some(mut node_ptr) = self.head {
            for _ in 0..index {
                unsafe {
                    match (*node_ptr.as_ptr()).next {
                        None => panic!("Index out of bounds"),
                        Some(next_node_ptr) => node_ptr = next_node_ptr,
                    }
                }
            }

            unsafe {
                let old_node = Box::from_raw(node_ptr.as_ptr());
                if let Some(mut prev) = old_node.prev {
                    prev.as_mut().next = old_node.next;
                }
                if let Some(mut next) = old_node.next {
                    next.as_mut().prev = old_node.prev;
                }
                self.size -= 1;
                return Some(old_node.val);
            }
        }
        None
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.size {
            panic!("Index out of bounds");
        }

        if let Some(mut node_ptr) = self.head {
            for _ in 0..index {
                unsafe {
                    match (*node_ptr.as_ptr()).next {
                        None => panic!("Index out of bounds"),
                        Some(next_node_ptr) => node_ptr = next_node_ptr,
                    }
                }
            }

            unsafe {
                return Some(&(*node_ptr.as_ptr()).val);
            }
        }
        None
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.poll_head().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.size(), 0);
    }

    #[test]
    fn offer_head() {
        let mut list = LinkedList::new();
        let expect_val = 2;

        list.offer_head(1);
        list.offer_head(expect_val);

        assert_eq!(list.get(0), Some(&expect_val));
    }

    #[test]
    fn offer_tail() {
        let mut list = LinkedList::new();
        let expect_val = 2;

        list.offer_tail(1);
        list.offer_tail(expect_val);

        assert_eq!(list.get(1), Some(&expect_val));
    }

    #[test]
    fn poll_head() {
        let mut list = LinkedList::new();

        list.offer_head(1);
        list.offer_head(2);

        assert_eq!(list.poll_head(), Some(2));
        assert_eq!(list.poll_head(), Some(1));
        assert_eq!(list.size(), 0);
    }

    #[test]
    fn poll_tail() {
        let mut list = LinkedList::new();

        list.offer_tail(1);
        list.offer_tail(2);

        assert_eq!(list.poll_tail(), Some(2));
        assert_eq!(list.poll_tail(), Some(1));
        assert_eq!(list.size(), 0);
    }
}
