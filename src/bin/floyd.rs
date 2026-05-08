use my_app::scanner::Scanner;

const INF: i32 = 10000;

fn find_path(u: usize, v: usize, next: &Vec<Vec<usize>>) -> Option<Vec<usize>> {
    if next[u][v] == 0 {
        return None;
    }

    let mut path: Vec<usize> = Vec::new();

    path.push(u);

    let mut t = u;

    while t != v {
        t = next[t][v];
        path.push(t);
    }

    Some(path)
}

fn main() {
    let mut input = Scanner::new();
    let n: usize = input.next();
    let u: usize = input.next();
    let v: usize = input.next();
    let mut d = vec![vec![0; n + 1]; n + 1];
    let mut next = vec![vec![0; n + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=n {
            let x: i32 = input.next();
            d[i][j] = x;
            if d[i][j] != INF {
                next[i][j] = j;
            }
        }
    }

    for k in 1..=n {
        for i in 1..=n {
            for j in 1..=n {
                if d[i][j] > d[i][k] + d[k][j] {
                    d[i][j] = d[i][k] + d[k][j];
                    next[i][j] = next[i][k];
                }
            }
        }
    }

    let path = find_path(u, v, &next);

    if let Some(path) = path {
        println!("{}", d[u][v]);
        for v in &path {
            print!("{} ", v);
        }
        println!();
    } else {
        println!("0");
    }
}
