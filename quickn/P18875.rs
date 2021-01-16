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

struct UnionFind {
    n: usize,
    rank: Vec<usize>,
    pi: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut pi: Vec<usize> = vec![0;n];
        for i in 0..n {
            pi[i] = i;
        }
        Self {
            n,
            rank: vec![0;n],
            pi,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.pi[x] == x {
            x
        } else {
            let r = self.find(self.pi[x]);
            self.pi[x] = r;
            r
        }
    }

    fn union(&mut self, x: usize, y: usize) {
        let (x_r, y_r) = (self.find(x), self.find(y));
        if x_r != y_r {
            if self.rank[x_r] > self.rank[y_r] {
                self.pi[y_r] = x_r;
            } else {
                self.pi[x_r] = y_r;
                if self.rank[x_r] == self.rank[y_r] {
                    self.rank[y_r] += 1;
                }
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
    let (n, m): (usize, usize) = (scan.token(), scan.token());
    let mut adj: Vec<Vec<usize>> = vec![vec![];n];
    for i in 0..m {
        let (a, b): (usize, usize) = (scan.token(), scan.token());
        adj[a-1].push(b-1);
    }
    let mut uf = UnionFind::new(n);
    for i in 0..n {
        if adj[i].len() > 0 {
            let v = adj[i][0];
            for j in 1..adj[i].len() {
                uf.union(v, adj[i][j]);
            }
        }
    }
    let mut res: Vec<usize> = vec![n;n];
    let mut assign: Vec<usize> = vec![n;n];
    let mut color = 0;
    for i in 0..n {
        let i_r = uf.find(i);
        if assign[i_r] != n {
            res[i] = assign[i_r];
        } else {
            color += 1;
            assign[i_r] = color;
            res[i] = color;
        }
        writeln!(sout, "{}", res[i]).ok();
    }
}
