use my_app::scanner::Scanner;
use std::collections::VecDeque;

fn dfs(u: usize, adj: &Vec<Vec<usize>>, vi: &mut Vec<bool>, es: &mut Vec<(usize, usize)>) {
    vi[u] = true;

    for &v in &adj[u] {
        if !vi[v] {
            es.push((u, v));
            dfs(v, adj, vi, es);
        }
    }
}

fn bfs(s: usize, adj: &Vec<Vec<usize>>, vi: &mut Vec<bool>, es: &mut Vec<(usize, usize)>) {
    let mut q = VecDeque::<usize>::new();
    q.push_back(s);
    vi[s] = true;

    while let Some(u) = q.pop_front() {
        for &v in &adj[u] {
            if !vi[v] {
                vi[v] = true;
                es.push((u, v));
                q.push_back(v);
            }
        }
    }
}

fn main() {
    let mut input = Scanner::new();
    let q: usize = input.next();
    let n: usize = input.next();
    let s: usize = input.next();

    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    let mut vi: Vec<bool> = vec![false; n + 1];
    let mut es: Vec<(usize, usize)> = Vec::new();

    for i in 1..=n {
        for j in 1..=n {
            let x: usize = input.next();
            if x > 0 {
                adj[i].push(j);
            }
        }
    }

    if q == 1 {
        dfs(s, &adj, &mut vi, &mut es);
    } else {
        bfs(s, &adj, &mut vi, &mut es);
    }

    println!("{}", es.len());

    for (u, v) in &es {
        println!("{} {}", u, v);
    }
}
