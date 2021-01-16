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

const MAX: usize = 100_000;

#[derive(Clone)]
struct Tree {
    n: usize,
    adj: Vec<Vec<usize>>,
    arr: Vec<i64>,
    divisor: Vec<i64>,
    cnt: Vec<i64>,
    res: i64,
}

impl Tree {
    fn new(n: usize, adj: Vec<Vec<usize>>, arr: Vec<i64>) -> Self {
        Self {
            n,
            adj,
            arr,
            divisor: vec![0;MAX+1],
            cnt: vec![0;MAX+1],
            res: 0,
        }
    }
    
    fn dfs(&mut self, u: usize) {
        {
            self.res += self.cnt[self.arr[u] as usize];
            self.cnt[self.arr[u] as usize] += 1;
            self.res += self.divisor[self.arr[u] as usize];
            let mut k = 1;
            while k*k <= self.arr[u] {
                if self.arr[u] % k == 0 {
                    let x = k;
                    let y = self.arr[u] / k;
                    if x == y {
                        if x < self.arr[u] {
                            self.divisor[x as usize] += 1;
                            self.res += self.cnt[x as usize];
                        }
                    } else {
                        if x < self.arr[u] {
                            self.divisor[x as usize] += 1;
                            self.res += self.cnt[x as usize];
                        }
                        if y < self.arr[u] {
                            self.divisor[y as usize] += 1;
                            self.res += self.cnt[y as usize];
                        }
                    }
                }
                k += 1;
            }
        }
        for v in self.adj[u].clone() {
            self.dfs(v);
        }
        {
            self.cnt[self.arr[u] as usize] -= 1;
            let mut k = 1;
            while k*k <= self.arr[u] {
                if self.arr[u] % k == 0 {
                    let x = k;
                    let y = self.arr[u] / k;
                    if x == y {
                        if x < self.arr[u] {
                            self.divisor[x as usize] -= 1;
                        }
                    } else {
                        if x < self.arr[u] {
                            self.divisor[x as usize] -= 1;
                        }
                        if y < self.arr[u] {
                            self.divisor[y as usize] -= 1;
                        }
                    }
                }
                k += 1;
            }
        }
    }
}

use std::collections::HashSet;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let n: usize = scan.token();
    let mut arr: Vec<i64> = vec![0;n];
    for i in 0..n {
        arr[i] = scan.token();
    }
    let mut adj: Vec<Vec<usize>> = vec![vec![];n];
    for i in 1..n {
        let x: usize = scan.token();
        adj[x-1].push(i);
    }
    let mut t = Tree::new(n, adj, arr);
    t.dfs(0);
    writeln!(sout, "{}", t.res).ok();
}
