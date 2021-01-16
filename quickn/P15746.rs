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
struct Tree {
    adj: Vec<Vec<usize>>,
    cnt: Vec<i64>,
    res: i64,
    dp2: Vec<i64>,
    cost: Vec<i64>,
    leaf: Vec<bool>,
    leafs: i64,
}

impl Tree {
    fn new(n: usize, adj: Vec<Vec<usize>>, cost: Vec<i64>) -> Self {
        Self {
            adj,
            cnt: vec![0;n],
            res: std::i64::MAX,
            dp2: vec![0;n],
            cost,
            leaf: vec![false;n],
            leafs: 0,
        }
    }

    fn dfs(&mut self, u: usize) {
        if self.adj[u].len() == 0 {
            // length is 0
            self.leafs += 1;
            self.cnt[u] = 0;
            self.dp2[u] = self.cost[u];
            self.leaf[u] = true;
        } else {
            for v in self.adj[u].clone() {
                self.dfs(v);
            }
            for v in self.adj[u].clone() {
                if self.leaf[v] {
                    self.cnt[u] += 1;
                } else {
                    self.cnt[u] += self.cnt[v];
                }
                self.dp2[u] += self.dp2[v] + self.cnt[v]*(self.cost[v]+1);
            }
        }
    }

    fn dfs2(&mut self, u: usize, c1: i64, c2: i64) {
        use std::cmp::min;
        self.res = min(self.res, c1 + c2 + self.dp2[u]);
        for v in self.adj[u].clone() {
            if !self.leaf[v] {
                self.dfs2(v, c1 + (self.leafs - self.cnt[v])*3, c2 + self.dp2[u] - self.dp2[v] - self.cnt[v]*(self.cost[v]+1));
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
    let mut adj: Vec<Vec<usize>> = vec![vec![];n];
    let mut cost: Vec<i64> = vec![0;n];
    for i in 0..n {
        let (s, m): (String, usize) = (scan.token(), scan.token());
        cost[i] = s.len() as i64;
        for j in 0..m {
            let m_j: usize = scan.token();
            adj[i].push(m_j-1);
        }
    }
    let mut tree = Tree::new(n, adj, cost);
    tree.dfs(0);
    tree.dfs2(0, 0, 0);
    writeln!(sout, "{}", tree.res).ok();
}
