use std::collections::BinaryHeap;

#[allow(unused_macros)]
macro_rules! scan {
    () => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().to_string()
        }
    };
    (;;) => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>()
        }
    };
    (;;$n:expr) => {
        {
            (0..$n).map(|_| scan!()).collect::<Vec<_>>()
        }
    };
    ($t:ty) => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
        }
    };
    ($($t:ty),*) => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                $(iter.next().unwrap().parse::<$t>().unwrap(),)*
            )
        }
    };
    ($t:ty;;) => {
        {
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
        }
    };
    ($t:ty;;$n:expr) => {
        (0..$n).map(|_| scan!($t;;)).collect::<Vec<_>>()
    };
    ($t:ty; $n:expr) => {
        (0..$n).map(|_|
                    scan!($t)
        ).collect::<Vec<_>>()
    };
    ($($t:ty),*; $n:expr) => {
        (0..$n).map(|_|
                    scan!($($t),*)
        ).collect::<Vec<_>>()
    };
}

fn dikstra(graph: &[Vec<(usize, usize)>]) -> Vec<usize> {
    let mut dists = vec![1usize << 30; graph.len()];

    dists[0] = 0;

    let mut h = BinaryHeap::new();
    h.push((0, 0));
    while let Some((_, n)) = h.pop() {
        for &(to, d) in &graph[n] {
            let dist = dists[n] + d;
            if dist < dists[to] {
                dists[to] = dist;
                h.push((-(dists[to] as i64), to))
            }
        }
    }
    dists
}

fn main() {
    let n = scan!(usize);

    let mut graph = vec![vec![]; n];
    for _ in 0..n {
        let vs = scan!(usize;;);
        for i in 0..vs[1] {
            graph[vs[0]].push((vs[(i + 1) * 2], vs[(i + 1) * 2 + 1]))
        }
    }
    let dists = dikstra(&graph);
    for (i, d) in dists.into_iter().enumerate() {
        println!("{} {}", i, d)
    }
}
