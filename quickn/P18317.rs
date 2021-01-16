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

use std::collections::HashMap;

const MAX: usize = 2_000_000;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let (n, q): (usize, usize) = (scan.token(), scan.token());
    let mut arr: Vec<i32> = vec![0;n+1];
    for i in 1..=n {
        arr[i] = scan.token();
    }
    let mut queries: Vec<(usize, usize)> = vec![];
    for i in 0..q {
        let (a, b): (usize, usize) = (scan.token(), scan.token());
        queries.push((a, b));
    }
    let mut cnt: Vec<Vec<i32>> = vec![vec![0;n+1];n+1];
    let mut hash: Vec<i32> = vec![0;MAX*2 + 1];
    for i in 1..=(n-2) {
        if i != 1 {
            for j in i..=n {
                hash[(arr[j] + (MAX as i32)) as usize] -= 1;
            }
        }
        hash[(arr[i+1] + (MAX as i32)) as usize] += 1;
        for j in (i+2)..=n {
            cnt[i][j] = hash[(-(arr[i] + arr[j]) + (MAX as i32)) as usize];
            hash[(arr[j] + (MAX as i32)) as usize] += 1;
        }
    }
    let mut dp: Vec<Vec<i64>> = vec![vec![0;n+1];n+1];
    for l in 3..=n {
        for i in 1..=(n-l+1) {
            let j = i + l - 1;
            dp[i][j] = dp[i+1][j] + dp[i][j-1] - dp[i+1][j-1] + (cnt[i][j] as i64);
        }
    }
    for (a, b) in queries {
        writeln!(sout, "{}", dp[a][b]).ok();
    }
}
