use std::path::Path;
use std::fs::File;
use std::fs;
use std::io::Read;

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
    let mut paths: Vec<String> = fs::read_dir(path).unwrap()
        .map(|result| result.unwrap().path().display().to_string()).collect();
    paths.sort();

    let s: Vec<String> = paths.iter().map(|path| read_testcase(path)).collect();
    s.join("\n")
}
