/*
    date   : 2020 / 5 / 5
    author : quickn (quickn.ga)
    email  : quickwshell@gmail.com
*/

use std::io::{self, BufWriter, Write};

mod scanner {
    use std::{io, str};
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

struct Tree {
    n: usize,
    adj: Vec<Vec<usize>>,
    cost: Vec<usize>,
}

impl Tree {
    fn new(n: usize, adj: Vec<Vec<usize>>, cost: Vec<usize>) -> Self {
        Self {
            n,
            adj,
            cost,
        }
    }

    fn dfs(&mut self, u: usize) -> usize {
        let mut max_t = 0;
        for v in self.adj[u].clone() {
            if max_t < self.cost[v] {
                max_t = self.cost[v];
            }
        }
        let mut cnt = 0;
        for v in self.adj[u].clone() {
            if max_t > self.cost[v] {
                cnt += 1;
            }
        }
        if cnt != 0 {
        let t = self.cost[u] / cnt;
        self.cost[u] -= t*cnt;
        for v in self.adj[u].clone() {
            if max_t > self.cost[v] {
                self.cost[v] += t;
            }
        }
        }
        dbg!(self.cost[u]);
        let mut res = 0;
        use std::cmp::max;
        for v in self.adj[u].clone() {
            res = max(self.dfs(v), res);
        }
        res += self.cost[u];
        res
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
    for i in 1..n {
        let u: usize = scan.token();
        adj[u-1].push(i);
    }
    let mut cost: Vec<usize> = vec![0;n];
    for i in 0..n {
        cost[i] = scan.token();
    }
    let mut t = Tree::new(n, adj, cost);
    let r = t.dfs(0);
    writeln!(sout, "{}", r).ok();
}
