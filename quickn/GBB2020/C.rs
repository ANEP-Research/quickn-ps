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

use std::cmp::{max, min};

fn s2vec(s: String) -> Vec<char> {
    s.chars().collect()
}

use std::collections::HashSet;

#[derive(Clone, Debug)]
struct DFS {
    n: usize,
    adj: Vec<Vec<usize>>,
    res2: Vec<usize>,
    set: BTreeSet<usize>,
}

impl DFS {
    fn new(n: usize, adj: Vec<Vec<usize>>) -> Self {
        Self {
            n,
            adj,
            res2: vec![],
            set: BTreeSet::new(),
        }
    }

    fn dfs2(&mut self, u: usize, prev: usize) {
        self.res2.push(u);
        for v in self.adj[u].clone() {
            if v != prev {
                if !self.set.contains(&v) {
                    self.dfs2(v, u);
                }
            }
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
    let (n, q): (usize, usize) = (scan.token(), scan.token());
    let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
    let mut deg: Vec<i32> = vec![0; n];
    for i in 0..n {
        let (u, v): (usize, usize) = (scan.token(), scan.token());
        adj[u - 1].push(v - 1);
        adj[v - 1].push(u - 1);
        deg[u - 1] += 1;
        deg[v - 1] += 1;
    }
    let mut g = DFS::new(n, adj.clone());
    let mut group: Vec<usize> = vec![n; n];
    let mut cnt = 0;
    let mut heap: BTreeSet<(i32, usize)> = BTreeSet::new();
    for i in 0..n {
        heap.insert((deg[i], i));
    }
    while let Some(&(d, u)) = heap.iter().nth(0) {
        if d > 1 {
            break;
        }
        heap.remove(&(d, u));
        for v in adj[u].clone() {
            heap.remove(&(deg[v], v));
            deg[v] -= 1;
            if deg[v] > 0 {
                heap.insert((deg[v], v));
            }
        }
    }
    let mut set: BTreeSet<usize> = BTreeSet::new();
    for (_, u) in heap.clone() {
        set.insert(u);
    }
    g.set = set.clone();
    for u in set {
        g.res2.clear();
        g.dfs2(u, n);
        for v in g.res2.clone() {
            group[v] = cnt;
        }
        cnt += 1;
    }
    for i in 0..q {
        let (mut u, mut v): (usize, usize) = (scan.token(), scan.token());
        u -= 1;
        v -= 1;
        if group[u] == group[v] {
            writeln!(sout, "1").ok();
        } else {
            writeln!(sout, "2").ok();
        }
    }
}
