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

use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Circle {
    x: i32,
    r: i32,
}

impl Circle {
    fn new(x: i32, r: i32) -> Self {
        Self {
            x,
            r,
        }
    }
}

#[derive(Clone)]
struct UnionFind {
    n: usize,
    pi: Vec<usize>,
    rank: Vec<usize>,
    c: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut pi: Vec<usize> = vec![0;n];
        let rank: Vec<usize> = vec![0;n];
        for i in 0..n {
            pi[i] = i;
        }
        Self {
            n,
            pi,
            rank,
            c: n,
        }
    }

    fn root(&mut self, u: usize) -> usize {
        if self.pi[u] != u {
            let r = self.root(self.pi[u]);
            self.pi[u] = r;
            r
        } else {
            u
        }
    }

    fn union(&mut self, u: usize, v: usize) {
        let (u_r, v_r) = (self.root(u), self.root(v));
        if u_r != v_r {
            self.c -= 1;
            if self.rank[u_r] > self.rank[v_r] {
                self.pi[v_r] = u_r;
            } else if self.rank[u_r] < self.rank[v_r] {
                self.pi[u_r] = v_r;
            } else {
                self.pi[v_r] = u_r;
                self.rank[u_r] += 1;
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
    let mut arr: Vec<Circle> = vec![];
    for i in 0..n {
        let (x_i, r_i): (i32, i32) = (scan.token(), scan.token());
        arr.push(Circle::new(x_i, r_i));
    }
    let mut edges = 0;
    let mut set: HashSet<i32> = HashSet::new();
    let mut hash: HashMap<i32, usize> = HashMap::new();
    let mut uf = UnionFind::new(n);
    for i in 0..n {
        let (l, r) = (arr[i].x - arr[i].r, arr[i].x + arr[i].r);
        set.insert(l);
        set.insert(r);
        if let Some(&k) = hash.get(&l) {
            uf.union(i, k);
        } else {
            hash.insert(l, i);
        }
        if let Some(&k) = hash.get(&r) {
            uf.union(i, k);
        } else {
            hash.insert(r, i);
        }
        edges += 2;
    }
    writeln!(sout, "{}", 1 + uf.c + edges - set.len()).ok();
}
