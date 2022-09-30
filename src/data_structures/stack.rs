/// 基于单链表实现的栈
///
/// 栈内元素满足先进后出（FILO）顺序，因此在栈顶添加和删除元素的时间复杂度为 O(1)。
pub struct Stack<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    val: T,
    next: Link<T>,
}

impl<T> Stack<T> {
    /// 创建一个空元素的栈
    ///
    /// # Example
    ///
    /// ```
    /// use rust_demo::data_structures::Stack;
    /// let stack:Stack<i32> = Stack::new();
    /// ```
    pub fn new() -> Self {
        Stack { head: None }
    }

    /// 向栈中添加一个元素
    /// 
    /// # Example
    /// 
    /// ```
    /// use rust_demo::data_structures::Stack;
    /// let mut stack = Stack::new();
    /// stack.push(1);
    /// ```
    pub fn push(&mut self, val: T) {
        let new_node = Box::new(Node {
            val,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    /// 从栈中移除一个元素
    /// 
    /// # Example
    /// 
    /// ```
    /// use rust_demo::data_structures::Stack;
    /// let mut stack = Stack::new();
    /// 
    /// assert_eq!(stack.pop(), None);
    /// 
    /// stack.push(1);
    /// assert_eq!(stack.pop(), Some(1));
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        if let Some(h) = self.head.take() {
            self.head = h.next;
            return Some(h.val);
        }
        None
    }

    /// 判断栈是否为空
    /// 
    /// # Example
    /// 
    /// ```
    /// use rust_demo::data_structures::Stack;
    /// let mut stack:Stack<i32> = Stack::new();
    /// 
    /// assert!(stack.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// 查看栈顶元素
    /// 
    /// # Example
    /// 
    /// ```
    /// use rust_demo::data_structures::Stack;
    /// let mut stack = Stack::new();
    /// 
    /// assert_eq!(stack.peek(), None);
    /// 
    /// stack.push(1);
    /// assert_eq!(stack.peek(), Some(&1));
    /// ```
    pub fn peek(&self) -> Option<&T> {
        if let Some(h) = self.head.as_ref() {
            return Some(&h.val);
        }
        None
    }

    /// 获取栈元素的不可变引用迭代器
    /// 
    /// # Example
    /// 
    /// ```
    /// use rust_demo::data_structures::Stack; 
    /// let stack:Stack<i32> = Stack::new();
    /// let iter = stack.iter();
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    /// 获取栈元素的可变引用迭代器
    /// 
    /// # Example
    /// 
    /// ```
    /// use rust_demo::data_structures::Stack; 
    /// let mut stack:Stack<i32> = Stack::new();
    /// let iter_mut = stack.iter_mut();
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }

    /// 获取栈元素的迭代器
    /// 
    /// # Example
    /// 
    /// ```
    /// use rust_demo::data_structures::Stack; 
    /// let stack:Stack<i32> = Stack::new();
    /// let into_iter = stack.into_iter();
    /// ```
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
