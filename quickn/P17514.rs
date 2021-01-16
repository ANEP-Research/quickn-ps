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
    n: usize,
    adj: Vec<(usize, usize)>,
    seq: Vec<usize>,
    visited: Vec<bool>,
}

impl Graph {
    fn new(n: usize, adj: Vec<(usize, usize)>) -> Self {
        Self {
            n,
            adj,
            seq: vec![],
            visited: vec![false;n],
        }
    }

    fn dfs(&mut self, v: usize, goal: usize) -> bool {
        self.visited[v] = true;
        if v == goal {
            true
        } else {
            if self.adj[v].0 == std::usize::MAX || self.visited[self.adj[v].0] {
                false
            } else {
                self.seq.push(self.adj[v].1);
                if self.seq.len() > MAX {
                    false
                } else {
                    self.dfs(self.adj[v].0, goal)
                }
            }
        }
    }
}

const MAX: usize = 1_000_000;

#[derive(Clone, Debug)]
struct Graph2 {
    n: usize,
    adj: Vec<Vec<(usize, usize)>>,
    visited: Vec<bool>,
    reached: Vec<(usize, usize, usize)>,
}

impl Graph2 {
    fn new(n: usize, adj: Vec<Vec<(usize, usize)>>) -> Self {
        Self {
            n,
            adj,
            visited: vec![false;n],
            reached: vec![],
        }
    }

    fn dfs(&mut self, v: usize) {
        self.visited[v] = true;
        for (u, c) in self.adj[v].clone() {
            self.reached.push((u, v, c));
            if !self.visited[u] {
                self.dfs(u);
            }
        }
    }

    fn find(&self, u: usize) -> bool {
        self.visited[u]
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let (n, m, mut s, mut t): (usize, usize, usize, usize) = (scan.token(), scan.token(), scan.token(), scan.token());
    s -= 1;
    t -= 1;
    let mut adj1: Vec<Vec<(usize, usize)>> = vec![vec![];n];
    let mut adj2: Vec<(usize, usize)> = vec![(std::usize::MAX, std::usize::MAX);n];
    let mut adjt: Vec<Vec<(usize, usize)>> = vec![vec![];n];
    for i in 0..m {
        let (u_i, v_i, c_i): (usize, usize, usize) = (scan.token(), scan.token(), scan.token());
        adj1[u_i-1].push((v_i-1, c_i));
        adjt[v_i-1].push((u_i-1, c_i));
    }
    let mut g2 = Graph2::new(n, adj1);
    let mut g3 = Graph2::new(n, adjt);
    g2.dfs(s);
    g3.dfs(t);
    if !g2.find(t) {
        writeln!(sout, "IMPOSSIBLE").ok();
    } else {
        for (u, v, c) in g3.reached {
            if adj2[u].1 > c {
                adj2[u].1 = c;
                adj2[u].0 = v;
            }
        }
        let mut g1 = Graph::new(n, adj2);
        if g1.dfs(s, t) {
            for a in g1.seq {
                write!(sout, "{} ", a).ok();
            }
        } else {
            writeln!(sout, "TOO LONG").ok();
        }
    }
}
