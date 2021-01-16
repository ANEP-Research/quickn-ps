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
struct UnionFind {
    n: usize,
    pi: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<i64>
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut pi: Vec<usize> = vec![0;n];
        for i in 0..n {
            pi[i] = i;
        }
        Self {
            n,
            pi,
            rank: vec![0;n],
            size: vec![1;n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.pi[x] == x {
            x
        } else {
            let root = self.find(self.pi[x]);
            self.pi[x] = root;
            root
        }
    }

    fn union(&mut self, x: usize, y: usize) {
        let (x_t, y_t) = (self.find(x), self.find(y));
        if x_t != y_t {
            if self.rank[x_t] > self.rank[y_t] {
                self.pi[y_t] = x_t;
                self.size[x_t] += self.size[y_t];
            } else {
                self.pi[x_t] = y_t;
                self.size[y_t] += self.size[x_t];
                if self.rank[x_t] == self.rank[y_t] {
                    self.rank[y_t] += 1;
                }
            }
        }
    }

    fn size(&mut self, x: usize) -> i64 {
        let r = self.find(x);
        self.size[r]
    }
}

const MOD: i64 = 1_000_000_000;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let (n, m): (usize, usize) = (scan.token(), scan.token());
    let mut edges: Vec<(usize, usize, i64)> = vec![];
    let mut uf = UnionFind::new(n);
    let mut res = 0;
    let mut sum = 0;
    for i in 0..m {
        let (x, y, c): (usize, usize, i64) = (scan.token(), scan.token(), scan.token());
        edges.push((x-1, y-1, c));
        sum += c;
    }
    edges.sort_by(|&(_, _, c1), &(_, _, c2)| c2.cmp(&c1));
    for i in 0..m {
        if uf.find(edges[i].0) != uf.find(edges[i].1) {
        //dbg!(uf.size(edges[i].0), edges[i].2);
            res += sum*(uf.size(edges[i].0)*uf.size(edges[i].1));
            uf.union(edges[i].0, edges[i].1);
        }
        res %= MOD;
        sum -= edges[i].2;
    }
    writeln!(sout, "{}", res).ok();
}
