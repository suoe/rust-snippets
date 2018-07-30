use std;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

pub fn read_testcase(path: &str) -> String {
    let path = Path::new(path);
    let mut file = match File::open(&path) {
        Err(msg) => panic!("{}", msg),
        Ok(file) => file,
    };

    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    s
}

pub fn read_from_directory(path: &str) -> String {
    let mut paths: Vec<String> = fs::read_dir(path)
        .unwrap()
        .map(|result| result.unwrap().path().display().to_string())
        .collect();
    paths.sort();

    paths
        .iter()
        .map(|path| read_testcase(path))
        .collect::<Vec<String>>()
        .join("\n")
}

pub struct TestCase<'a> {
    iter: std::iter::Peekable<std::str::SplitWhitespace<'a>>,
}

impl<'a> TestCase<'a> {
    pub fn new(s: &'a str) -> Self {
        TestCase {
            iter: s.split_whitespace().peekable(),
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

    pub fn is_empty(&mut self) -> bool {
        match self.iter.peek() {
            Some(_) => false,
            None    => true,
        }
    }
}
