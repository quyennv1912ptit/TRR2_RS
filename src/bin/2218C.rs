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
        let mut a = vec![0; 3*n];
        /*
        [1, 3n-1, 3n]
        [2, 3n-3, 3n-2]
        ...
        [n, 2n-1, 2n]
         */
        for i in 0..n {
            a[3*i] = i+1;
            a[3*i+1] = 3*n-1-2*i;
            a[3*i+2] = 3*n-2*i;
        }
        
        for x in &a {
            print!("{} ", x);
        }
        println!();
    }
}
