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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let (r, g, b2): (usize, usize, usize) = (scan.token(), scan.token(), scan.token());
    let mut a: Vec<i64> = vec![0;r];
    let mut b: Vec<i64> = vec![0;g];
    let mut c: Vec<i64> = vec![0;b2];
    for i in 0..r {
        a[i] = scan.token();
    }
    for i in 0..g {
        b[i] = scan.token();
    }
    for i in 0..b2 {
        c[i] = scan.token();
    }
    a.sort();
    b.sort();
    c.sort();
    let mut dp: Vec<Vec<Vec<i64>>> = vec![vec![vec![0;b2+1];g+1];r+1];
    use std::cmp::max;
    let mut res = 0;
    for i in 0..=r {
        for j in 0..=g {
            for k in 0..=b2 {
                if i > 0 && j > 0 {
                    dp[i][j][k] = max(dp[i][j][k], dp[i-1][j-1][k] + a[i-1]*b[j-1]);
                }
                if i > 0 && k > 0 {
                    dp[i][j][k] = max(dp[i][j][k], dp[i-1][j][k-1] + a[i-1]*c[k-1]);
                }
                if j > 0 && k > 0 {
                    dp[i][j][k] = max(dp[i][j][k], dp[i][j-1][k-1] + b[j-1]*c[k-1]);
                }
                res = max(res, dp[i][j][k]);
            }
        }
    }
    writeln!(sout, "{}", res).ok();
}
