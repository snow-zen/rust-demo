use std::cmp::Ordering;
use std::cmp::Ordering::Less;

#[cfg(test)]
mod tests;

/// 具有固定大小容量的优先队列
pub struct PriorityQueue<T: PartialOrd> {
    data: Vec<T>,
    size: usize,
    capacity: usize,
    compartor: Box<dyn Fn(&T, &T) -> Option<Ordering>>,
}

impl<T: PartialOrd> PriorityQueue<T> {
    /// 创建指定容量的优先队列，内部排序使用[`PartialOrd`]默认排序规则。
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_demo::heap::PriorityQueue;
    /// let pq: PriorityQueue<i32> = PriorityQueue::new(1);
    /// assert_eq!(pq.size(), 0);
    /// assert!(pq.is_empty());
    /// ```
    pub fn new(capacity: usize) -> Self {
        PriorityQueue {
            data: Vec::new(),
            size: 0,
            capacity,
            compartor: Box::new(|t1, t2| t1.partial_cmp(t2)),
        }
    }

    /// 创建指定容量的优先队列，同时可自定义排序规则。
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_demo::heap::PriorityQueue;
    /// let pq: PriorityQueue<i32> = PriorityQueue::with_compare(1, |i1: &i32, i2: &i32| i1.partial_cmp(i2));
    /// assert_eq!(pq.size(), 0);
    /// assert!(pq.is_empty());
    /// ```
    pub fn with_compare<F>(capacity: usize, compartor: F) -> Self
    where
        F: Fn(&T, &T) -> Option<Ordering> + 'static,
    {
        PriorityQueue {
            data: Vec::new(),
            size: 0,
            capacity,
            compartor: Box::new(compartor),
        }
    }

    /// 获取优先队列的当前元素数量
    pub fn size(&self) -> usize {
        self.size
    }

    /// 判断优先队列是否为空
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// 判断优先队列是否已满
    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    /// 清除优先队列中的所有元素
    pub fn clear(&mut self) {
        self.data.clear();
        self.size = 0;
    }

    /// 添加元素到优先队列
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_demo::heap::PriorityQueue;
    /// let mut pq = PriorityQueue::new(1);
    /// pq.push(1);
    /// assert_eq!(pq.size(), 1);
    /// ```
    pub fn push(&mut self, item: T) {
        self.data.push(item);
        self.size += 1;
        self.swim();
    }

    /// 从优先队列中删除队首元素
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_demo::heap::PriorityQueue;
    /// let mut pq = PriorityQueue::new(3);
    /// pq.push(1);
    /// pq.push(3);
    /// pq.push(2);
    /// assert_eq!(pq.pop(), 3);
    /// assert_eq!(pq.pop(), 2);
    /// assert_eq!(pq.pop(), 1);
    /// ```
    pub fn pop(&mut self) -> T {
        let max = self.data.swap_remove(0);
        self.size -= 1;
        self.sink();
        max
    }

    /// 元素上浮
    fn swim(&mut self) {
        let mut idx = self.size - 1;
        // while idx > 0 && (*self.compartor)(&self.data[idx / 2], &self.data[idx]) == Some(Less) {
        while idx > 0 && self.less(&self.data[idx / 2], &self.data[idx]) {
            self.data.swap(idx / 2, idx);
            idx /= 2;
        }
    }

    /// 元素下沉
    fn sink(&mut self) {
        let mut k = 0;
        while 2 * k + 1 < self.size {
            let mut j = 2 * k + 1;
            if let Some(x) = self.data.get(j + 1) {
                if self.less(&self.data[j], x) {
                    j += 1;
                }
            }
            if !self.less(&self.data[k], &self.data[j]) {
                break;
            }
            self.data.swap(k, j);
            k = j;
        }
    }

    /// 比较传入的两个参数中，前一个参数是否小于后一个参数
    fn less(&self, c1: &T, c2: &T) -> bool {
        (*self.compartor)(c1, c2) == Some(Less)
    }
}
