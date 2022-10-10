pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

impl UnionFind {
    pub fn new(count: usize) -> Self {
        let mut parent = vec![0; count];
        let mut size = vec![0; count];
        for i in 0..count {
            parent[i] = i;
            size[i] = 1;
        }
        Self {
            parent,
            size,
            count,
        }
    }

    pub fn find(&self, x: usize) -> usize {
        self.validate(x);
        let mut x = x;
        while x != self.parent[x] {
            x = self.parent[x];
        }
        x
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        self.validate(x);
        self.validate(y);
        let x = self.find(x);
        let y = self.find(y);

        if x == y {
            return false;
        }

        if self.size[x] < self.size[y] {
            self.parent[x] = y;
            self.size[y] += self.size[x];
        } else {
            self.parent[y] = x;
            self.size[x] += self.size[y];
        }
        self.count -= 1;
        true
    }

    pub fn is_same_set(&self, x: usize, y: usize) -> bool {
        self.validate(x);
        self.validate(y);
        self.find(x) == self.find(y)
    }

    pub fn count(&self) -> usize {
        self.count
    }

    fn validate(&self, x: usize) {
        if x >= self.parent.len() {
            panic!("Index of bounds");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_find() {
        let mut uf = UnionFind::new(10);
        assert_eq!(uf.find(0), 0);
        assert_eq!(uf.find(1), 1);
        assert_eq!(uf.find(2), 2);
        assert_eq!(uf.find(3), 3);
        assert_eq!(uf.find(4), 4);
        assert_eq!(uf.find(5), 5);
        assert_eq!(uf.find(6), 6);
        assert_eq!(uf.find(7), 7);
        assert_eq!(uf.find(8), 8);
        assert_eq!(uf.find(9), 9);

        assert!(uf.union(0, 1));
        assert!(uf.union(1, 2));
        assert!(uf.union(2, 3));
        assert!(uf.union(3, 4));
        assert!(uf.union(4, 5));
        assert!(uf.union(5, 6));
        assert!(uf.union(6, 7));
        assert!(uf.union(7, 8));
        assert!(uf.union(8, 9));
        assert_eq!(uf.union(9, 0), false);

        assert_eq!(1, uf.count());
    }
}