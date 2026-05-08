use my_app::scanner::Scanner;
use std::{cmp::Reverse, collections::BinaryHeap, fs};

const INF: u32 = 10000;

fn main() {
    let buffer = fs::read_to_string("DN.INP").expect("Could not read file");
    let mut input = Scanner::from_string(buffer);

    let n: usize = input.next();
    let s: usize = input.next();
    let t: usize = input.next();

    let mut adj: Vec<Vec<(u32, usize)>> = vec![Vec::new(); n + 1];

    for i in 1..=n {
        for j in 1..=n {
            let x: u32 = input.next();
            if x > 0 && x < INF {
                adj[i].push((x, j));
            }
        }
    }

    //dijikstra
    let mut dist: Vec<u32> = vec![INF; n + 1];
    let mut parent: Vec<usize> = vec![0; n + 1];

    let mut min_heap = BinaryHeap::new();
    dist[s] = 0;
    min_heap.push(Reverse((0, s)));

    while let Some(Reverse((d, u))) = min_heap.pop() {
        if d > dist[u] {
            continue;
        }
        for &(w, v) in &adj[u] {
            if dist[v] > dist[u] + w {
                dist[v] = dist[u] + w;
                parent[v] = u;
                min_heap.push(Reverse((dist[v], v)));
            }
        }
    }

    let mut out = String::new();

    if dist[t] != INF {
        out.push_str(&format!("{}\n", dist[t]));
        let mut tmp = t;
        let mut path = Vec::new();
        path.push(tmp);
        while tmp != s {
            tmp = parent[tmp];
            path.push(tmp);
        }
        path.reverse();
        for v in &path {
            out.push_str(&format!("{} ", v));
        }
        out.push_str("\n");
    } else {
        out.push_str("0\n");
    }

    fs::write("DN.OUT", out).expect("Could not write file");
}
