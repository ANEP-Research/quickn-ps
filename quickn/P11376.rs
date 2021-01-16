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

use std::cmp::{min, max};

#[derive(Clone, Debug)]
struct BipartiteGraph {
    n: usize,
    m: usize,
    mat: Vec<usize>,
    adj: Vec<Vec<usize>>,
    visited: Vec<bool>,
}

impl BipartiteGraph {
    fn new(n: usize, m: usize, adj: Vec<Vec<usize>>) -> Self {
        Self {
            n,
            m,
            mat: vec![n;m],
            adj,
            visited: vec![false;n],
        }
    }

    fn dfs(&mut self, u: usize) -> bool {
        if !self.visited[u] {
            self.visited[u] = true;
            let mut cnt = false;
            for i in 0..self.adj[u].len() {
                let v = self.adj[u][i];
                if self.mat[v] != u && (self.mat[v] == self.n || self.dfs(self.mat[v])) {
                    self.mat[v] = u;
                    cnt = true;
                    break;
                }
            }
            cnt
        } else {
            false
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let (n, m): (usize, usize) = (scan.token(), scan.token());
    let mut adj: Vec<Vec<usize>> = vec![vec![];n];
    for i in 0..n {
        let k: usize = scan.token();
        for j in 0..k {
            let x: usize = scan.token();
            adj[i].push(x-1);
        }
    }
    let mut g = BipartiteGraph::new(n, m, adj);
    let mut res = 0;
    for i in 0..n {
        g.visited = vec![false;n];
        if g.dfs(i) {
            res += 1;
        }
        g.visited = vec![false;n];
        if g.dfs(i) {
            res += 1;
        }
    }
    writeln!(sout, "{}", res).ok();
}
