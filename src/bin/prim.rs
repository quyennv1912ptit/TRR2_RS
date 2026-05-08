use my_app::scanner::Scanner;
use std::{
    cmp::{Reverse, max, min},
    collections::BinaryHeap,
    vec,
    fs,
};

const INF: usize = 10000;

fn prim(
    s: usize,
    n: usize,
    graph: &Vec<Vec<(usize, usize)>>,
    edges: &mut Vec<(usize, usize, usize)>,
) -> usize {
    //dist: store minimum distance from u to one node in in_mst array
    let mut dist = vec![INF; n + 1];
    //mark a node if exist in mst
    let mut in_mst = vec![false; n + 1];
    let mut parent = vec![0; n + 1];

    let mut h = 0;
    let mut pq = BinaryHeap::new();

    dist[s] = 0;
    pq.push(Reverse((0, s)));

    while !pq.is_empty() {
        if let Some(Reverse((w, u))) = pq.pop() {
            if in_mst[u] {
                continue;
            }

            in_mst[u] = true;
            h += w;

            if u != s {
                let u_min = min(u, parent[u]);
                let u_max = max(u, parent[u]);

                edges.push((u_min, u_max, w));
            }

            for &(weight, v) in &graph[u] {
                if !in_mst[v] && weight < dist[v] {
                    dist[v] = weight;
                    parent[v] = u;
                    pq.push(Reverse((weight, v)));
                }
            }
        }
    }

    h
}

#[allow(unused_variables)]
fn main() {
    let buffer = fs::read_to_string("CK.INP").expect("Error: could not open file");
    let mut cin = Scanner::from_string(buffer);
    let n: usize = cin.next();
    let s: usize = cin.next();
    let mut graph: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n + 1];
    let mut edges: Vec<(usize, usize, usize)> = Vec::new();

    for i in 1..=n {
        for j in 1..=n {
            let x: usize = cin.next();
            if x > 0 && x < INF {
                graph[i].push((x, j));
            }
        }
    }

    let ans = prim(s, n, &graph, &mut edges);
    // edges.sort();

    let mut out = String::new();

    out.push_str(&format!("{}\n", ans));

    for (u, v, w) in &edges {
        out.push_str(&format!("{} {} {}\n", u, v, w));
    }

    fs::write("CK.OUT", out).expect("Error: could not write file");
}
