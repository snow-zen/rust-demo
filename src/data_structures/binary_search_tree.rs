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
}
