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

    let mut visited = vec![false; graph.len()];
    loop {
        let tmp = dists.clone();
        if let Some((dist, i)) = tmp
            .into_iter()
            .enumerate()
            .filter(|&(i, _)| !visited[i])
            .map(|(i, v)| (v, i))
            .min()
        {
            visited[i] = true;
            for &(to, d) in &graph[i] {
                if dist + d < dists[to] {
                    dists[to] = dist + d;
                }
            }
        } else {
            break;
        }
    }

    dists
}

fn main() {
    let n = scan!(usize);

    let mut graph = vec![vec![]; n];

    for _ in 0..n {
        let vs = scan!(usize;;);
        let v = vs[0];
        let m = vs[1];
        for i in 0..m {
            graph[v].push((vs[1 + i * 2 + 1], vs[1 + i * 2 + 2]))
        }
    }

    for (i, d) in dikstra(&graph).into_iter().enumerate() {
        println!("{} {}", i, d);
    }
}
