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
        let mut total = 0;
        let n: usize = input.next();
        let k: usize = input.next();
        let mut a: Vec<u32> = vec![0; n];
        for x in &mut a {
            *x = input.next();
            total += *x;
        }
        if (total % 2 != 0) || (n * k % 2 == 0) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
