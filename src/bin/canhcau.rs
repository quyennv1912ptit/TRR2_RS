use my_app::scanner::Scanner;
use std::cmp::min;

fn dfs(
    u: usize,
    p: usize,
    adj: &[Vec<usize>],
    num: &mut Vec<u32>,
    low: &mut Vec<u32>,
    time_dfs: &mut u32,
    es: &mut Vec<(usize, usize)>,
) {
    *time_dfs += 1;
    num[u] = *time_dfs;
    low[u] = *time_dfs;

    for &v in &adj[u] {
        if v == p {
            continue;
        }
        if num[v] == 0 {
            dfs(v, u, adj, num, low, time_dfs, es);
            low[u] = min(low[u], low[v]);

            if low[v] > num[u] {
                let e = if u < v {(u,v)} else {(v, u)};
                es.push(e);
            }
        } else {
            low[u] = min(num[v], low[u]);
        }
    }
}

fn main() {
    let mut input = Scanner::new();
    let n: usize = input.next();

    let mut adj = vec![Vec::new(); n + 1];
    let mut num = vec![0; n + 1];
    let mut low = vec![0; n + 1];
    let mut es = Vec::new();

    let mut time_dfs = 0;

    for i in 1..=n {
        for j in 1..=n {
            let x: u32 = input.next();
            if x > 0 {
                adj[i].push(j);
            }
        }
    }

    for v in 1..=n {
        if num[v] == 0 {
            dfs(
                v,
                v,
                &adj,
                &mut num,
                &mut low,
                &mut time_dfs,
                &mut es,
            );
        }
    }

    es.sort();

    println!("{}", es.len());
    for &(u, v) in &es {
        println!("{} {}", u, v);
    }
}
