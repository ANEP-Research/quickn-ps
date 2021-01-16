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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Edge {
    cost: u8,
    u: usize,
    v: usize,
}

impl Edge {
    fn new(u: usize, v: usize, red: bool) -> Self {
        Self {
            cost: if red { 0 } else { 1 },
            u,
            v,
        }
    }
}

#[derive(Clone)]
struct UnionFind {
    n: usize,
    pi: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            n, 
            pi: (0..n).collect(),
            rank: vec![0;n],
        }
    }

    fn find(&mut self, u: usize) -> usize {
        if u == self.pi[u] {
            u
        } else {
            let r = self.find(self.pi[u]);
            self.pi[u] = r;
            r
        }
    }

    fn union(&mut self, u: usize, v: usize) -> bool {
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
            true
        } else {
            false
        }
    }
}

struct Tree {
    n: usize,
    adj: Vec<Vec<Edge>>,
    path: Vec<Edge>
}

impl Tree {
    fn new(n: usize, edges: BTreeSet<Edge>) -> Self {
        let mut adj: Vec<Vec<Edge>> = vec![vec![];n];
        for e in edges {
            adj[e.u].push(e);
            adj[e.v].push(e);
        }
        Self {
            n,
            adj,
            path: vec![],
        }
    }

    fn dfs(&mut self, u: usize, prev: usize, target: usize) -> bool {
        if target == u {
            true
        } else {
            let mut res = false;
            for e in self.adj[u].clone() {
                if e.u != prev && e.v != prev {
                    let r = if e.u == u { self.dfs(e.v, u, target) } else { self.dfs(e.u, u, target) };
                    if r {
                        self.path.push(e);
                    }
                    res |= r;
                }
            }
            res
        }
    }
}

use std::collections::BTreeSet;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let (n, m, k): (usize, usize, usize) = (scan.token(), scan.token(), scan.token());
    let mut adj: Vec<Vec<Edge>> = vec![vec![];n];
    let mut edges: Vec<Edge> = Vec::new();
    let mut deg: Vec<usize> = vec![0;n];
    for _i in 0..m {
        let (t, u, v): (char, usize, usize) = (scan.token(), scan.token(), scan.token());
        let e = Edge::new(u-1, v-1, if t == 'R' { true } else { false });
        adj[u-1].push(e);
        adj[v-1].push(e);
        edges.push(e);
        if t == 'B' {
            deg[u-1] += 1;
            deg[v-1] += 1;
        }
    }
    edges.sort();
    let mut mst: BTreeSet<Edge> = BTreeSet::new();
    let mut mst2: Vec<Edge> = Vec::new();
    let mut uf: UnionFind = UnionFind::new(n);
    let (mut cost, mut cost2): (usize, usize) = (0, 0);
    for e in edges.clone() {
        if uf.union(e.u, e.v) {
            mst.insert(e);
            if e.cost == 1 {
                cost += 1;
            }
        }
    }
    let mut uf2 = UnionFind::new(n);
    edges.reverse();
    for e in edges.clone() {
        if uf2.union(e.u, e.v) {
            mst2.push(e);
            if e.cost == 1 {
                cost2 += 1;
            }
        }
    }
    if cost <= k && k <= cost2 {
        let mut mst3: BTreeSet<Edge> = mst.clone();
        let mut cost3 = cost;
        for i in 0..(n-1) {
            if cost3 == k {
                break;
            }
            let mut t = Tree::new(n, mst3.clone());
            let mut e = mst2[i];
            t.dfs(e.u, n, e.v);
            let p = t.path.clone();
            for e2 in p {
                if mst.contains(&e2) {
                    mst.remove(&e2);
                    mst3.remove(&e2);
                    if e2.cost == 1 {
                        cost3 -= 1;
                    }
                    break;
                }
            }
            if e.cost == 1 {
                cost3 += 1;
            }
            mst3.insert(e);
        }
        for e in mst3 {
            writeln!(sout, "{} {}", e.u+1, e.v+1).ok();
        }
    } else {
        writeln!(sout, "0").ok();
    }
}
