use std;

#[snippet = "segment_tree"]
pub trait Monoid: Copy {
    fn id() -> Self;
    fn op(a: Self, b: Self) -> Self;
    // Monoid must satisfy following laws:
    // op(a, id()) == op(id(), a) == a
    // op(a, op(b, c)) == op(op(a, b), c)
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
            data: vec![M::id(); 2 * m],
        }
    }

    pub fn update(&mut self, k: usize, a: M) {
        let mut k = k + self.n - 1;
        self.data[k] = a;
        while k > 0 {
            k = (k - 1) / 2;
            self.data[k] = M::op(self.data[k * 2 + 1], self.data[k * 2 + 2]);
        }
    }

    pub fn query(&self, l: usize, r: usize) -> M {
        self.query_range(l, r, 0, 0, self.n)
    }

    fn query_range(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> M {
        if r <= a || b <= l {
            M::id()
        } else if a <= l && r <= b {
            self.data[k]
        } else {
            let vl = self.query_range(a, b, k * 2 + 1, l, (l + r) / 2);
            let vr = self.query_range(a, b, k * 2 + 2, (l + r) / 2, r);
            M::op(vl, vr)
        }
    }
}

#[snippet = "min_monoid"]
type Min = usize;
impl Monoid for Min {
    #[inline]
    fn id() -> Min {
        2147483647
    }

    #[inline]
    fn op(a: Min, b: Min) -> Min {
        std::cmp::min(a, b)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use util;
    use util::TestCase;

    // http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A
    #[test]
    fn dsl_2_a() {
        let s = util::read_from_directory("./testcases/DSL_2_A/in");
        let mut cin = TestCase::new(&s);
        let s = util::read_from_directory("./testcases/DSL_2_A/out");
        let mut cout = TestCase::new(&s);

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
