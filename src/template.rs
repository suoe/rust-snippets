use std;
#[snippet = "template"]
use std::io::Read;

#[snippet = "template"]
fn main() {
    let buf = {
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        s
    };
    let mut sc = io::Scanner::new(&buf);
}

#[snippet = "template"]
mod io {
    use std::str::{FromStr, SplitWhitespace};

    pub struct Scanner<'a> {
        pub iter: SplitWhitespace<'a>,
    }

    impl<'a> Scanner<'a> {
        pub fn new(s: &'a str) -> Scanner<'a> {
            Scanner {
                iter: s.split_whitespace(),
            }
        }

        pub fn read<T: FromStr>(&mut self) -> T {
            self.iter
                .next()
                .unwrap()
                .parse()
                .ok()
                .expect("parsing failed")
        }

        pub fn read_vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
            (0..n).map(|_| self.read()).collect()
        }
    }
}
