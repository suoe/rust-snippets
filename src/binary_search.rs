#[snippet = "binary_search"]
pub trait BinarySearch<T, F> {
    fn bin_search(&self, cond: F) -> Option<T>;
}

#[snippet = "binary_search"]
pub trait LowerBound<T> {
    fn lower_bound(&self, x: &T) -> Option<usize>;
}

#[snippet = "binary_search"]
pub trait UpperBound<T> {
    fn upper_bound(&self, x: &T) -> Option<usize>;
}

#[snippet = "binary_search"]
macro_rules! impl_bounds {
    ($t:ty, $u:ident, $($ts:ident),+) => {
        impl<$u: Ord, $($ts),+> LowerBound<$u> for $t<$($ts),+> {
            fn lower_bound(&self, x: &$u) -> Option<usize> {
                self.bin_search( | value: &$u | value >= x)
            }
        }

        impl<$u: Ord, $($ts),+> UpperBound<$u> for $t<$($ts),+> {
            fn upper_bound(&self, x: &$u) -> Option<usize> {
                self.bin_search( | value: &$u | value > x)
            }
        }
    };

    ($t:ty, $u:ident) => {
        impl<$u: Ord> LowerBound<$u> for $t {
            fn lower_bound(&self, x: &$u) -> Option<usize> {
                self.bin_search( | value: &$u | value >= x)
            }
        }

        impl<$u: Ord> UpperBound<$u> for $t {
            fn upper_bound(&self, x: &$u) -> Option<usize> {
                self.bin_search( | value: &$u | value > x)
            }
        }
    };
}

#[snippet = "binary_search"]
impl<T, F> BinarySearch<usize, F> for [T]
where
    F: Fn(&T) -> bool,
{
    // Assume that the slice is monotone on givin contdition,
    // that is, condition is like [fffffftttttttt]
    fn bin_search(&self, cond: F) -> Option<usize> {
        let mut left = -1;
        let mut right = self.len() as isize;

        while right - left > 1 {
            let mid = (right + left) / 2;
            if cond(&self[mid as usize]) {
                right = mid;
            } else {
                left = mid;
            }
        }
        if right == self.len() as isize {
            None
        } else if left == -1 {
            Some(0)
        } else {
            Some(right as usize)
        }
    }
}

#[snippet = "binary_search"]
impl_bounds!([T], T);

#[cfg(test)]
mod test {
    use super::*;
    use rand;
    use rand::Rng;

    #[test]
    fn bin_search_on_slice() {
        let mut rng = rand::thread_rng();
        let mut numbers: Vec<usize> = (0..10000).map(|_| {
            rng.gen_range(1, 21)
        }).collect();
        numbers.sort();

        let count_element = | x | {
            let mut table: Vec<Option<usize>> = vec![None; 21];
            if let Some(n) = table[x] {
                return n;
            }
            let mut count = 0;
            for n in &numbers {
                if *n == x {
                    count += 1;
                } else {
                    if count == 0 {
                        continue;
                    } else {
                        break;
                    }
                }
            }
            table[x] = Some(count);
            count
        };

        for i in 1..=20 {
            if let Some(n) = numbers.lower_bound(&i) {
                if let Some(m) = numbers.upper_bound(&i) {
                    let count = m - n;
                    assert_eq!(count, count_element(i));
                } else {
                    let m = numbers.len();
                    let count = m - n;
                    assert_eq!(count, count_element(i));
                }
            }
        }
    }
}
