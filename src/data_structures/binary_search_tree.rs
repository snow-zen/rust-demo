use std::cmp::Ordering;

struct Node<T>
where
    T: PartialOrd,
{
    val: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T> Node<T>
where
    T: PartialOrd,
{
    fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, val: T) {
        let target_node = if self.val > val {
            &mut self.left
        } else {
            &mut self.right
        };

        match target_node {
            None => *target_node = Some(Box::new(Node::new(val))),
            Some(ref mut node) => {
                node.insert(val);
            }
        }
    }

    fn search(&self, val: &T) -> bool {
        if self.val == *val {
            return true;
        }

        let target_node = if self.val > *val {
            &self.left
        } else {
            &self.right
        };

        match target_node {
            None => false,
            Some(ref node) => node.search(val),
        }
    }

    fn max(&self) -> Option<&T> {
        match &self.left {
            None => Some(&self.val),
            Some(left_node) => left_node.max(),
        }
    }

    fn min(&self) -> Option<&T> {
        match &self.right {
            None => Some(&self.val),
            Some(right_node) => right_node.min(),
        }
    }

    fn floor(&self, val: &T) -> Option<&T> {
        match self.val.partial_cmp(val) {
            Some(Ordering::Greater) => {
                // node.val > val
                match &self.right {
                    None => None,
                    Some(right_node) => right_node.floor(val),
                }
            }
            Some(Ordering::Less) => {
                // node.val < val
                match &self.left {
                    None => Some(&self.val),
                    Some(left_node) => {
                        let floor_node = left_node.floor(val);
                        match floor_node {
                            None => Some(&self.val),
                            Some(_) => floor_node,
                        }
                    }
                }
            }
            Some(Ordering::Equal) => Some(&self.val),
            None => None,
        }
    }

    fn ceil(&self, val: &T) -> Option<&T> {
        match self.val.partial_cmp(val) {
            Some(Ordering::Greater) => {
                // node.val > val
                match &self.right {
                    None => Some(&self.val),
                    Some(right_node) => {
                        let ceil_node = right_node.ceil(val);
                        match ceil_node {
                            Some(_) => ceil_node,
                            None => Some(&self.val),
                        }
                    }
                }
            }
            Some(Ordering::Less) => {
                // node.val < val
                match &self.left {
                    None => None,
                    Some(left_node) => left_node.ceil(val),
                }
            }
            Some(Ordering::Equal) => Some(&self.val),
            None => None,
        }
    }
}

type Link<T> = Option<Box<Node<T>>>;

/// 二叉搜索树
pub struct BinarySearchTree<T>
where
    T: PartialOrd,
{
    root: Link<T>,
}

impl<T> BinarySearchTree<T>
where
    T: PartialOrd,
{
    /// 创建一个空的二叉搜索树
    ///
    /// # Example
    ///
    /// ```
    /// use rust_demo::data_structures::BinarySearchTree;
    /// let tree:BinarySearchTree<i32> = BinarySearchTree::new();
    /// ```
    pub fn new() -> Self {
        Self { root: None }
    }

    /// 向二叉搜索树添加一个元素
    ///
    /// # Example
    ///
    /// ```
    /// use rust_demo::data_structures::BinarySearchTree;
    /// let mut tree = BinarySearchTree::new();
    ///
    /// tree.insert(1);
    /// ```
    pub fn insert(&mut self, val: T) {
        match &mut self.root {
            None => self.root = Some(Box::new(Node::new(val))),
            Some(ref mut node) => {
                node.insert(val);
            }
        }
    }

    /// 查询二叉搜索树中是否存在指定值
    ///
    /// # Example
    ///
    /// ```
    /// use rust_demo::data_structures::BinarySearchTree;
    /// let tree = BinarySearchTree::new();
    ///
    /// assert_eq!(tree.search(&1), false);
    /// ```
    pub fn search(&self, val: &T) -> bool {
        match self.root {
            None => false,
            Some(ref node) => node.search(val),
        }
    }

    /// 获取二叉搜索树中的最大值
    ///
    /// # Example
    ///
    /// ```
    /// use rust_demo::data_structures::BinarySearchTree;
    /// let tree:BinarySearchTree<i32> = BinarySearchTree::new();
    ///
    /// assert_eq!(tree.max(), None);
    /// ```
    pub fn max(&self) -> Option<&T> {
        match &self.root {
            None => None,
            Some(node) => node.max(),
        }
    }

    /// 获取二叉搜索树中的最小值
    ///
    /// # Example
    ///
    /// ```
    /// use rust_demo::data_structures::BinarySearchTree;
    /// let tree:BinarySearchTree<i32> = BinarySearchTree::new();
    ///
    /// assert_eq!(tree.min(), None);
    /// ```
    pub fn min(&self) -> Option<&T> {
        match &self.root {
            None => None,
            Some(node) => node.min(),
        }
    }

    /// 获取二叉搜索树中小于指定值的最大值
    ///
    /// # Example
    ///
    /// ```
    /// use rust_demo::data_structures::BinarySearchTree;
    /// let tree:BinarySearchTree<i32> = BinarySearchTree::new();
    ///
    /// assert_eq!(tree.floor(), None);
    /// ```
    pub fn floor(&self, val: &T) -> Option<&T> {
        match &self.root {
            None => None,
            Some(node) => node.floor(val),
        }
    }

    /// 获取二叉搜索树中大于指定值的最小值
    ///
    /// # Example
    ///
    /// ```
    /// use rust_demo::data_structures::BinarySearchTree;
    /// let tree:BinarySearchTree<i32> = BinarySearchTree::new();
    ///
    /// assert_eq!(tree.ceil(), None);
    /// ```
    pub fn ceil(&self, val: &T) -> Option<&T> {
        match &self.root {
            None => None,
            Some(node) => node.ceil(val),
        }
    }
}
