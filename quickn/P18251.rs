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
struct Tree {
    n: usize,
    el: Vec<(i64, usize, usize)>,
    data: Vec<i64>,
    t: usize,
}

impl Tree {
    fn new(n: usize, depth: usize, data: Vec<i64>) -> Self {
        Self {
            n,
            el: vec![],
            data,
            t: 0,
        }
    }

    fn dfs(&mut self, node: usize, d: usize) {
        if node <= self.n {
            self.dfs(node * 2, d + 1);
            self.el.push((self.data[node-1], self.t, d));
            self.t += 1;
            self.dfs((node * 2) + 1, d + 1);
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
    let mut depth = 0;
    let mut n_t = n;
    while n_t > 0 {
        depth += 1;
        n_t /= 2;
    }
    let mut arr: Vec<i64> = vec![0; n];
    for i in 0..n {
        arr[i] = scan.token();
    }
    let mut g = Tree::new(n, depth, arr);
    g.dfs(1, 0);
    let mut el2: Vec<Vec<i64>> = vec![vec![0; n]; depth];
    for (d, x, y) in g.el.clone() {
        el2[depth - y - 1][x] = d;
    }
    let mut el2_prefix: Vec<Vec<i64>> = vec![vec![0; n]; depth];
    for i in 0..depth {
        for j in 0..n {
            if j == 0 {
                el2_prefix[i][j] = el2[i][j];
            } else {
                el2_prefix[i][j] = el2_prefix[i][j - 1];
                el2_prefix[i][j] += el2[i][j];
            }
        }
    }
    let mut res = 0;
    let depth2 = depth;
    for case in 0..depth2 {
        let mut el2_prefix2: Vec<Vec<i64>> = vec![vec![0; n]; depth];
        for i in 0..depth {
            for j in 0..n {
                if i == 0 {
                    el2_prefix2[i][j] = el2_prefix[i + case][j];
                } else {
                    el2_prefix2[i][j] = el2_prefix2[i - 1][j];
                    el2_prefix2[i][j] += el2_prefix[i + case][j];
                }
            }
        }
        use std::cmp::{max, min};
        for i in 0..depth {
            let mut min_val = 0;
            for j in 0..n {
                res = max(res, el2_prefix2[i][j] - min_val);
                min_val = min(min_val, el2_prefix2[i][j]);
            }
        }
        depth -= 1;
    }
    writeln!(sout, "{}", res).ok();
}

