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

#[derive(Clone, Debug)]
struct DP {
    n: usize,
    h: i64,
    dp: Vec<i64>,
    visited: Vec<bool>,
    k: Vec<i64>,
    w: Vec<i64>,
    f: Vec<i64>,
    res: i64,
}

impl DP {
    fn new(n: usize, h: i64, k: Vec<i64>, w: Vec<i64>, f: Vec<i64>) -> Self {
        Self {
            n,
            h,
            dp: vec![0;(1<<n)],
            visited: vec![false;(1<<n)],
            k,
            w,
            f,
            res: -1,
        }
    }

    fn dp(&mut self, mask: usize) -> i64 {
        if !self.visited[mask] {
            self.visited[mask] = true;
            let mut res = -1;
            let mut sum = 0;
            for i in 0..self.n {
                if ((1 << i) & mask) != 0 {
                    sum += self.k[i];
                    if (1 << i) == mask {
                        res = self.f[i];
                    } else {
                        let prev = self.dp(mask ^ (1 << i));
                        if prev >= self.w[i] {
                            res = max(res, min(prev - self.w[i], self.f[i]));
                        }
                    }
                }
            }
            if self.h <= sum {
                self.res = max(res, self.res);
            }
            self.dp[mask] = res;
            res
        } else {
            self.dp[mask]
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let (n, h): (usize, i64) = (scan.token(), scan.token());
    let mut w: Vec<i64> = vec![0;n];
    let mut f: Vec<i64> = vec![0;n];
    let mut k: Vec<i64> = vec![0;n];
    for i in 0..n {
        let (ki, wi, fi): (i64, i64, i64) = (scan.token(), scan.token(), scan.token());
        k[i] = ki;
        w[i] = wi;
        f[i] = fi;
    }
    let mut dp = DP::new(n, h, k, w, f);
    dp.dp((1<<n)-1);
    if dp.res == -1 {
        writeln!(sout, "Mark is too tall").ok();
    } else {
        writeln!(sout, "{}", dp.res).ok();
    }
}

