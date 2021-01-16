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

const TIME_MAX: usize = 1000;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let n: usize = scan.token();
    let m: usize = scan.token();
    let c: i64 = scan.token();
    let mut cost: Vec<i64> = vec![0;n];
    for i in 0..n {
        cost[i] = scan.token();
    }
    let mut adj_inv: Vec<Vec<usize>> = vec![vec![];n];
    for i in 0..m {
        let (a, b): (usize, usize) = (scan.token(), scan.token());
        adj_inv[b-1].push(a-1);
    }
    use std::cmp::max;
    let mut dp: Vec<Vec<i64>> = vec![vec![-1;n];TIME_MAX+1];
    dp[0][0] = 0;
    let mut res = 0;
    for t in 1..=TIME_MAX {
        for i in 0..n {
            for u in adj_inv[i].clone() {
                if dp[t-1][u] != -1 {
                    dp[t][i] = max(dp[t][i], dp[t-1][u] + cost[i]);
                }
            }
        }
        res = max(res, dp[t][0] - c*(t as i64)*(t as i64));
    }
    writeln!(sout, "{}", res).ok();
}
