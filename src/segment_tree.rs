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
