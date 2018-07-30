#[snippet = "segment_tree"]
pub trait Monoid: Copy {
    fn mempty() -> Self;
    fn mappend(a: Self, b: Self) -> Self;
}

#[snippet = "segment_tree"]
pub struct SegmentTree<M> {
    n: usize,
    data: Vec<M>,
}

#[snippet = "segment_tree"]
impl<M: Monoid> SegmentTree<M> {
    pub fn new(n: usize) -> SegmentTree<M> {
        let mut m = 1;
        while m <= n {
            m *= 2;
        }

        SegmentTree {
            n: m,
            data: vec![M::mempty(); 2 * m],
        }
    }

    pub fn update(&mut self, k: usize, a: M) {
        let mut k = k + self.n - 1;
        self.data[k] = a;
        while k > 0 {
            k = (k - 1) / 2;
            self.data[k] = M::mappend(self.data[k * 2 + 1], self.data[k * 2 + 2]);
        }
    }

    pub fn query(&self, l: usize, r: usize) -> M {
        self.query_range(l, r, 0, 0, self.n)
    }

    pub fn query_range(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> M {
        if r <= a || b <= l {
            M::mempty()
        } else if a <= l && r <= b {
            self.data[k]
        } else {
            let vl = self.query_range(a, b, k * 2 + 1, l, (l + r) / 2);
            let vr = self.query_range(a, b, k * 2 + 2, (l + r) / 2, r);
            M::mappend(vl, vr)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std;
    use util;
    use util::TestCase;

    // http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A
    #[test]
    fn dsl_2_a() {
        let s = util::read_from_directory("./testcases/DSL_2_A/in");
        let mut cin = TestCase::new(&s);
        let s = util::read_from_directory("./testcases/DSL_2_A/out");
        let mut cout = TestCase::new(&s);

        type Min = usize;

        impl Monoid for Min {
            fn mempty() -> Min {
                2147483647 // 2^31 - 1
            }

            fn mappend(a: Min, b: Min) -> Min {
                std::cmp::min(a, b)
            }
        }

        while !cin.is_empty() {
            let n: usize = cin.read();
            let q: usize = cin.read();
            let mut seg = SegmentTree::<Min>::new(n);
            let queries: Vec<(usize, usize, usize)> = (0..q)
                .map(|_| (cin.read(), cin.read(), cin.read()))
                .collect();
            for query in queries {
                if query.0 == 0 {
                    seg.update(query.1, query.2);
                } else {
                    let res = seg.query(query.1, query.2 + 1);
                    assert_eq!(res, cout.read());
                }
            }
        }
    }
}
