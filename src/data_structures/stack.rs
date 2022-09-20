pub struct Stack<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    val: T,
    next: Link<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            head: None
        }
    }

    pub fn push(&mut self, val: T) {
        let new_node = Box::new(Node {
            val,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(h) = self.head.take() {
            self.head = h.next;
            return Some(h.val);
        }
        None
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(h) = self.head.as_ref() {
            return Some(&h.val);
        }
        None
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref()
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut()
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for Stack<T> {
    #![allow(unused_assignments)]
    fn drop(&mut self) {
        let mut head = self.head.take();
        if let Some(mut it) = head {
            head = it.next.take();
        }
    }
}

pub struct IntoIter<T>(Stack<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|it| {
            self.next = it.next.as_deref_mut();
            &mut it.val
        })
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|x| {
            self.next = x.next.as_deref();
            &x.val
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
    }

    #[test]
    fn push_and_pop() {
        let mut stack = Stack::new();
        assert_eq!(stack.pop(), None);
        assert!(stack.is_empty());

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));

        assert_eq!(stack.is_empty(), false);

        stack.push(4);
        stack.push(5);

        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), Some(4));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);

        assert!(stack.is_empty());
    }

    #[test]
    fn peek() {
        let mut stack = Stack::new();
        assert_eq!(stack.peek(), None);

        stack.push(1);
        assert_eq!(stack.peek(), Some(&1));
        assert_eq!(stack.is_empty(), false);
    }
}
