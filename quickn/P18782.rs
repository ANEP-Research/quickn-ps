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

use std::collections::{BTreeMap, HashSet};

struct Tree {
    n: usize,
    adj: Vec<Vec<usize>>,
    subtree: Vec<usize>,
    num: Vec<Vec<usize>>,
}

impl Tree {
    fn new(n: usize, adj: Vec<Vec<usize>>) -> Self {
        Self {
            n,
            adj,
            subtree: vec![0;n],
            num: vec![vec![];n],
        }
    }

    fn dp(&mut self, u: usize, prev: usize) {
        self.subtree[u] = 1;
        for v in self.adj[u].clone() {
            if v != prev {
            self.dp(v, u);
            self.subtree[u] += self.subtree[v];
            self.num[u].push(self.subtree[v]);
            }
        }
        self.num[u].push(self.n - self.subtree[u]);
    }

    fn check(&self, k: usize) -> bool {
        let mut cnt: Vec<usize> = vec![0;k];
        let mut res = true;
        for i in 0..self.n {
            let mut hash: HashSet<usize> = HashSet::new();
            for &num in &self.num[i] {
                cnt[num%k] = 0;
                cnt[(k - (num%k))%k] = 0;
                hash.insert(num);
            }
            for &num in &self.num[i] {
                if num%k != 0 {
                    if cnt[k - (num%k)] > 0 {
                        cnt[k - (num%k)] -= 1;
                    } else {    
                        cnt[num%k] += 1;
                    }
                }
            }
            for num in hash {
                if cnt[num%k] > 0 {
                    res &= false;
                    break;
                }
            }
        }
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
    for i in 0..(n-1) {
        let (u, v): (usize, usize) = (scan.token(), scan.token());
        adj[u-1].push(v-1);
        adj[v-1].push(u-1);
    }
    let mut res: Vec<bool> = vec![false;n-1];
    let mut t = Tree::new(n, adj);
    t.dp(0, n);
    for i in 1..=(n-1) {
        if (n-1) % i == 0 {
            if t.check(i) {
                write!(sout, "1").ok();
            } else {
                write!(sout, "0").ok();
            }
        } else {
            write!(sout, "0").ok();
        }
    }
}