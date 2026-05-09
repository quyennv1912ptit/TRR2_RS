// use my_app::scanner::Scanner;
use std::io::{Read, stdin};
use std::str::FromStr;

struct Scanner {
    tokens: std::vec::IntoIter<String>,
}

#[allow(dead_code)]
impl Scanner {
    fn new() -> Self {
        let mut buffer = String::new();
        stdin()
            .read_to_string(&mut buffer)
            .expect("Error: could not read input");
        Scanner {
            tokens: buffer
                .split_whitespace()
                .map(String::from)
                .collect::<Vec<_>>()
                .into_iter(),
        }
    }

    fn from_string(buffer: String) -> Self {
        Scanner {
            tokens: buffer
                .split_whitespace()
                .map(String::from)
                .collect::<Vec<_>>()
                .into_iter(),
        }
    }

    fn next<T: FromStr>(&mut self) -> T {
        self.tokens
            .next()
            .expect("EOF")
            .parse()
            .ok()
            .expect("Error: could not cast datatype")
    }
}

fn main() {
    let mut input = Scanner::new();
    let t: usize = input.next();

    for _ in 0..t {
        let n: usize = input.next();
        for i in (1..=n).rev() {
            print!("{} ", i);
        }
        println!();
    }
}
