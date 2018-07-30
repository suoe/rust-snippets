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
            let par = self.par[x];
            self.par[x] = self.root(par);
            self.par[x]
        }
    }

    pub fn merge(&mut self, x: usize, y: usize) -> bool {
        let x = self.root(x);
        let y = self.root(y);
        if x == y {
            return false;
        }
        if self.size[x] > self.size[y] {
            self.par[y] = x;
            self.size[x] += self.size[y];
        } else {
            self.par[x] = y;
            self.size[y] += self.size[x];
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use util;
    use util::TestCase;

    // http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_1_A
    #[test]
    fn dsl_1_a() {
        let s = util::read_from_directory("./testcases/DSL_1_A/in");
        let mut cin = TestCase::new(&s);
        let s = util::read_from_directory("./testcases/DSL_1_A/out");
        let mut cout = TestCase::new(&s);

        while !cin.is_empty() {
            let n: usize = cin.read();
            let q: usize = cin.read();
            let queries: Vec<(usize, usize, usize)> = (0..q)
                .map(|_| (cin.read(), cin.read(), cin.read()))
                .collect();
            let mut uf = UnionFind::new(n);

            for query in queries {
                if query.0 == 0 {
                    uf.merge(query.1, query.2);
                } else {
                    let res = if uf.root(query.1) == uf.root(query.2) {
                        1
                    } else {
                        0
                    };
                    assert_eq!(res, cout.read())
                }
            }
        }
    }
}
