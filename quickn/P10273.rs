/*
    date   : 2020 / 5 / 5
    author : quickn (quickn.ga)
    email  : quickwshell@gmail.com
*/

use std::io::{self, BufWriter, Write};

mod scanner {
    use std::{io, str};
    /* https://github.com/EbTech/rust-algorithms */

    /// Same API as Scanner but nearly twice as fast, using horribly unsafe dark arts
    /// **REQUIRES** Rust 1.34 or higher
    pub struct UnsafeScanner<R> {
        reader: R,
        buf_str: Vec<u8>,
        buf_iter: str::SplitAsciiWhitespace<'static>,
    }

    impl<R: io::BufRead> UnsafeScanner<R> {
        pub fn new(reader: R) -> Self {
            Self {
                reader,
                buf_str: Vec::new(),
                buf_iter: "".split_ascii_whitespace(),
            }
        }

        /// This function should be marked unsafe, but noone has time for that in a
        /// programming contest. Use at your own risk!
        pub fn token<T: str::FromStr>(&mut self) -> T {
            loop {
                if let Some(token) = self.buf_iter.next() {
                    return token.parse().ok().expect("Failed parse");
                }
                self.buf_str.clear();
                self.reader
                    .read_until(b'\n', &mut self.buf_str)
                    .expect("Failed read");
                self.buf_iter = unsafe {
                    let slice = str::from_utf8_unchecked(&self.buf_str);
                    std::mem::transmute(slice.split_ascii_whitespace())
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Graph {
    adj: Vec<Vec<(usize, i64)>>,
    res: Vec<usize>,
    visited: Vec<bool>,
}

impl Graph {
    fn new(adj: Vec<Vec<(usize, i64)>>) -> Self {
        let n =adj.len();
        Self {
            adj,
            res: vec![],
            visited: vec![false;n]
        }
    }

    fn dfs(&mut self, u: usize) {
        if !self.visited[u] {
        self.visited[u] = true;
        for v in self.adj[u].clone() {
            if !self.visited[v.0] {
                self.dfs(v.0);
            }
        }
        self.res.push(u);
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let t: usize = scan.token();
    for _case in 0..t {
        let (n, e): (usize, usize) = (scan.token(), scan.token());
        let mut cost: Vec<i64> = vec![0;n];
        for i in 0..n {
            cost[i] = scan.token();
        }
        let mut adj: Vec<Vec<(usize, i64)>> = vec![vec![];n];
        for i in 0..e {
            let (ae, be, ce): (usize, usize, i64) = (scan.token(), scan.token(), scan.token());
            adj[ae-1].push((be-1, ce));
        }
        let mut g = Graph::new(adj.clone());
        for i in 0..n {
            g.dfs(i);
        }
        let ord: Vec<usize> = g.res.clone();
        let mut dp: Vec<i64> = vec![std::i64::MIN;n];
        let mut pi: Vec<usize> = vec![n;n];
        for i in 0..n {
            let u = ord[i];
            dp[u] = cost[u];
            for e in adj[u].clone() {
                let v = e.0;
                let new_cost = dp[v] - e.1 + cost[u];
                if dp[u] < new_cost {
                    dp[u] = new_cost;
                    pi[u] = v;
                }
            }
        }
        let mut path: Vec<usize> = vec![];
        let mut i = 0;
        while pi[i] != n {
            path.push(i);
            i = pi[i];
        }
        path.push(i);
        writeln!(sout, "{} {}", dp[0], path.len()).ok();
        for v in path {
            write!(sout, "{} ", v+1).ok();
        }
        writeln!(sout, "").ok();
    }
}
