pub struct UnionFind {
    size: usize,
    parent_or_size: Vec<i32>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        UnionFind {
            size,
            parent_or_size: vec![-1; size],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        assert!(x < self.size);
        if self.parent_or_size[x] < 0 {
            return x;
        }
        self.parent_or_size[x] = self.find(self.parent_or_size[x] as usize) as i32;
        self.parent_or_size[x] as usize
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        assert!(x < self.size);
        assert!(y < self.size);
        self.find(x) == self.find(y)
    }

    pub fn unit(&mut self, x: usize, y: usize) -> usize {
        assert!(x < self.size);
        assert!(y < self.size);
        let (mut x, mut y) = (self.find(x), self.find(y));
        if x == y {
            return x;
        }
        if -self.parent_or_size[x] < -self.parent_or_size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent_or_size[x] += self.parent_or_size[y];
        self.parent_or_size[y] = x as i32;
        x
    }

    pub fn group(&mut self) -> Vec<Vec<usize>> {
        let mut result = vec![Vec::new(); self.size];
        for i in 0..self.size {
            let parent = self.find(i);
            result[parent].push(i);
        }

        result
            .into_iter()
            .filter(|x| !x.is_empty())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn union_find_works() {
        let mut uf = UnionFind::new(5);

        assert_eq!(uf.find(0), 0);
        assert_eq!(uf.find(3), 3);

        assert_eq!(uf.same(1, 1), true);
        assert_eq!(uf.same(1, 2), false);

        uf.unit(0, 1);
        uf.unit(2, 3);
        assert_eq!(uf.same(0, 1), true);
        assert_eq!(uf.same(3, 2), true);
        assert_eq!(uf.same(0, 2), false);

        uf.unit(0, 2);
        assert_eq!(uf.same(1, 3), true);

        assert_eq!(uf.group(), vec![vec![0, 1, 2, 3], vec![4]]);
    }
}