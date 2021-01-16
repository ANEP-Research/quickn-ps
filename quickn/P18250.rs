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

#[derive(Clone)]
struct Graph {
    adj: Vec<Vec<usize>>,
    visited: Vec<bool>,
}

impl Graph {
    fn new(n: usize, adj: Vec<Vec<usize>>) -> Self {
        Self {
            adj,
            visited: vec![false;n],
        }
    }
    
    fn dfs(&mut self, u: usize) -> usize {
        if !self.visited[u] {
            self.visited[u] = true;
            let mut res = 0;
            for v in self.adj[u].clone() {
                if !self.visited[v] {
                    res += self.dfs(v);
                }
            }
            if self.adj[u].len() % 2 == 1 {
                res += 1;
            }
            res
        } else {
            0
        }
    }
}

use std::collections::HashSet;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let (n, m): (usize, usize) = (scan.token(), scan.token());
    let mut adj: Vec<Vec<usize>> = vec![vec![];n];
    let mut set: HashSet<usize> = HashSet::new();
    for i in 0..m {
        let (u, v): (usize, usize) = (scan.token(), scan.token());
        adj[u-1].push(v-1);
        adj[v-1].push(u-1);
        set.insert(u-1);
        set.insert(v-1);
    }
    let mut g = Graph::new(n, adj);
    let mut res = 0;
    for i in set {
        if !g.visited[i] {
        let cnt = g.dfs(i);
        if cnt == 0 {
            res += 1;
        } else {
            res += cnt / 2;
        }
        }
    }
    writeln!(sout, "{}", res).ok();
}
