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
    size: Vec<usize>,
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

    fn get(&mut self, u: usize) -> usize {
        if self.pi[u] == u {
            u
        } else {
            let r = self.get(self.pi[u]);
            self.pi[u] = r;
            r
        }
    }

    fn union(&mut self, u: usize, v: usize) {
        let (u_r, v_r) = (self.get(u), self.get(v));
        if u_r != v_r {
            if self.rank[u_r] > self.rank[v_r] {
                self.pi[v_r] = u_r;
                self.size[u_r] += self.size[v_r];
            } else {
                self.pi[u_r] = v_r;
                if self.rank[u_r] == self.rank[v_r] {
                    self.rank[v_r] += 1;
                }
                self.size[v_r] += self.size[u_r];
            }
        }
    }

    fn size(&mut self, u: usize) -> usize {
        let r = self.get(u);
        self.size[r]-1
    }
}

#[derive(Clone, Copy, Debug)]
struct Edge {
    u: usize,
    v: usize,
    w: i32,
}

impl Edge {
    fn new(u: usize, v: usize, w: i32) -> Self {
        Self {
            u,
            v,
            w,
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let (n, q): (usize, usize) = (scan.token(), scan.token());
    let mut edges: Vec<Edge> = vec![];
    for i in 0..(n-1) {
        let (p_i, q_i, r_i): (usize, usize, i32) = (scan.token(), scan.token(), scan.token());
        edges.push(Edge::new(p_i-1, q_i-1, r_i));
    }
    edges.sort_by(|&e1, &e2| e2.w.cmp(&e1.w));
    let mut idx = 0;
    let mut queries: Vec<(usize, i32, usize)> = vec![];
    let mut uf = UnionFind::new(n);
    for i in 0..q {
        let (k_i, v_i): (i32, usize) = (scan.token(), scan.token());
        queries.push((i, k_i, v_i-1));
    }
    queries.sort_by(|&(_, k1, _), &(_, k2, _)| k2.cmp(&k1));
    let mut ans: Vec<usize> = vec![0;q];
    for (i, k_i, v_i) in queries {
        while idx < edges.len() && k_i <= edges[idx].w {
            uf.union(edges[idx].u, edges[idx].v);
            idx += 1;
        }
        ans[i] = uf.size(v_i);
    }
    for i in 0..q {
        writeln!(sout, "{}", ans[i]).ok();
    }
}
