#[snippet = "union_find"]
pub struct UnionFind {
    par: Vec<usize>,
    size: Vec<usize>,
}

#[snippet = "union_find"]
impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            par: (0..n).collect(),
            size: vec![0; n],
        }
    }

    pub fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            self.par[x] = self.root(self.par[x]);
            self.par[x]
        }
    }

    pub fn merge(&mut self, x: usize, y: usize) -> bool {
        let x = self.root(x);
        let y = self.root(y);
        if x == y {
            return false;
        }
        if size[x] > size[y] {
            par[y] = x;
        } else {
            par[x] = y;
        }
    }
}
