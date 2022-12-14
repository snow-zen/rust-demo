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
        let target_node = if self.val < val {
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

        let target_node = if self.val < *val {
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

/// ???????????????
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
    /// ?????????????????????????????????
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

    /// ????????????????????????????????????
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

    /// ?????????????????????????????????????????????
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

    /// ????????????????????????????????????
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

    /// ????????????????????????????????????
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

    /// ???????????????????????????????????????????????????
    ///
    /// # Example
    ///
    /// ```
    /// use rust_demo::data_structures::BinarySearchTree;
    /// let tree:BinarySearchTree<i32> = BinarySearchTree::new();
    ///
    /// assert_eq!(tree.floor(&1), None);
    /// ```
    pub fn floor(&self, val: &T) -> Option<&T> {
        match &self.root {
            None => None,
            Some(node) => node.floor(val),
        }
    }

    /// ???????????????????????????????????????????????????
    ///
    /// # Example
    ///
    /// ```
    /// use rust_demo::data_structures::BinarySearchTree;
    /// let tree:BinarySearchTree<i32> = BinarySearchTree::new();
    ///
    /// assert_eq!(tree.ceil(&1), None);
    /// ```
    pub fn ceil(&self, val: &T) -> Option<&T> {
        match &self.root {
            None => None,
            Some(node) => node.ceil(val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BinarySearchTree;

    fn perpare_tree() -> BinarySearchTree<i32> {
        let mut result = BinarySearchTree::new();
        result.insert(1);
        result.insert(2);
        result.insert(3);
        result.insert(5);
        result.insert(6);
        result.insert(7);
        result
    }

    #[test]
    fn test_search() {
        let tree = perpare_tree();
        assert!(tree.search(&1));
        assert!(tree.search(&2));
        assert!(tree.search(&3));

        assert!(!tree.search(&4));
    }

    #[test]
    fn test_max_and_min() {
        let tree = perpare_tree();
        assert_eq!(tree.max(), Some(&7));
        assert_eq!(tree.min(), Some(&1));
    }

    #[test]
    fn test_floor_and_ceil() {
        let tree = perpare_tree();
        assert_eq!(tree.floor(&4), Some(&3));
        assert_eq!(tree.ceil(&4), Some(&5));
    }
}
