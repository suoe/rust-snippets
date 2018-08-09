#[snippet = "fenwick_tree"]
pub trait Abel: Copy {
    fn id() -> Self;
    fn op(Self, Self) -> Self;
    fn inv(Self) -> Self;
    // Abelian Group must satisfy following laws:
    // op(id(), a) == op(a, id()) == a
    // op(a, op(b, c)) == op(op(a, b), c)
    // op(a, inv(a)) == op(inv(a), a) == id()
    // op(a, b) == op(b, a)
}

#[snippet = "fenwick_tree"]
pub struct FenwickTree<A> {
    n: usize,
    data: Vec<A>,
}

#[snippet = "fenwick_tree"]
impl<A: Abel> FenwickTree<A> {
    pub fn new(n: usize) -> FenwickTree<A> {
        FenwickTree {
            n: n,
            data: vec![A::id(); n + 1],
        }
    }

    pub fn add(&mut self, k: usize, a: A) {
        let mut k = k;
        while k <= self.n {
            self.data[k] = A::op(self.data[k], a);
            k += (k as isize & -(k as isize)) as usize;
        }
    }

    // [l, r)
    pub fn sum(&mut self, l: usize, r: usize) -> A {
        A::op(self.sum_from_one(r), A::inv(self.sum_from_one(l)))
    }

    fn sum_from_one(&mut self, k: usize) -> A {
        let mut s = A::id();
        let mut k = k - 1;
        while k > 0 {
            s = A::op(s, self.data[k]);
            k -= (k as isize & -(k as isize)) as usize;
        }
        s
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use util;
    use util::TestCase;

    // http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_B
    #[test]
    fn dsl_2_b() {
        let s = util::read_from_directory("./testcases/DSL_2_B/in");
        let mut cin = TestCase::new(&s);
        let s = util::read_from_directory("./testcases/DSL_2_B/out");
        let mut cout = TestCase::new(&s);

        type Sum = isize;

        impl Abel for Sum {
            fn id() -> Sum {
                0
            }

            fn op(a: Sum, b: Sum) -> Sum {
                a + b
            }

            fn inv(a: Sum) -> Sum {
                -a
            }
        }

        while !cin.is_empty() {
            let n: usize = cin.read();
            let q: usize = cin.read();
            let mut fen = FenwickTree::<Sum>::new(n);
            let queries: Vec<(usize, usize, usize)> = (0..q)
                .map(|_| (cin.read(), cin.read(), cin.read()))
                .collect();
            for query in queries {
                if query.0 == 0 {
                    fen.add(query.1, query.2 as isize);
                } else {
                    let res = fen.sum(query.1, query.2 + 1);
                    assert_eq!(res, cout.read());
                }
            }
        }
    }
}
