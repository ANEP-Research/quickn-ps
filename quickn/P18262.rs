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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Vertex {
    u: usize,
    dist: i32,
}

use std::cmp::Ordering;

impl Vertex {
    fn new(u: usize, dist: i32) -> Self {
        Self {
            u,
            dist,
        }
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.dist.cmp(&self.dist).then_with(|| other.u.cmp(&self.u)))
    }
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist).then_with(|| other.u.cmp(&self.u))
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let (n, m): (usize, usize) = (scan.token(), scan.token());
    let mut edges: Vec<(usize, usize, i32, i32)> = vec![];
    for i in 0..m {
        let (u, v, c, f): (usize, usize, i32, i32) = (scan.token(), scan.token(), scan.token(), scan.token());
        edges.push((u-1, v-1, c, f));
    }
    edges.sort_by(|&(_, _, _, f2), &(_, _, _, f1)| f1.cmp(&f2));
    let mut res: i32 = 0;
    for i in 0..m {
        let mut adj: Vec<Vec<(usize, i32, i32)>> = vec![vec![];n];
        for (u, v, c, f) in edges.clone() {
            adj[u].push((v, c, f));
            adj[v].push((u, c, f));
        }
        let mut dist: Vec<i32> = vec![std::i32::MAX;n];
        let mut flow: Vec<i32> = vec![std::i32::MAX;n];
        dist[0] = 0;
        use std::collections::BinaryHeap;
        let mut heap: BinaryHeap<Vertex> = BinaryHeap::new();
        heap.push(Vertex::new(0, 0));
        use std::cmp::{min, max};
        while let Some(_u) = heap.pop() {
            let u = _u.u;
            for e in adj[u].clone() {
                let v = e.0;
                let new_dist = dist[u] + e.1;
                if dist[v] > new_dist {
                    dist[v] = new_dist;
                    use std::cmp::min;
                    flow[v] = min(flow[u], e.2);
                    heap.push(Vertex::new(v, dist[v]));
                }
            }
        }
        if flow[n-1] != std::i32::MAX {
            res = max(res, (1_000_000*flow[n-1])/dist[n-1]);
        }
        edges.pop();
    }
    writeln!(sout, "{}", res).ok();
}
