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

use std::collections::BinaryHeap;

#[derive(Clone, Copy, Debug)]
struct Edge {
    end: usize,
    cost: i64,
}

impl Edge {
    fn new(end: usize, cost: i64) -> Self {
        Self {
            end,
            cost,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Vertex {
    v: usize,
    dist: i64,
}

impl Vertex {
    fn new(v: usize, dist: i64) -> Self {
        Self {
            v,
            dist,
        }
    }
}

use std::cmp::Ordering;

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.dist.cmp(&self.dist).then_with(|| other.v.cmp(&self.v)))
    }
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist).then_with(|| other.v.cmp(&self.v))
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let (n, m, k): (usize, usize, usize) = (scan.token(), scan.token(), scan.token());
    let mut adj: Vec<Vec<Edge>> = vec![vec![];n+1];
    for i in 0..m {
        let (a_i, b_i, t_i): (usize, usize, i64) = (scan.token(), scan.token(), scan.token());
        adj[a_i-1].push(Edge::new(b_i-1, t_i));
        adj[b_i-1].push(Edge::new(a_i-1, t_i));
    }
    let mut yum: Vec<i64> = vec![0;n];
    let mut t: Vec<usize> = vec![];
    for i in 0..k {
        let (x, y_i): (usize, i64) = (scan.token(), scan.token());
        yum[x-1] = y_i;
        t.push(x-1);
    }
    let mut pq: BinaryHeap<Vertex> = BinaryHeap::new();
    let mut dist1: Vec<i64> = vec![std::i64::MAX;n];
    dist1[n-1] = 0;
    pq.push(Vertex::new(n-1, dist1[n-1]));
    while let Some(v) = pq.pop() {
        let u = v.v;
        for i in 0..adj[u].len() {
            let e = adj[u][i].clone();
            let mut new_cost = dist1[u] + e.cost;
            if new_cost < dist1[e.end] {
                dist1[e.end] = new_cost;
                pq.push(Vertex::new(e.end, new_cost));
            }
        }
    }
    let mut dist2: Vec<i64> = vec![std::i64::MAX;n+1];
    dist2[n] = 0;
    for i in t {
        adj[n].push(Edge::new(i, dist1[i] - yum[i]));
    }
    pq.push(Vertex::new(n, dist2[n]));
    while let Some(v) = pq.pop() {
        let u = v.v;
        for i in 0..adj[u].len() {
            let e = adj[u][i].clone();
            let mut new_cost = dist2[u] + e.cost;
            if new_cost < dist2[e.end] {
                dist2[e.end] = new_cost;
                pq.push(Vertex::new(e.end, new_cost));
            }
        }
    }
    for i in 0..(n-1) {
        if dist1[i] >= dist2[i] {
            writeln!(sout, "1").ok();
        } else {
            writeln!(sout, "0").ok();
        }
    }
}
