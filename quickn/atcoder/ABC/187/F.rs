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

#[derive(Clone, Debug)]
struct UnionFind {
    n: usize,
    c: usize,
    pi: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut pi: Vec<usize> = vec![0;n];
        for i in 0..n {
            pi[i] = i;
        }
        Self {
            n,
            c: n,
            pi,
            rank: vec![0;n],
        }
    }
    
    fn find(&mut self, u: usize) -> usize {
        if self.pi[u] == u {
            u 
        } else {
            let r = self.find(self.pi[u]);
            self.pi[u] = r;
            r
        }
    }

    fn union(&mut self, u: usize, v: usize) {
        let (u_r, v_r) = (self.find(u), self.find(v));
        if u_r != v_r {
            if self.rank[u_r] > self.rank[v_r] {
                self.pi[v_r] = u_r;
            } else {
                self.pi[u_r] = v_r;
                if self.rank[u_r] == self.rank[v_r] {
                    self.rank[v_r] += 1;
                }
            }
            self.c -= 1;
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
        adj[b-1].push(a-1);
    }
    let mut res = n;
    for i in 0..(1 << n) {
        let mut uf = UnionFind::new(n);
        for j in 0..n {
            if (i & (1 << j)) != 0 {
                for v in adj[j].clone() {
                    uf.union(j, v);
                }
            }
        }
        res = min(res, uf.c);
    }
    writeln!(sout, "{}", res).ok();
}
