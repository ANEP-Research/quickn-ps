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

fn s2vec(s: String) -> Vec<char> {
    s.chars().collect()
}

#[derive(Clone)]
struct Tree {
    n: usize,
    adj: Vec<Vec<(usize, usize)>>,
    
}

impl Tree {
    fn new(n: usize, adj: Vec<Vec<(usize, usize)>>) -> Self {
        Self {
            n,
            adj,
            first: vec![n;n],
            post: vec![0;n-1],
            pre: vec![0;n-1],
            time: vec![],
        }
    }

    fn dfs(&mut self, u: usize, prev: usize) {
        for (v, i) in self.adj[u].clone() {
            if prev != v {
                self.first[i] = u;
                self.time.push(i);
                self.pre[i] = self.time.len();
                self.dfs(v, u, i);
                self.time.push(i);
                self.post[i] = self.time.len();
            }
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let n: usize = scan.token();
    let mut edges: Vec<(usize, usize)> = vec![];
    let mut adj: Vec<Vec<usize>> = vec![vec![];n];
    for i in 0..(n-1) {
        let (u, v): (usize, usize) = (scan.token(), scan.token());
        edges.push((u-1, v-1));
        adj[u-1].push((v-1, i));
        adj[v-1].push((u-1, i));
    }
    let mut t = Tree::new(n, adj.clone());
    t.dfs(0, 0);
    let q: usize = scan.token();
    let mut cost: Vec<i64> = vec![0;t.time.len()];
    let mut c: Vec<i64> = vec![0;n-1];
    for i in 0..q {
        let (ty, e, x): (usize, usize, i64) = (scan.token(), scan.token(), scan.token());
        let (a, b) = edges[e-1];
        if ty == 1 {
            if t.first[e-1] == a {
                cost[t.pre[e-1]-1] 
            } else {

            }
        } else {
            
        }
    }
    for i in 0..n {
        writeln!(sout, "{}", c[i]/2).ok();
    }
}

