use std::io::{Read, stdin};
use std::str::FromStr;

pub struct Scanner {
    tokens: std::vec::IntoIter<String>
}

impl Scanner {
    pub fn new() -> Self {
        let mut buffer = String::new();
        stdin().read_to_string(&mut buffer).expect("Error: could not read input");
        Scanner { tokens: buffer.split_whitespace().map(String::from).collect::<Vec<_>> ().into_iter() }
    }
    
    pub fn next<T: FromStr>(&mut self) -> T {
        self.tokens.next().expect("EOF").parse().ok().expect("Error: could not cast datatype")
    } 
}